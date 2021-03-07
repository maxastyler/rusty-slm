use crate::image;
use crate::slm::slm_server::{Slm, SlmServer};
use crate::slm::{
    image_data::ImageOneof, image_description::ColourType, EmptyParams, ImageDescription, Position,
    Response, Screen,
};
use tokio::sync::mpsc;
use tonic::{Request, Status, Streaming};
use winit::monitor::MonitorHandle;


#[derive(Debug)]
pub struct SlmService {
    pub screens: Vec<MonitorHandle>,
    pub tx: mpsc::Sender<Message>,
}

#[derive(Clone)]
pub enum Message {
    SetImage(image::ImageData),
}

#[tonic::async_trait]
impl Slm for SlmService {
    async fn set_image(
        &self,
        request: Request<Streaming<crate::slm::ImageData>>,
    ) -> Result<tonic::Response<Response>, Status> {
        let mut image_bytes = vec![];
        let mut stream = request.into_inner();

        let mut image_data = image::ImageData {
            colour_type: image::ColourType::GreyScale,
            size: (0, 0),
            bytes: vec![],
            offset: None,
        };

        while let Some(data) = stream.message().await? {
            match data.image_oneof {
                Some(ImageOneof::Description(ImageDescription {
                    width,
                    height,
                    colour_type,
                })) => {
                    image_data.size.0 = width;
                    image_data.size.1 = height;
                    println!("{:?}", ColourType::Grey8 as i32);
                    match 3 {
                        x if x == (3.0 as u32) => 2,
                        _ => 3,
                    };
                    image_data.colour_type = match colour_type {
                        x if x == ColourType::Grey8 as i32 => image::ColourType::GreyScale,
                        _ => image::ColourType::RGB,
                    };
                }
                Some(ImageOneof::Data(mut data)) => image_bytes.append(&mut data),
                _ => {}
            }
        }
        image_data.bytes = image_bytes;
        if let Err(_) = self.tx.send(Message::SetImage(image_data)).await {
            return Err(tonic::Status::aborted("Couldn't send image data to main thread"))
        };
        Ok(tonic::Response::new(Response {
            completed: true,
            error: "".to_string(),
        }))
    }
    async fn set_screen(
        &self,
        request: Request<Screen>,
    ) -> Result<tonic::Response<Response>, Status> {
        unimplemented!();
    }
    async fn set_position(
        &self,
        request: Request<Position>,
    ) -> Result<tonic::Response<Response>, Status> {
        unimplemented!();
    }
}
