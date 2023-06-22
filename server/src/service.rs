use std::{net::{TcpListener, TcpStream}, thread::{self, JoinHandle}, io::Read};

use stub::packet::ServerPacket;

use crate::error::AppError;



pub fn start_service(port: usize) -> Result<JoinHandle<Result<(), AppError>>, AppError> {
    let handle = thread::spawn(move || -> Result<(), AppError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            match stream {
                Ok(client) => handle_client(client),
                Err(_) => (),
            };
        }

        Ok(())
    });

    Ok(handle)
}

fn handle_client(mut client: TcpStream) {
    println!("Client connected");
    
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
                    println!("received: {:?}", value);
                },
                Err(_) => {
                    println!("Error blocking thread");
                    break
                },
            }
        }

        Ok(())
    });
    
}
