use std::{
    collections::HashMap,
    sync::mpsc::{self, Sender},
    thread,
};

use stub::packet::{ClientPacket, ServerPacket};

use crate::{
    error::AppError,
    room::{start_room, RoomMessage},
};

#[derive(Debug)]
pub enum ServerMessage {
    Connect(Sender<ClientMessage>),
    Disconnect(usize),
    Packet(usize, ServerPacket),
    RemoveRoom(usize),
}

#[derive(Debug)]
pub enum ClientMessage {
    Id(usize),
    Packet(ClientPacket),
}

pub fn start_server() -> Result<Sender<ServerMessage>, AppError> {
    let (tx, rx) = mpsc::channel::<ServerMessage>();
    let mut server = Server::new(tx.clone());
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => server.handle(msg).unwrap(),
            Err(_) => (),
        }
    });
    Ok(tx)
}

pub struct Server {
    srv: Sender<ServerMessage>,

    user_id_counter: usize,
    room_id_counter: usize,

    // userid -> roomid
    users: HashMap<usize, usize>,
    user_sender: HashMap<usize, Sender<ClientMessage>>,

    rooms: HashMap<usize, Sender<RoomMessage>>,
}

impl Server {
    fn new(srv: Sender<ServerMessage>) -> Self {
        Server {
            srv,
            user_id_counter: 0,
            room_id_counter: 0,
            user_sender: HashMap::new(),
            users: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: ServerMessage) -> Result<(), AppError> {
        match msg {
            ServerMessage::Connect(client) => {
                let id = self.user_id_counter;
                self.user_id_counter += 1;
                let _ = client.send(ClientMessage::Id(id));
                self.user_sender.insert(id, client);
            }
            ServerMessage::Disconnect(id) => {
                if let Some(room_id) = self.users.get(&id) {
                    let _ = self
                        .rooms
                        .get(&room_id)
                        .unwrap()
                        .send(RoomMessage::Leave(id));
                }
                self.user_sender.remove(&id);
                println!("client disconnected {id}");
            }
            ServerMessage::Packet(id, packet) => self.handle_user_packet(id, packet)?,
            ServerMessage::RemoveRoom(room_id) => {
                self.rooms.remove(&room_id);
            }
        }

        Ok(())
    }

    fn handle_user_packet(&mut self, user_id: usize, packet: ServerPacket) -> Result<(), AppError> {
        match packet {
            ServerPacket::CreateLobby => {
                if self.users.get(&user_id).is_some() {
                    return Ok(());
                }

                let room_id = self.room_id_counter;
                self.room_id_counter += 1;
                let room = start_room(room_id, self.srv.clone())?;
                self.user_join_room(user_id, room_id, &room);
                self.rooms.insert(room_id, room.clone());
            }
            ServerPacket::LeaveLobby => {
                let room_id = match self.users.get(&user_id) {
                    Some(room_id) => room_id,
                    None => return Ok(()),
                };

                let room = self.rooms.get(&room_id).unwrap();
                let _ = room.send(RoomMessage::Leave(user_id));
                self.users.remove(&user_id);
            }
            ServerPacket::JoinLobby(room_id) => {
                if self.users.get(&user_id).is_some() {
                    return Ok(());
                }

                let room = match self.rooms.get(&room_id) {
                    Some(room) => room,
                    None => return Ok(()),
                }
                .clone();
                self.user_join_room(user_id, room_id, &room);
            }
            packet => {
                if let Some(room_id) = self.users.get(&user_id) {
                    let _ = self
                        .rooms
                        .get(room_id)
                        .unwrap()
                        .send(RoomMessage::Packet(user_id, packet));
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn user_join_room(&mut self, user_id: usize, room_id: usize, room: &Sender<RoomMessage>) {
        let user = self.user_sender.get(&user_id).unwrap();
        let _ = room.send(RoomMessage::Join(user_id, user.clone()));
        self.users.insert(user_id, room_id);

        let _ = user.send(ClientMessage::Packet(ClientPacket::JoinLobby));
    }
}
