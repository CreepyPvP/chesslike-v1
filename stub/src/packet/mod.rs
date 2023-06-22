use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPacket {
    Hello(String),
    World(usize),
}
