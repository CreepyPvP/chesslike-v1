use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPacket {
    CreateLobby,
    JoinLobby(usize),
    LeaveLobby,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientPacket {
    JoinLobby,
}
