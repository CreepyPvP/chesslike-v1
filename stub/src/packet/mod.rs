use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPacket {
    CreateLobby,
    JoinLobby(String),
    LeaveLobby,
}
