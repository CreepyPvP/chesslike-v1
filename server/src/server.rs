use std::{
    sync::mpsc::{self, Sender},
    thread, collections::HashMap,
};

use stub::packet::ServerPacket;

use crate::{error::AppError, room::{RoomMessage, start_room}};

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
                    let _ = self.rooms.get(&room_id).unwrap().send(RoomMessage::Leave(id));
                }
                self.user_sender.remove(&id);
                println!("client disconnected {id}");
            },
            ServerMessage::Packet(id, packet) => self.handle_user_packet(id, packet)?,
            ServerMessage::RemoveRoom(room_id) => {
                self.rooms.remove(&room_id);
            },
        }

        Ok(())
    }

    fn handle_user_packet(&mut self, user_id: usize, packet: ServerPacket) -> Result<(), AppError> {
        match packet {
            ServerPacket::CreateLobby => {
                if self.users.get(&user_id).is_some() {
                    return Ok(());
                }

                let lobby_id = self.room_id_counter;
                self.room_id_counter += 1;
                let lobby = start_room(lobby_id, self.srv.clone())?;
                let _ = lobby.send(RoomMessage::Join(user_id, self.user_sender.get(&user_id).unwrap().clone()));
                self.users.insert(user_id, lobby_id);
                self.rooms.insert(lobby_id, lobby);
            },
            ServerPacket::LeaveLobby => {
                let room_id = match self.users.get(&user_id) {
                    Some(room_id) => room_id,
                    None => return Ok(()),
                };

                let room = self.rooms.get(&room_id).unwrap();
                let _ = room.send(RoomMessage::Leave(user_id));
                self.users.remove(&user_id);
            },
            ServerPacket::JoinLobby(room_id) => {
                if self.users.get(&user_id).is_some() {
                    return Ok(());
                }

                let room = match self.rooms.get(&room_id) {
                    Some(room) => room,
                    None => return Ok(()),
                };
                let _ = room.send(RoomMessage::Join(user_id, self.user_sender.get(&user_id).unwrap().clone()));
                self.users.insert(user_id, room_id);
            },
            packet => {
                if let Some(room_id) = self.users.get(&user_id) {
                    let _ = self.rooms.get(room_id).unwrap().send(RoomMessage::Packet(user_id, packet));
                    return Ok(());
                }
            },
        }

        Ok(())
    }

}
