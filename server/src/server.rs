use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use crate::error::AppError;

pub struct Server {}

impl Server {
    fn new() -> Self {
        Server {}
    }

    fn handle(&mut self, msg: ServerMessage) {
        println!("got message {:?}", msg);
    }
}

#[derive(Debug)]
pub enum ServerMessage {
    Connect(Sender<usize>),
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
