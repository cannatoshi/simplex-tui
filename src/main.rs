//! SimpleX TUI - Secure Terminal Messenger
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

mod app;
mod colors;
mod handlers;
mod types;
mod ui;
mod websocket;

use std::io;
use std::sync::mpsc;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{DisableMouseCapture, EnableMouseCapture},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use types::SimplexEvent;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let result = run_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    if let Err(e) = result { eprintln!("Error: {}", e); }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();
    
    let (event_tx, event_rx) = mpsc::channel();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    
    app.cmd_tx = Some(cmd_tx);
    websocket::spawn(event_tx, cmd_rx);
    
    while app.running {
        app.tick();
        
        while let Ok(event) = event_rx.try_recv() {
            handle_simplex_event(&mut app, event);
        }
        
        terminal.draw(|frame| ui::render(frame, &mut app))?;
        handlers::handle_events(&mut app)?;
    }
    
    Ok(())
}

fn handle_simplex_event(app: &mut App, event: SimplexEvent) {
    match event {
        SimplexEvent::Connected => {
            app.connected = true;
            app.status = "Ready".into();
        }
        
        SimplexEvent::Disconnected => {
            app.connected = false;
            app.status = "Reconnecting...".into();
        }
        
        SimplexEvent::Contacts(contacts) => {
            let mut new_contacts = contacts;
            for new_c in &mut new_contacts {
                if let Some(old_c) = app.contacts.iter().find(|c| c.name == new_c.name) {
                    new_c.unread = old_c.unread;
                }
            }
            app.contacts = new_contacts;
            app.status = "Ready".into();
        }
        
        SimplexEvent::Messages(messages) => {
            let count = messages.len();
            app.messages = messages;
            app.auto_scroll();
            
            if let Some(current) = &app.current_contact {
                if let Some(contact) = app.contacts.iter_mut().find(|c| &c.name == current) {
                    contact.unread = 0;
                }
                app.status = format!("{} messages", count);
            }
        }
        
        SimplexEvent::NewMessage { sender, text } => {
            let is_current_chat = app.current_contact.as_ref() == Some(&sender);
            
            if is_current_chat {
                use chrono::Local;
                app.messages.push(types::ChatMessage {
                    sender: sender.clone(),
                    content: text.clone(),
                    time: Local::now().format("%H:%M").to_string(),
                    mine: false,
                    status: types::MessageStatus::Delivered,
                });
                app.auto_scroll();
                app.status = "New message".into();
            } else {
                if let Some(contact) = app.contacts.iter_mut().find(|c| c.name == sender) {
                    contact.unread += 1;
                }
                app.status = format!("📩 {}", sender);
            }
        }
        
        SimplexEvent::MessageUpdate { status } => {
            for msg in app.messages.iter_mut().rev() {
                if msg.mine && (msg.status == types::MessageStatus::Sending || msg.status == types::MessageStatus::Sent) {
                    msg.status = status;
                    break;
                }
            }
            match status {
                types::MessageStatus::Sent => app.status = "Sent ✓".into(),
                types::MessageStatus::Delivered => app.status = "Delivered ✓✓".into(),
                _ => {}
            }
        }
        
        SimplexEvent::InviteLink(link) => {
            app.invite_link = Some(link);
            app.pending_new_address = false;
            app.status = "Address ready".into();
        }
        
        SimplexEvent::AddressDeleted => {
            app.finish_create_address();
        }
        
        SimplexEvent::AddressCreated => {
            app.send_cmd("/sa");
        }
        
        SimplexEvent::ContactRequest(name) => {
            app.status = format!("✓ Connected: {}", name);
            app.send_cmd("/contacts");
        }
        
        SimplexEvent::ContactDeleted(name) => {
            app.on_contact_deleted(&name);
            app.send_cmd("/contacts");
        }
        
        SimplexEvent::ChatCleared(name) => {
            app.on_chat_cleared(&name);
        }
        
        SimplexEvent::ContactInfo(info_data) => {
            app.contact_info_data = Some(info_data);
            app.mode = crate::types::Mode::ContactInfo;
        }
        
        SimplexEvent::Error(err) => {
            if err == "No address" && app.pending_new_address {
                app.finish_create_address();
            } else if err == "Address exists" {
                app.request_address();
            } else {
                app.status = err;
            }
        }
        
        SimplexEvent::Status(status) => {
            app.status = status;
        }
    }
}
