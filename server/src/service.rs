use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{Sender, channel, self},
    thread::{self, JoinHandle},
};

use stub::packet::ServerPacket;
use crate::{error::AppError, server::ServerMessage};

pub fn start_service(
    port: usize,
    srv: Sender<ServerMessage>,
) -> Result<JoinHandle<Result<(), AppError>>, AppError> {
    let handle = thread::spawn(move || -> Result<(), AppError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            match stream {
                Ok(client) => handle_client(client, srv.clone()),
                Err(_) => (),
            };
        }

        Ok(())
    });

    Ok(handle)
}

fn handle_client(mut client: TcpStream, srv: Sender<ServerMessage>) {
    println!("Client connected");

    let (tx, rx) = mpsc::channel::<usize>();
    let _ = srv.send(ServerMessage::Connect(tx));

    thread::spawn(move || -> Result<(), AppError> {
        let mut buf = [0; 128];

        loop {
            match client.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        println!("Client disconnected");
                        break;
                    }

                    let value: ServerPacket = bincode::deserialize(&buf[0..len]).unwrap();
                    // println!("received: {:?}", value);
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
        client.write(&[0; 10]);
        Ok(())
    });
}
