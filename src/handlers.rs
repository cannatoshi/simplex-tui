//! Event handling
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use std::time::Duration;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEventKind, MouseButton};

use crate::app::App;
use crate::types::{Mode, Panel};
use crate::ui::modals::{BUTTON_REFRESH, BUTTON_CREATE, BUTTON_CLOSE};

pub fn handle_events(app: &mut App) -> Result<()> {
    if !event::poll(Duration::from_millis(50))? { return Ok(()); }
    
    match event::read()? {
        Event::Key(key) => handle_key(app, key.code, key.modifiers),
        Event::Mouse(mouse) => handle_mouse(app, mouse),
        _ => {}
    }
    Ok(())
}

fn handle_mouse(app: &mut App, mouse: event::MouseEvent) {
    let x = mouse.column;
    let y = mouse.row;
    
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            match app.mode {
                Mode::AddContact => {
                    unsafe {
                        if let Some(btn) = BUTTON_REFRESH {
                            if x >= btn.x && x < btn.x + btn.width && y >= btn.y && y < btn.y + btn.height {
                                app.request_address();
                                return;
                            }
                        }
                        if let Some(btn) = BUTTON_CREATE {
                            if x >= btn.x && x < btn.x + btn.width && y >= btn.y && y < btn.y + btn.height {
                                app.create_address();
                                return;
                            }
                        }
                        if let Some(btn) = BUTTON_CLOSE {
                            if x >= btn.x && x < btn.x + btn.width && y >= btn.y && y < btn.y + btn.height {
                                app.mode = Mode::Normal;
                                app.connect_input.clear();
                                return;
                            }
                        }
                    }
                }
                Mode::Normal | Mode::Input => {
                    let term_height = crossterm::terminal::size().map(|(_, h)| h).unwrap_or(24);
                    
                    if x < 32 {
                        if y >= term_height - 3 && y < term_height - 1 {
                            if x < 10 {
                                app.mode = Mode::AddContact;
                                app.request_address();
                                return;
                            } else if x < 20 {
                                app.refresh_chat();
                                app.send_cmd("/contacts");
                                return;
                            } else if x < 32 {
                                app.show_help = !app.show_help;
                                return;
                            }
                        }
                        
                        if y > 2 && y < term_height - 3 {
                            let idx = ((y - 3) / 2) as usize;
                            if idx < app.contacts.len() {
                                app.contact_state.select(Some(idx));
                                app.select_contact();
                            }
                        }
                    }
                    
                    if let Some(input_y) = app.last_input_y {
                        if y >= input_y && y < input_y + 3 && x > 32 {
                            app.panel = Panel::Input;
                            app.mode = Mode::Input;
                        }
                    }
                }
                _ => {}
            }
        }
        MouseEventKind::ScrollUp => app.scroll_up(),
        MouseEventKind::ScrollDown => app.scroll_down(),
        _ => {}
    }
}

fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    match code {
        KeyCode::F(12) => { app.mode = Mode::Panic; return; }
        _ => {}
    }
    
    if mods.contains(KeyModifiers::CONTROL) {
        match code {
            KeyCode::Char('c') | KeyCode::Char('q') => app.running = false,
            KeyCode::Char('l') => app.send_cmd("/contacts"),
            _ => {}
        }
        return;
    }
    
    match app.mode {
        Mode::Panic => handle_panic(app, code),
        Mode::AddContact => handle_add_contact(app, code),
        Mode::Normal => handle_normal(app, code),
        Mode::Input => handle_input(app, code),
    }
}

fn handle_panic(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Char('1') => app.status = "Screen locked".into(),
        KeyCode::Char('2') => app.running = false,
        KeyCode::Char('3') => app.status = "NUKE initiated...".into(),
        KeyCode::Char('4') => app.status = "SSH stealth mode".into(),
        _ => {}
    }
}

fn handle_add_contact(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.connect_input.clear();
        }
        KeyCode::Char('g') => app.request_address(),
        KeyCode::Char('c') => app.create_address(),
        KeyCode::Enter => {
            if !app.connect_input.is_empty() {
                app.connect_to_invite();
            }
        }
        KeyCode::Backspace => { app.connect_input.pop(); }
        KeyCode::Char(c) => {
            if !c.is_control() {
                app.connect_input.push(c);
            }
        }
        _ => {}
    }
}

fn handle_normal(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') => app.running = false,
        KeyCode::Char('?') => app.show_help = !app.show_help,
        KeyCode::Esc => {
            if app.show_help { app.show_help = false; }
        }
        KeyCode::Tab => app.cycle_panel(),
        KeyCode::Char('i') => {
            app.mode = Mode::AddContact;
            // Always request address when opening modal
            app.request_address();
        }
        KeyCode::Char('r') => {
            app.refresh_chat();
            app.send_cmd("/contacts");
        }
        KeyCode::Char('j') | KeyCode::Down => {
            match app.panel {
                Panel::Contacts => app.next_contact(),
                Panel::Chat => app.scroll_down(),
                _ => {}
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            match app.panel {
                Panel::Contacts => app.prev_contact(),
                Panel::Chat => app.scroll_up(),
                _ => {}
            }
        }
        KeyCode::Enter => {
            if app.panel == Panel::Contacts {
                app.select_contact();
            } else {
                app.panel = Panel::Input;
                app.mode = Mode::Input;
            }
        }
        _ => {}
    }
}

fn handle_input(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.panel = Panel::Chat;
        }
        KeyCode::Tab => app.cycle_panel(),
        KeyCode::Enter => app.send_message(),
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Delete => app.input_delete(),
        KeyCode::Left => app.cursor_left(),
        KeyCode::Right => app.cursor_right(),
        KeyCode::Home => app.cursor_home(),
        KeyCode::End => app.cursor_end(),
        KeyCode::Char(c) => app.input_char(c),
        _ => {}
    }
}
