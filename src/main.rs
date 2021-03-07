use futures::executor::block_on;
use std::thread;
use tokio::sync::mpsc;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};

mod image;
mod server;
mod state;
mod texture;
mod vertex;
mod slm {
    tonic::include_proto!("slm");
}

use crate::image::{ColourType, ImageData};
use crate::slm::slm_server::SlmServer;
use state::State;
use tonic::transport::Server;

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

fn main() {
    env_logger::init();
    let event_loop: EventLoop<server::Message> = EventLoop::with_user_event();
    let event_loop_proxy: EventLoopProxy<server::Message> = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(event_loop.available_monitors().last())))
        .with_always_on_top(true)
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(State::new(&window));
    let (tx, mut rx) = mpsc::channel(10);
    let mut server = server::SlmService {
        screens: window.available_monitors().collect(),
        tx,
    };

    let svc = SlmServer::new(server);
    // spawn the service thread
    thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
            .block_on(
                Server::builder()
                    .add_service(svc)
                    .serve("[::1]:10000".parse().unwrap()),
            );
    });
    // spawn the message thread
    thread::spawn(move || loop {
        match rx.blocking_recv() {
            Some(message) => {
                event_loop_proxy.send_event(message);
            }
            None => break,
        }
    });

    event_loop.run(move |event, _, control_flow| match event {
        Event::UserEvent(server::Message::SetImage(im)) => {
            state.set_image(im);
        }
        Event::UserEvent(server::Message::SetScreen(monitor_handle)) => window.set_fullscreen(
            Some(winit::window::Fullscreen::Borderless(Some(monitor_handle))),
        ),
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Recreate the swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        } => {
                            state.set_image(ImageData {
                                colour_type: ColourType::RGB,
                                bytes: vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255],
                                size: (2, 2),
                                offset: None,
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
