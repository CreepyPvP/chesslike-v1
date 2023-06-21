use async_trait::async_trait;
use futures::SinkExt;
use ntex::rt;
use crate::event::client_message::{ClientChannel, ClientEvent};
use crate::event::server_message::ServerMessage;
use crate::handling::ClientMessageHandler;

pub struct InGamePhase {}

#[async_trait]
impl ClientMessageHandler for InGamePhase {

    async fn handle(&self, message: ServerMessage, mut outbound: ClientChannel) {
        println!("Just a test message from inside a game");
        rt::spawn(async move {
            outbound.send(ClientEvent::Message("test")).await
        });
    }
}
