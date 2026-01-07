//! Application state and logic
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use std::sync::mpsc;
use std::time::Instant;
use chrono::Local;
use ratatui::widgets::ListState;

use crate::types::{Contact, ChatMessage, Panel, Mode, MessageStatus, ContactOption};

const DOUBLE_CLICK_MS: u128 = 400;

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
    
    pub contact_for_options: Option<String>,
    pub contact_info_data: Option<crate::types::ContactInfoData>,
    pub option_selection: usize,
    pub confirm_action: Option<ContactOption>,
    
    last_click_time: Option<Instant>,
    last_click_index: Option<usize>,
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
            
            contact_for_options: None,
            contact_info_data: None,
            option_selection: 0,
            confirm_action: None,
            
            last_click_time: None,
            last_click_index: None,
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
    
    pub fn check_double_click(&mut self, contact_index: usize) -> bool {
        let now = Instant::now();
        let is_double = if let (Some(last_time), Some(last_idx)) = (self.last_click_time, self.last_click_index) {
            last_idx == contact_index && now.duration_since(last_time).as_millis() < DOUBLE_CLICK_MS
        } else {
            false
        };
        
        self.last_click_time = Some(now);
        self.last_click_index = Some(contact_index);
        
        is_double
    }
    
    pub fn open_contact_options(&mut self, contact_name: String) {
        self.contact_for_options = Some(contact_name);
        self.option_selection = 0;
        self.confirm_action = None;
        self.mode = Mode::ContactOptions;
    }
    
    pub fn close_contact_options(&mut self) {
        self.contact_for_options = None;
        self.option_selection = 0;
        self.confirm_action = None;
        self.mode = Mode::Normal;
    }
    
    pub fn next_option(&mut self) {
        let len = ContactOption::all().len();
        self.option_selection = (self.option_selection + 1) % len;
    }
    
    pub fn prev_option(&mut self) {
        let len = ContactOption::all().len();
        self.option_selection = if self.option_selection == 0 { len - 1 } else { self.option_selection - 1 };
    }
    
    pub fn execute_selected_option(&mut self) {
        let options = ContactOption::all();
        let option = options.get(self.option_selection).copied().unwrap_or(ContactOption::Cancel);
        
        if option.is_destructive() && self.confirm_action.is_none() {
            self.confirm_action = Some(option);
            self.status = format!("Press Enter again to confirm {}", option.label());
        } else {
            self.execute_option(option);
        }
    }
    
    pub fn execute_option(&mut self, option: ContactOption) {
        if let Some(contact_name) = &self.contact_for_options.clone() {
            match option {
                ContactOption::DeleteContact => {
                    self.delete_contact(contact_name);
                }
                ContactOption::ClearChat => {
                    self.clear_chat(contact_name);
                }
                ContactOption::ContactInfo => {
                    self.get_contact_info(contact_name);
                }
                ContactOption::Cancel => {
                    self.close_contact_options();
                }
            }
        } else {
            self.close_contact_options();
        }
    }
    
    pub fn delete_contact(&mut self, name: &str) {
        self.send_cmd(&format!("/d '{}'", name));
        
        self.contacts.retain(|c| c.name != name);
        
        if self.contact_state.selected().unwrap_or(0) >= self.contacts.len() && !self.contacts.is_empty() {
            self.contact_state.select(Some(self.contacts.len() - 1));
        } else if self.contacts.is_empty() {
            self.contact_state.select(Some(0));
        }
        
        if self.current_contact.as_ref() == Some(&name.to_string()) {
            self.current_contact = None;
            self.messages.clear();
        }
        
        self.status = format!("Deleted: {}", name);
        self.close_contact_options();
    }
    
    pub fn clear_chat(&mut self, name: &str) {
        self.send_cmd(&format!("/clear '{}'", name));
        self.status = format!("Clearing chat with {}...", name);
        
        if self.current_contact.as_ref() == Some(&name.to_string()) {
            self.messages.clear();
        }
        
        self.close_contact_options();
    }
    
    pub fn get_contact_info(&mut self, name: &str) {
        self.send_cmd(&format!("/info '{}'", name));
        self.status = format!("Loading info for {}...", name);
    }
    
    pub fn on_contact_deleted(&mut self, name: &str) {
        self.contacts.retain(|c| c.name != name);
        
        if self.contact_state.selected().unwrap_or(0) >= self.contacts.len() && !self.contacts.is_empty() {
            self.contact_state.select(Some(self.contacts.len() - 1));
        }
        
        self.status = format!("Deleted: {}", name);
    }
    
    pub fn on_chat_cleared(&mut self, name: &str) {
        self.status = format!("Chat cleared: {}", name);
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
    
    pub fn request_address(&mut self) {
        self.send_cmd("/sa");
        self.status = "Loading address...".into();
    }
    
    pub fn create_address(&mut self) {
        self.invite_link = None;
        self.pending_new_address = true;
        self.send_cmd("/da");
        self.status = "Creating new address...".into();
    }
    
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
