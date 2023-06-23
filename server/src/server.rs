use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use stub::packet::ServerPacket;

use crate::error::AppError;

pub struct Server {
    id_counter: usize,
}

impl Server {
    fn new() -> Self {
        Server {
            id_counter: 0,
        }
    }

    fn handle(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::Connect(client) => {
                let _ = client.send(ClientMessage::Id(self.id_counter));
                self.id_counter += 1;
            },
            ServerMessage::Disconnect(id) => println!("client disconnected {id}"),
            ServerMessage::Packet(packet) => println!("got packet {:?}", packet), 
        }
    }
}

#[derive(Debug)]
pub enum ServerMessage {
    Connect(Sender<ClientMessage>),
    Disconnect(usize),
    Packet(ServerPacket),
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
            Ok(msg) => server.handle(msg),
            Err(_) => (),
        }
    });
    Ok(tx)
}
