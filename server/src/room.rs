use std::{
    collections::HashMap,
    sync::mpsc::{self, Sender},
    thread,
};

use stub::packet::ServerPacket;

use crate::{
    error::AppError,
    server::{ClientMessage, ServerMessage},
};

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
    srv: Sender<ServerMessage>,
    id: usize,
}

impl Room {
    fn new(srv: Sender<ServerMessage>, id: usize) -> Self {
        Room {
            state: RoomState::Lobby,
            users: HashMap::new(),
            srv,
            id,
        }
    }

    fn handle(&mut self, msg: RoomMessage) -> bool {
        match msg {
            RoomMessage::Join(user_id, client) => {
                self.users.insert(user_id, client);
                println!("user joined room. {} users in room", self.users.len());
            }
            RoomMessage::Leave(user_id) => {
                self.users.remove(&user_id);
                if self.users.len() == 0 {
                    println!("no users left; closing room");
                    let _ = self.srv.send(ServerMessage::RemoveRoom(self.id));
                    return true;
                }
            }
            RoomMessage::Packet(user_id, packet) => {
                println!("got room packet {user_id} {:?}", packet)
            }
        }

        false
    }
}

pub fn start_room(id: usize, srv: Sender<ServerMessage>) -> Result<Sender<RoomMessage>, AppError> {
    let mut lobby = Room::new(srv, id);
    let (tx, rx) = mpsc::channel::<RoomMessage>();

    let _ = thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => {
                if lobby.handle(msg) {
                    break;
                }
            }
            Err(_) => break,
        };
    });

    Ok(tx)
}
