use std::{net::TcpStream, sync::mpsc::Receiver, io::Write, ops::DerefMut};
use stub::packet::{ClientPacket, ServerPacket};

use crate::{AppContext, AppState};

#[derive(Debug)]
pub enum ClientMessage {
    Packet(ClientPacket),    
    Disconnected,

    CreateLobby,
}

pub fn start_client(rx: Receiver<ClientMessage>, mut context: AppContext, mut connection: TcpStream) {
    loop {
        match rx.recv() {
            Ok(msg) => process_message(msg, &mut context, &mut connection),
            Err(_) => break,
        }
    }
}

fn process_message(msg: ClientMessage, context: &mut AppContext, connection: &mut TcpStream) {
    match msg {
        ClientMessage::CreateLobby => {
            let value = ServerPacket::CreateLobby;
            let encoded = bincode::serialize(&value).unwrap();
            let _ = connection.write(&encoded); 
        },
        ClientMessage::Packet(ClientPacket::JoinLobby) => {
            println!("joined lobby");
            context.0.lock().unwrap().deref_mut().state = AppState::Lobby;
        },
        _ => (),
    }
}
