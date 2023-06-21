mod rooms;

use async_trait::async_trait;
use crate::event::client_message::ClientChannel;
use crate::event::server_message::ServerMessage;

#[async_trait]
pub trait ClientMessageHandler {
    async fn handle(&self, message: ServerMessage, outbound: ClientChannel);
}
