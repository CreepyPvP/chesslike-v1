use std::{
    sync::mpsc::{self, Sender},
    thread, collections::HashMap,
};

use stub::packet::ServerPacket;

use crate::{error::AppError, server::ClientMessage};


#[derive(Debug)]
pub enum RoomMessage {
    Join(usize, Sender<ClientMessage>),
    Leave(usize),
    Packet(usize, ServerPacket),
}

pub enum RoomState {
    Lobby,
    Ingame,
}

pub struct Room {
    state: RoomState,
    users: HashMap<usize, Sender<ClientMessage>>,
}

impl Room {
    fn new() -> Self {
        Room {
            state: RoomState::Lobby,
            users: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: RoomMessage) -> bool {
        match msg {
            RoomMessage::Join(user_id, client) => {
                self.users.insert(user_id, client);
                println!("user joined room. {} users in room", self.users.len());
            },
            RoomMessage::Leave(user_id) => {
                self.users.remove(&user_id);
                if self.users.len() == 0 {
                    println!("no users left; closing room");
                    return true;
                }
            },
            RoomMessage::Packet(user_id, packet) => println!("got room packet {user_id} {:?}", packet),
        }

        false
    }
}

pub fn start_room(_: usize) -> Result<Sender<RoomMessage>, AppError> {
    let mut lobby = Room::new();
    let (tx, rx) = mpsc::channel::<RoomMessage>();

    let _ = thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => {
                if lobby.handle(msg) {
                    break;
                }
            },
            Err(_) => break,
        };
    });

    Ok(tx)
}
