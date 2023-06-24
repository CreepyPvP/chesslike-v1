use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use client::{start_client, ClientMessage};
use error::ClientError;
use network::connect;
use raylib::prelude::{Color, RaylibDraw};

mod client;
mod error;
mod network;

pub enum AppState {
    Idle,
    Lobby,
    Ingame,
}

pub struct AppData {
    pub state: AppState,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            state: AppState::Idle,
        }
    }
}

#[derive(Clone)]
pub struct AppContext(Arc<Mutex<AppData>>);

impl AppContext {
    fn new(data: AppData) -> Self {
        AppContext(Arc::new(Mutex::new(data)))
    }
}

fn main() -> Result<(), ClientError> {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let context = AppContext::new(AppData::default());

    let (tx, rx) = channel();
    let connection = connect("127.0.0.1:3000", tx.clone())?;

    let client_context = context.clone();
    thread::spawn(move || {
        start_client(rx, client_context, connection);
    });

    while !rl.window_should_close() {
        if rl.is_key_pressed(raylib::prelude::KeyboardKey::KEY_A) {
            let _ = tx.send(ClientMessage::CreateLobby);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        match context.0.lock().unwrap().state {
            AppState::Idle => d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK),
            AppState::Lobby => d.draw_text("Now in lobby", 12, 12, 20, Color::BLACK),
            AppState::Ingame => d.draw_text("Now ingame", 12, 12, 20, Color::BLACK),
        }
    }

    Ok(())
}
