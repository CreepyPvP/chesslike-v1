mod in_game;

use async_trait::async_trait;
use crate::event::client_message::{ClientChannel};
use crate::event::server_message::ServerMessage;
use crate::handling::rooms::in_game::InGamePhase;

use super::ClientMessageHandler;


enum GameState {
    Lobby,
    InGame(InGamePhase),
}

#[async_trait]
impl ClientMessageHandler for GameState {
    async fn handle(&self, message: ServerMessage, mut outbound: ClientChannel) {
        match self {
            Self::Lobby => println!("its a lobby"),
            Self::InGame(game) => game.handle(message, outbound).await,
        }
    }
}

pub struct Room {
    id: String,
    state: GameState,
}

#[async_trait]
impl ClientMessageHandler for Room {
    async fn handle(&self, message: ServerMessage, outbound: ClientChannel) {
        self.state.handle(message, outbound).await
    }
}
