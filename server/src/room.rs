use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use stub::packet::ServerPacket;

use crate::error::AppError;


#[derive(Debug)]
pub enum RoomMessage {
    Join(usize),
    Leave(usize),
    Packet(usize, ServerPacket),
}

pub enum RoomState {
    Lobby,
    Ingame,
}

pub struct Room {
    state: RoomState,
}

impl Room {
    fn new() -> Self {
        Room {
            state: RoomState::Lobby,
        }
    }

    fn handle(&mut self, msg: RoomMessage) {
        match msg {
            RoomMessage::Join(user_id) => println!("user {user_id} joined room"),
            RoomMessage::Leave(user_id) => println!("user {user_id} left room"),
            RoomMessage::Packet(user_id, packet) => println!("got room packet {user_id} {:?}", packet),
        }
    }
}

pub fn start_room(id: usize) -> Result<Sender<RoomMessage>, AppError> {
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
