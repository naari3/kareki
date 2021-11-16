use std::io::Error;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::packet::client::{self, Pong, SlpResponse};
use crate::packet::server::{Ping, Request};
use crate::server::Worker;

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

#[derive(Debug)]
pub struct StatusResponse {
    pub description: Description,
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
}

impl Serialize for StatusResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StatusResponse", 3)?;
        s.serialize_field("description", &self.description)?;
        if self.favicon.is_some() {
            s.serialize_field("favicon", &self.favicon)?;
        }
        s.serialize_field("players", &self.players)?;
        s.serialize_field("version", &self.version)?;
        s.end()
    }
}

pub async fn handle_slp_status(worker: &mut Worker, _request: Request) -> Result<(), Error> {
    println!("get status request");

    let status_response = StatusResponse {
        description: Description {
            text: "Yo this was implemented by naari3 @naari_ @_naari_".to_string(),
        },
        players: Players {
            max: 12345,
            online: 126534640, // Japan population
            sample: Some(vec![]),
        },
        favicon: None,
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

pub async fn handle_slp_ping(worker: &mut Worker, ping: Ping) -> Result<(), Error> {
    println!("get ping");

    let pong = client::StatusPacket::Pong(Pong {
        payload: ping.payload,
    });
    worker.write_packet(pong).await?;

    println!("sent pong");
    Ok(())
}
