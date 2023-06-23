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
}

#[derive(Debug)]
pub enum ClientMessage {
    Id(usize),
}

pub fn start_server() -> Result<Sender<ServerMessage>, AppError> {
    let mut server = Server::new();
    let (tx, rx) = mpsc::channel::<ServerMessage>();
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => server.handle(msg).unwrap(),
            Err(_) => (),
        }
    });
    Ok(tx)
}

pub struct Server {
    user_id_counter: usize,
    room_id_counter: usize,
    // userid -> roomid
    users: HashMap<usize, usize>,
    rooms: HashMap<usize, Sender<RoomMessage>>
}

impl Server {
    fn new() -> Self {
        Server { 
            user_id_counter: 0,
            room_id_counter: 0,
            users: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn handle(&mut self, msg: ServerMessage) -> Result<(), AppError> {
        match msg {
            ServerMessage::Connect(client) => {
                let _ = client.send(ClientMessage::Id(self.user_id_counter));
                self.user_id_counter += 1;
            }
            ServerMessage::Disconnect(id) => {
                if let Some(room_id) = self.users.get(&id) {
                    let _ = self.rooms.get(&room_id).unwrap().send(RoomMessage::Leave(id));
                }
                println!("client disconnected {id}");
            },
            ServerMessage::Packet(id, packet) => self.handle_user_packet(id, packet)?,
        }

        Ok(())
    }

    fn handle_user_packet(&mut self, user_id: usize, packet: ServerPacket) -> Result<(), AppError> {
        println!("handling user packet");
        if let Some(room_id) = self.users.get(&user_id) {
            println!("sending room message");
            let _ = self.rooms.get(room_id).unwrap().send(RoomMessage::Packet(user_id, packet));
            return Ok(());
        }

        match packet {
            ServerPacket::CreateLobby => {
                if self.users.get(&user_id).is_some() {
                    return Ok(());
                }

                let lobby_id = self.room_id_counter;
                self.room_id_counter += 1;
                let lobby = start_room(lobby_id)?;
                let _ = lobby.send(RoomMessage::Join(user_id));
                self.users.insert(user_id, lobby_id);
                self.rooms.insert(lobby_id, lobby);
            }
        }

        Ok(())
    }

}
