use clap::{App, Arg};
use futures::executor::block_on;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use tokio::sync::mpsc;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    monitor::MonitorHandle,
    window::{Fullscreen, Window, WindowBuilder},
};

mod image;
mod server;
mod state;
mod texture;
mod vertex;
mod slm {
    tonic::include_proto!("slm");
}

use crate::slm::slm_server::SlmServer;
use state::State;
use tonic::transport::Server;

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

fn main() {
    let matches = App::new("SLM Server")
        .version("0.1.0")
        .author("Max Tyler <maxastyler@gmail.com>")
        .about("A server with associated fullscreen window for displaying patterns on an SLM")
        .arg(
            Arg::with_name("PORT")
                .help("The port for the server to run on")
                .required(true)
                .index(1)
                .validator(|v| {
                    if let Ok(_) = v.parse::<u32>() {
                        Ok(())
                    } else {
                        Err(format!("{} could not be parsed as a u32", v))
                    }
                }),
        )
        .arg(
            Arg::with_name("monitor")
                .short("m")
                .help("The name of the monitor to display the server on")
                .takes_value(true)
                .value_name("monitor")
                .default_value(""),
        )
        .get_matches();

    let icon = winit::window::Icon::from_rgba(
        include_bytes!("data/icon").iter().cloned().collect(),
        100,
        100,
    )
    .ok();

    let port: u16 = matches.value_of("PORT").unwrap().parse().unwrap();

    let event_loop: EventLoop<server::Message> = EventLoop::with_user_event();
    let event_loop_proxy: EventLoopProxy<server::Message> = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("SLM")
        .with_window_icon(icon)
        .build(&event_loop)
        .unwrap();

    if let Some(m) = event_loop
        .available_monitors()
        .find(|m| m.name() == matches.value_of("monitor").map(|x| x.to_owned()))
    {
        set_window_monitor(&window, port, m);
    } else {
        set_window_monitor(
            &window,
            port,
            event_loop.available_monitors().last().unwrap(),
        );
    }

    let mut state = block_on(State::new(&window));
    let (tx, mut rx) = mpsc::channel(4);
    let cloned_tx = tx.clone();
    let server = server::SlmService { tx };

    let svc = SlmServer::new(server);
    // spawn the service thread
    let server_thread = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
            .block_on(Server::builder().add_service(svc).serve(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port,
            )));
        block_on(cloned_tx.send(server::Message::Quit));
    });
    // spawn the message thread
    let message_thread = thread::spawn(move || loop {
        match rx.blocking_recv() {
            Some(message) => {
                if let Err(_) = event_loop_proxy.send_event(message) {
                    break;
                };
            }
            None => break,
        }
    });

    event_loop.run(move |event, _, control_flow| match event {
        Event::UserEvent(server::Message::SetImage(im)) => {
            state.set_image(im);
        }
        Event::UserEvent(server::Message::SetScreen(monitor)) => {
            if let Some(monitor_handle) = window.available_monitors().nth(monitor) {
                set_window_monitor(&window, port, monitor_handle)
            }
        }
        Event::UserEvent(server::Message::Quit) => *control_flow = ControlFlow::Exit,
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Recreate the swap_chain if lost
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(_) => {}
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
                    _ => {}
                }
            }
        }
        _ => {}
    });
}

/// set the window fullscreen on the given monitor
fn set_window_monitor(window: &Window, port: u16, monitor_handle: MonitorHandle) {
    if let Some(name) = monitor_handle.name() {
        window.set_title(&format!("SLM display: {}; port: {}", &name, port));
    }
    window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor_handle))));
}
