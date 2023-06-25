use std::{sync::mpsc::Sender, ffi::CString};

use raylib::{prelude::{RaylibDrawHandle, RaylibDraw, Color}, ffi::Rectangle, rgui::RaylibDrawGui};

use crate::{AppData, AppState, client::ClientMessage};

pub fn draw_context(mut d: RaylibDrawHandle<'_>, state: &AppData, tx: &Sender<ClientMessage>, width: i32, height: i32) {
    d.clear_background(Color::WHITE);

    match state.state {
        AppState::Idle(false) => {
            if d.gui_button(Rectangle{x: 20.0, y: 20.0, width: 100.0, height: 40.0}, Some(&CString::new("Create Lobby").unwrap())) {
                let _ = tx.send(ClientMessage::CreateLobby);
            }
            if d.gui_button(Rectangle{x: 20.0, y: 80.0, width: 100.0, height: 40.0}, Some(&CString::new("Join Lobby").unwrap())) {
                let _ = tx.send(ClientMessage::SetIdle(true));
            }
            d.draw_text("Chesslike, play now!", width / 2, height / 2, 20, Color::BLACK);
        },
        AppState::Idle(true) => {
            if d.gui_button(Rectangle{x: 20.0, y: 20.0, width: 100.0, height: 40.0}, Some(&CString::new("Leave").unwrap())) {
                let _ = tx.send(ClientMessage::SetIdle(false));
            }
            d.draw_text("Chesslike, play now!", width / 2, height / 2, 20, Color::BLACK);
        },
        AppState::Lobby => {
            d.draw_text("Now in lobby", 12, 12, 20, Color::BLACK);
        },
        AppState::Ingame => d.draw_text("Now ingame", 12, 12, 20, Color::BLACK),
    }
}
