//! Application state and logic
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use std::sync::mpsc;
use chrono::Local;
use ratatui::widgets::ListState;

use crate::types::{Contact, ChatMessage, Panel, Mode, MessageStatus};

pub struct App {
    pub running: bool,
    pub mode: Mode,
    pub panel: Panel,
    pub contacts: Vec<Contact>,
    pub contact_state: ListState,
    pub messages: Vec<ChatMessage>,
    pub scroll: usize,
    pub input: String,
    pub cursor: usize,
    pub show_help: bool,
    pub tick: u64,
    pub status: String,
    pub connected: bool,
    pub current_contact: Option<String>,
    pub cmd_tx: Option<mpsc::Sender<String>>,
    pub last_input_y: Option<u16>,
    pub invite_link: Option<String>,
    pub connect_input: String,
    pub pending_new_address: bool,
}

impl App {
    pub fn new() -> Self {
        let mut contact_state = ListState::default();
        contact_state.select(Some(0));
        
        Self {
            running: true,
            mode: Mode::Normal,
            panel: Panel::Contacts,
            contacts: vec![],
            contact_state,
            messages: vec![],
            scroll: 0,
            input: String::new(),
            cursor: 0,
            show_help: false,
            tick: 0,
            status: "Connecting...".into(),
            connected: false,
            current_contact: None,
            cmd_tx: None,
            last_input_y: None,
            invite_link: None,
            connect_input: String::new(),
            pending_new_address: false,
        }
    }
    
    pub fn selected_contact(&self) -> Option<&Contact> {
        self.contact_state.selected().and_then(|i| self.contacts.get(i))
    }
    
    pub fn next_contact(&mut self) {
        let len = self.contacts.len();
        if len == 0 { return; }
        let i = self.contact_state.selected().map(|i| (i + 1) % len).unwrap_or(0);
        self.contact_state.select(Some(i));
    }
    
    pub fn prev_contact(&mut self) {
        let len = self.contacts.len();
        if len == 0 { return; }
        let i = self.contact_state.selected().map(|i| if i == 0 { len - 1 } else { i - 1 }).unwrap_or(0);
        self.contact_state.select(Some(i));
    }
    
    pub fn cycle_panel(&mut self) {
        self.panel = match self.panel {
            Panel::Contacts => Panel::Chat,
            Panel::Chat => Panel::Input,
            Panel::Input => Panel::Contacts,
        };
        self.mode = if self.panel == Panel::Input { Mode::Input } else { Mode::Normal };
    }
    
    pub fn send_cmd(&self, cmd: &str) {
        if let Some(tx) = &self.cmd_tx {
            let _ = tx.send(cmd.to_string());
        }
    }
    
    pub fn select_contact(&mut self) {
        if let Some(contact) = self.selected_contact() {
            let name = contact.name.clone();
            self.current_contact = Some(name.clone());
            self.messages.clear();
            self.scroll = 0;
            
            if let Some(c) = self.contacts.iter_mut().find(|c| c.name == name) {
                c.unread = 0;
            }
            
            self.status = format!("Opening {}...", name);
            self.send_cmd(&format!("/tail @{} 50", name));
        }
    }
    
    pub fn refresh_chat(&self) {
        if let Some(name) = &self.current_contact {
            self.send_cmd(&format!("/tail @{} 50", name));
        }
    }
    
    pub fn send_message(&mut self) {
        if self.input.is_empty() { return; }
        
        if let Some(contact) = self.selected_contact() {
            let cmd = format!("@{} {}", contact.name, self.input);
            self.send_cmd(&cmd);
            
            self.messages.push(ChatMessage {
                sender: "You".into(),
                content: self.input.clone(),
                time: Local::now().format("%H:%M").to_string(),
                mine: true,
                status: MessageStatus::Sending,
            });
            
            self.input.clear();
            self.cursor = 0;
            self.auto_scroll();
            self.status = "Sending...".into();
        }
    }
    
    /// Show existing address
    pub fn request_address(&mut self) {
        self.send_cmd("/sa");
        self.status = "Loading address...".into();
    }
    
    /// Create NEW address (delete old first, then create new)
    pub fn create_address(&mut self) {
        self.invite_link = None;
        self.pending_new_address = true;
        // First delete old address
        self.send_cmd("/da");
        self.status = "Creating new address...".into();
    }
    
    /// Called after delete to create new
    pub fn finish_create_address(&mut self) {
        if self.pending_new_address {
            self.pending_new_address = false;
            self.send_cmd("/ad");
        }
    }
    
    pub fn connect_to_invite(&mut self) {
        if self.connect_input.is_empty() { return; }
        
        let link = self.connect_input.trim().to_string();
        let cmd = format!("/c {}", link);
        self.send_cmd(&cmd);
        self.status = "Connecting...".into();
        self.connect_input.clear();
        self.mode = Mode::Normal;
    }
    
    pub fn auto_scroll(&mut self) { self.scroll = usize::MAX; }
    pub fn scroll_up(&mut self) { self.scroll = self.scroll.saturating_sub(1); }
    pub fn scroll_down(&mut self) { self.scroll = self.scroll.saturating_add(1); }
    
    pub fn input_char(&mut self, c: char) { self.input.insert(self.cursor, c); self.cursor += 1; }
    pub fn input_backspace(&mut self) { if self.cursor > 0 { self.cursor -= 1; self.input.remove(self.cursor); } }
    pub fn input_delete(&mut self) { if self.cursor < self.input.len() { self.input.remove(self.cursor); } }
    pub fn cursor_left(&mut self) { self.cursor = self.cursor.saturating_sub(1); }
    pub fn cursor_right(&mut self) { if self.cursor < self.input.len() { self.cursor += 1; } }
    pub fn cursor_home(&mut self) { self.cursor = 0; }
    pub fn cursor_end(&mut self) { self.cursor = self.input.len(); }
    pub fn tick(&mut self) { self.tick = self.tick.wrapping_add(1); }
}

impl Default for App {
    fn default() -> Self { Self::new() }
}
