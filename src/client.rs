pub mod slm {
    tonic::include_proto!("slm");
}

use futures_util::stream;
use slm::slm_client::SlmClient;
use slm::{image_data::ImageOneof, image_description::ColourType, ImageData, ImageDescription};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SlmClient::connect("http://[::1]:10000").await?;

    let image_data = vec![
        ImageData {
            image_oneof: Some(ImageOneof::Description(ImageDescription {
                width: 2,
                height: 2,
                colour_type: ColourType::Rgb8 as i32,
            })),
        },
        ImageData {
            image_oneof: Some(ImageOneof::Data(vec![
                255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 255, 0, 255
            ])),
        },
    ];
    let response = client
        .set_image(Request::new(stream::iter(image_data)))
        .await?;

    println!("RESPONSE = {:?}", response);

    Ok(())
}
