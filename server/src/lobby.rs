use std::sync::mpsc::{Sender, self};

use crate::error::AppError;

pub enum LobbyState {
    Waiting,
    Ingame,
}

pub struct Lobby {
    state: LobbyState,
}

impl Lobby {
    fn new() -> Self {
        Lobby {
            state: LobbyState::Waiting,
        }
    }
}

pub fn start_lobby() -> Result<Sender<()>, AppError> {
    let lobby = Lobby::new();
    let (tx, rx) = mpsc::channel

    Ok(())
}
