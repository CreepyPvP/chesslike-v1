use std::{io::Read, net::TcpStream, sync::mpsc::Sender, thread};

use stub::packet::ClientPacket;

use crate::{error::ClientError, client::ClientMessage};

pub fn connect(url: &'static str, client: Sender<ClientMessage>) -> Result<TcpStream, ClientError> {
    let stream = TcpStream::connect(url)?;

    let mut reader = stream.try_clone()?;
    let mut buf = [0; 128];
    thread::spawn(move || loop {
        match reader.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    let _ = client.send(ClientMessage::Disconnected);
                    break;
                }
                if let Ok(packet) = bincode::deserialize::<ClientPacket>(&buf[0..len]) {
                    let _ = client.send(ClientMessage::Packet(packet));
                }
            }
            Err(_) => {
                println!("Error blocking thread");
                break;
            }
        }
    });

    Ok(stream)
}
