use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread, ffi::{CStr, CString},
};

use client::{start_client, ClientMessage};
use error::ClientError;
use network::connect;
use raylib::{prelude::{Color, RaylibDraw}, ffi::{GuiButton, Rectangle}, rgui::RaylibDrawGui};
use ui::draw_context;

mod client;
mod error;
mod network;
mod ui;

pub enum AppState {
    Idle(bool),
    Lobby,
    Ingame,
}

pub struct AppData {
    pub state: AppState,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            state: AppState::Idle(false),
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
    let width = 640;
    let height = 480;

    let (mut rl, thread) = raylib::init().size(width, height).title("Hello, World").build();

    let context = AppContext::new(AppData::default());

    let (tx, rx) = channel();
    let connection = connect("127.0.0.1:3000", tx.clone())?;

    let client_context = context.clone();
    thread::spawn(move || {
        start_client(rx, client_context, connection);
    });

    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        draw_context(d, &context.0.lock().unwrap(), &tx, width, height);
    }

    Ok(())
}
