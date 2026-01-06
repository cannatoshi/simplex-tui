//! Type definitions for SimpleX TUI
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Contact {
    pub name: String,
    pub unread: usize,
    pub online: bool,
}

impl Contact {
    pub fn new(name: String) -> Self {
        Self { name, unread: 0, online: false }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub time: String,
    pub mine: bool,
    pub status: MessageStatus,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Panel { Contacts, Chat, Input }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode { Normal, Input, Panic, AddContact }

#[derive(Debug)]
pub enum SimplexEvent {
    Connected,
    Disconnected,
    Contacts(Vec<Contact>),
    Messages(Vec<ChatMessage>),
    NewMessage { sender: String, text: String },
    MessageUpdate { status: MessageStatus },
    InviteLink(String),
    AddressDeleted,
    AddressCreated,
    ContactRequest(String),
    Error(String),
    Status(String),
}

#[derive(Serialize)]
pub struct ApiCommand {
    #[serde(rename = "corrId")]
    pub corr_id: String,
    pub cmd: String,
}

impl ApiCommand {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self {
            corr_id: format!("{}", chrono::Utc::now().timestamp_millis()),
            cmd: cmd.into(),
        }
    }
    
    pub fn with_id(corr_id: impl Into<String>, cmd: impl Into<String>) -> Self {
        Self { corr_id: corr_id.into(), cmd: cmd.into() }
    }
}
