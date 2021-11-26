use std::fs::File;
use std::io::{Cursor, Result};

use image::{
    gif::GifDecoder, imageops::resize, png::PngEncoder, AnimationDecoder, EncodableLayout, Frame,
};
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::packet::client::{self, Pong, SlpResponse};
use crate::packet::server::{Ping, Request};
use crate::server::Worker;

pub struct GifRunner {
    frames: Vec<Frame>,
    index: usize,
}

impl GifRunner {
    fn new(gif_decoder: GifDecoder<File>) -> Self {
        Self {
            frames: gif_decoder.into_frames().collect_frames().unwrap(),
            index: 0,
        }
    }

    pub fn next(&mut self) -> Result<String> {
        self.index = (self.index + 1) % self.frames.len();
        let frame = self.frames.get(self.index).expect("should be exist");
        let buffer = frame.buffer();
        let resized = resize(buffer, 64, 64, image::imageops::FilterType::Gaussian);
        let mut png_cursor = Cursor::new(vec![]);

        let png_encoder = PngEncoder::new(&mut png_cursor);
        png_encoder
            .encode(
                resized.as_bytes(),
                resized.width(),
                resized.height(),
                image::ColorType::Rgba8,
            )
            .unwrap();

        let data = base64::encode(png_cursor.into_inner());
        let data = format!("data:image/png;base64,{}", data);

        Ok(data)
    }
}

static GIF: Lazy<Mutex<GifRunner>> = Lazy::new(|| {
    let input = File::open("nc3132.gif").unwrap();
    // Configure the decoder such that it will expand the image to RGBA.
    let gif_decoder = GifDecoder::new(input).unwrap();
    let gif = GifRunner::new(gif_decoder);
    Mutex::new(gif)
});

#[derive(Debug, Serialize)]
pub struct Description {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<Sample>>,
}

#[derive(Debug, Serialize)]
pub struct Sample {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
}

pub async fn handle_slp_status(worker: &mut Worker, _request: Request) -> Result<()> {
    println!("get status request");

    let status_response = StatusResponse {
        description: Description {
            text: "this made by naari3 @naari_".to_string(),
        },
        players: Players {
            max: 12345,
            online: 126534640, // Japan population
            sample: Some(vec![]),
        },
        favicon: GIF.lock().await.next().ok(),
        version: Version {
            name: "1.15.2".to_string(),
            protocol: 578,
        },
    };

    let json_response = serde_json::to_string(&status_response)?;

    println!("will send: {}", json_response);

    let slp_response = client::StatusPacket::SlpResponse(SlpResponse { json_response });
    worker.write_packet(slp_response).await?;

    println!("sent status");
    Ok(())
}

pub async fn handle_slp_ping(worker: &mut Worker, ping: Ping) -> Result<()> {
    println!("get ping");

    let pong = client::StatusPacket::Pong(Pong {
        payload: ping.payload,
    });
    worker.write_packet(pong).await?;

    println!("sent pong");
    Ok(())
}
