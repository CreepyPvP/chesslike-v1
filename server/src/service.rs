use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{Sender, channel, self},
    thread::{self, JoinHandle},
};

use stub::packet::ServerPacket;
use crate::{error::AppError, server::{ServerMessage, ClientMessage}};

pub fn start_service(
    port: usize,
    srv: Sender<ServerMessage>,
) -> Result<JoinHandle<Result<(), AppError>>, AppError> {
    let handle = thread::spawn(move || -> Result<(), AppError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            match stream {
                Ok(client) => handle_client(client, srv.clone())?,
                Err(_) => (),
            };
        }

        Ok(())
    });

    Ok(handle)
}

fn handle_client(client: TcpStream, srv: Sender<ServerMessage>) -> Result<(), AppError> {
    let (tx, rx) = mpsc::channel::<ClientMessage>();
    let _ = srv.send(ServerMessage::Connect(tx));

    let id = match rx.recv() {
        Ok(ClientMessage::Id(id)) => id,
        _ => panic!("Server is a little stupid"),
    };
    println!("Client connected {id}");
    
    let mut reader = client.try_clone()?;

    thread::spawn(move || -> Result<(), AppError> {
        let mut buf = [0; 128];

        loop {
            match reader.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        let _ = srv.send(ServerMessage::Disconnect(id));
                        break;
                    }
                    if let Ok(msg) = bincode::deserialize::<ServerPacket>(&buf[0..len]) {
                        let _ = srv.send(ServerMessage::Packet(msg)); 
                    }
                }
                Err(_) => {
                    println!("Error blocking thread");
                    break;
                }
            }
        }

        Ok(())
    });

    thread::spawn(move || -> Result<(), AppError> {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    println!("got client msg: {:?}", msg)
                },
                Err(_) => break,
            }
        }
        // let _ = client.write(&[0; 10]);
        Ok(())
    });

    Ok(())
}
