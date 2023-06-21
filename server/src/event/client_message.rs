use futures::channel::mpsc::UnboundedSender;
use serde::{Serialize, Deserialize};

use super::RawPacket;

pub enum ClientEvent {
    Id(usize),
    Message(ClientMessage),
}

pub type ClientChannel = UnboundedSender<ClientEvent>;


// outgoing

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub token: String,
}

pub enum ClientMessage {
    Auth(AuthData),
}

impl ClientMessage {
    pub fn stringfy(self) -> Result<String, serde_json::Error> {
        let (channel, data) = match self {
            Self::Auth(data) => ("set_auth_token", serde_json::to_value(data)?),
        };
        let raw = RawPacket {
            channel: channel.to_string(), data
        };

        serde_json::to_string(&raw)
    }
}
