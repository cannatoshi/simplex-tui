//! UI module
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

mod layout;
mod contacts;
mod chat;
mod input;
mod status;
pub mod modals;

pub fn render(frame: &mut ratatui::Frame, app: &mut crate::app::App) {
    layout::render(frame, app);
}
