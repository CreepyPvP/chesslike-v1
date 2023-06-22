use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use crate::error::AppError;

pub enum RoomState {
    Waiting,
    Ingame,
}

pub struct Room {
    state: RoomState,
}

#[derive(Debug)]
pub enum RoomMessage {}

impl Room {
    fn new() -> Self {
        Room {
            state: RoomState::Waiting,
        }
    }

    fn handle(&mut self, msg: RoomMessage) {
        println!("Received message: {:?}", msg);
    }
}

pub fn start_lobby() -> Result<Sender<RoomMessage>, AppError> {
    let mut lobby = Room::new();
    let (tx, rx) = mpsc::channel::<RoomMessage>();

    let _ = thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => lobby.handle(msg),
            Err(err) => break,
        };
    });

    Ok(tx)
}
