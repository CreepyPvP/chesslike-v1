use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPacket {
    Hello(String),
    World(usize),
}
