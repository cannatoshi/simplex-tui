//! WebSocket connection handling
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use std::sync::mpsc;
use std::thread;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::types::{ApiCommand, ChatMessage, Contact, SimplexEvent, MessageStatus};

pub fn spawn(
    event_tx: mpsc::Sender<SimplexEvent>,
    cmd_rx: mpsc::Receiver<String>,
) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { run_websocket(event_tx, cmd_rx).await; });
    });
}

async fn run_websocket(event_tx: mpsc::Sender<SimplexEvent>, cmd_rx: mpsc::Receiver<String>) {
    loop {
        match connect_async("ws://127.0.0.1:5225").await {
            Ok((ws, _)) => {
                let _ = event_tx.send(SimplexEvent::Connected);
                let (mut write, mut read) = ws.split();
                
                // Load contacts AND address at startup
                let cmd1 = ApiCommand::with_id("init", "/contacts");
                let _ = write.send(Message::Text(serde_json::to_string(&cmd1).unwrap().into())).await;
                
                let cmd2 = ApiCommand::with_id("addr", "/sa");
                let _ = write.send(Message::Text(serde_json::to_string(&cmd2).unwrap().into())).await;
                
                loop {
                    // Check for commands to send
                    while let Ok(cmd) = cmd_rx.try_recv() {
                        let api = ApiCommand::new(&cmd);
                        if write.send(Message::Text(serde_json::to_string(&api).unwrap().into())).await.is_err() {
                            break;
                        }
                    }
                    
                    // Check for incoming messages with timeout
                    match tokio::time::timeout(
                        tokio::time::Duration::from_millis(50),
                        read.next()
                    ).await {
                        Ok(Some(Ok(Message::Text(txt)))) => {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&txt) {
                                process_message(&json, &event_tx);
                            }
                        }
                        Ok(Some(Err(_))) | Ok(None) => break, // Connection closed
                        Ok(Some(Ok(_))) => {} // Other message types
                        Err(_) => {} // Timeout, continue loop
                    }
                }
                
                let _ = event_tx.send(SimplexEvent::Disconnected);
            }
            Err(_) => {
                let _ = event_tx.send(SimplexEvent::Disconnected);
            }
        }
        
        // Wait before reconnect attempt
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}

fn process_message(json: &serde_json::Value, event_tx: &mpsc::Sender<SimplexEvent>) {
    let resp = json.get("resp").unwrap_or(json);
    let resp_type = resp.get("type").and_then(|t| t.as_str()).unwrap_or("");
    
    match resp_type {
        "contactsList" => {
            if let Some(arr) = resp.get("contacts").and_then(|v| v.as_array()) {
                let contacts = parse_contacts(arr);
                let _ = event_tx.send(SimplexEvent::Contacts(contacts));
            }
        }
        
        "chatItems" => {
            if let Some(items) = resp.get("chatItems").and_then(|v| v.as_array()) {
                let messages = parse_chat_history(items);
                let _ = event_tx.send(SimplexEvent::Messages(messages));
            }
        }
        
        "newChatItems" => {
            if let Some(items) = resp.get("chatItems").and_then(|v| v.as_array()) {
                for item in items {
                    let contact_name = item.get("chatInfo")
                        .and_then(|ci| ci.get("contact"))
                        .and_then(|c| c.get("localDisplayName"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    
                    if let Some(ci) = item.get("chatItem") {
                        let dir = ci.get("chatDir").and_then(|d| d.get("type")).and_then(|t| t.as_str()).unwrap_or("");
                        let content_type = ci.get("content").and_then(|c| c.get("type")).and_then(|t| t.as_str()).unwrap_or("");
                        
                        if dir == "directSnd" {
                            let status = get_item_status(Some(ci));
                            let _ = event_tx.send(SimplexEvent::MessageUpdate { status });
                            
                            // Gesendete Datei anzeigen
                            if content_type == "sndMsgContent" || content_type == "file" {
                                if let Some(file) = ci.get("file") {
                                    let file_name = file.get("fileName").and_then(|f| f.as_str()).unwrap_or("file");
                                    let file_size = file.get("fileSize").and_then(|s| s.as_u64()).unwrap_or(0);
                                    let _ = event_tx.send(SimplexEvent::NewMessage {
                                        sender: "You".to_string(),
                                        text: format!("📎 {} ({} bytes)", file_name, file_size),
                                    });
                                }
                            }
                        } else {
                            // Empfangene Nachricht oder Datei
                            if let Some(text) = ci.get("content")
                                .and_then(|c| c.get("msgContent"))
                                .and_then(|mc| mc.get("text"))
                                .and_then(|t| t.as_str()) 
                            {
                                if !text.is_empty() {
                                    let _ = event_tx.send(SimplexEvent::NewMessage { 
                                        sender: contact_name.clone(), 
                                        text: text.to_string() 
                                    });
                                }
                            }
                            
                            // Empfangene Datei
                            if let Some(file) = ci.get("file") {
                                let file_name = file.get("fileName").and_then(|f| f.as_str()).unwrap_or("file");
                                let file_size = file.get("fileSize").and_then(|s| s.as_u64()).unwrap_or(0);
                                let file_id = file.get("fileId").and_then(|f| f.as_u64()).unwrap_or(0);
                                let _ = event_tx.send(SimplexEvent::NewMessage {
                                    sender: contact_name,
                                    text: format!("📎 {} ({} bytes) [/fr {} ./]", file_name, file_size, file_id),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        "newChatItem" => {
            let status = get_item_status(resp.get("chatItem"));
            let _ = event_tx.send(SimplexEvent::MessageUpdate { status });
        }
        
        "chatItemStatusUpdated" => {
            let status = get_item_status(resp.get("chatItem"));
            let _ = event_tx.send(SimplexEvent::MessageUpdate { status });
        }
        
        "chatItemsStatusesUpdated" => {
            if let Some(items) = resp.get("chatItems").and_then(|v| v.as_array()) {
                if let Some(first) = items.first() {
                    let status = get_item_status(first.get("chatItem"));
                    let _ = event_tx.send(SimplexEvent::MessageUpdate { status });
                }
            }
        }
        
        // Address show response
        "userContactLink" => {
            let link = resp.get("contactLink")
                .and_then(|cl| cl.get("connLinkContact"))
                .and_then(|cll| cll.get("connShortLink"))
                .and_then(|l| l.as_str());
            
            if let Some(link) = link {
                let _ = event_tx.send(SimplexEvent::InviteLink(link.to_string()));
            }
        }
        
        // Address created - trigger refresh
        "userContactLinkCreated" | "userContactLinkUpdated" => {
            let _ = event_tx.send(SimplexEvent::AddressCreated);
        }
        
        "userContactLinkDeleted" => {
            let _ = event_tx.send(SimplexEvent::Status("Address deleted, creating new...".into()));
            let _ = event_tx.send(SimplexEvent::AddressDeleted);
        }
        
        "invitation" => {
            if let Some(link) = resp.get("connReqInvitation").and_then(|l| l.as_str()) {
                let _ = event_tx.send(SimplexEvent::InviteLink(link.to_string()));
            }
        }
        
        "sentConfirmation" | "sentInvitation" => {
            let _ = event_tx.send(SimplexEvent::Status("Invitation sent!".into()));
        }
        
        "contactConnecting" => {
            let _ = event_tx.send(SimplexEvent::Status("Connecting...".into()));
        }
        
        "contactConnected" => {
            let name = resp.get("contact").and_then(|c| c.get("localDisplayName")).and_then(|n| n.as_str()).unwrap_or("Contact");
            let _ = event_tx.send(SimplexEvent::ContactRequest(name.to_string()));
        }
        
        "receivedContactRequest" => {
            let name = resp.get("contactRequest").and_then(|cr| cr.get("localDisplayName")).and_then(|n| n.as_str()).unwrap_or("Someone");
            let _ = event_tx.send(SimplexEvent::ContactRequest(name.to_string()));
        }
        
        "chatCmdError" | "chatError" | "cmdError" => {
            let err = extract_error(resp);
            let _ = event_tx.send(SimplexEvent::Error(err));
        }
        
        "contactInfo" => {
            let contact = resp.get("contact");
            let conn_stats = resp.get("connectionStats");
            
            let name = contact
                .and_then(|c| c.get("localDisplayName"))
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown")
                .to_string();
            
            let bio = contact
                .and_then(|c| c.get("profile"))
                .and_then(|p| p.get("shortDescr"))
                .and_then(|b| b.as_str())
                .unwrap_or("")
                .to_string();
            
            let address = contact
                .and_then(|c| c.get("profile"))
                .and_then(|p| p.get("contactLink"))
                .and_then(|l| l.as_str())
                .unwrap_or("")
                .to_string();
            
            let receiving_server = conn_stats
                .and_then(|cs| cs.get("rcvServers"))
                .and_then(|rs| rs.as_array())
                .and_then(|arr| arr.first())
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .to_string();
            
            let sending_server = conn_stats
                .and_then(|cs| cs.get("sndServers"))
                .and_then(|ss| ss.as_array())
                .and_then(|arr| arr.first())
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .to_string();
            
            let pq_encryption = contact
                .and_then(|c| c.get("activeConn"))
                .and_then(|ac| ac.get("pqEncryption"))
                .and_then(|pq| pq.as_bool())
                .unwrap_or(false);
            
            let connection_status = contact
                .and_then(|c| c.get("activeConn"))
                .and_then(|ac| ac.get("connStatus"))
                .and_then(|cs| cs.as_str())
                .unwrap_or("unknown")
                .to_string();
            
            let chat_version = contact
                .and_then(|c| c.get("activeConn"))
                .and_then(|ac| ac.get("peerChatVRange"))
                .map(|v| format!("v{}-{}", 
                    v.get("minVersion").and_then(|m| m.as_u64()).unwrap_or(0),
                    v.get("maxVersion").and_then(|m| m.as_u64()).unwrap_or(0)))
                .unwrap_or_default();
            
            let created_at = contact
                .and_then(|c| c.get("createdAt"))
                .and_then(|t| t.as_str())
                .map(|s| s.chars().take(10).collect())
                .unwrap_or_default();
            
            let updated_at = contact
                .and_then(|c| c.get("updatedAt"))
                .and_then(|t| t.as_str())
                .map(|s| s.chars().take(10).collect())
                .unwrap_or_default();
            
            let info_data = crate::types::ContactInfoData {
                name: name.clone(),
                bio,
                address,
                receiving_server,
                sending_server,
                created_at,
                updated_at,
                pq_encryption,
                connection_status,
                chat_version,
            };
            let _ = event_tx.send(SimplexEvent::ContactInfo(info_data));
        }
        "contactDeleted" => {
            let name = resp.get("contact")
                .and_then(|c| c.get("localDisplayName"))
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let _ = event_tx.send(SimplexEvent::ContactDeleted(name));
        }
        "chatCleared" => {
            let name = resp.get("chatInfo")
                .and_then(|ci| ci.get("contact"))
                .and_then(|c| c.get("localDisplayName"))
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown")
                .to_string();
            let _ = event_tx.send(SimplexEvent::ChatCleared(name));
        }
        _ => {}
    }
}

fn extract_error(resp: &serde_json::Value) -> String {
    if let Some(err) = resp.get("chatError") {
        if let Some(store_err) = err.get("storeError") {
            if let Some(t) = store_err.get("type").and_then(|t| t.as_str()) {
                return match t {
                    "duplicateContactLink" => "Address exists".into(),
                    "userContactLinkNotFound" => "No address".into(),
                    _ => t.to_string(),
                };
            }
        }
        if let Some(agent_err) = err.get("agentError") {
            if let Some(smp_err) = agent_err.get("smpErr") {
                if let Some(t) = smp_err.get("type").and_then(|t| t.as_str()) {
                    return match t {
                        "AUTH" => "Invalid link".into(),
                        _ => format!("Server: {}", t),
                    };
                }
            }
        }
    }
    "Error".into()
}

fn get_item_status(chat_item: Option<&serde_json::Value>) -> MessageStatus {
    let status_type = chat_item.and_then(|ci| ci.get("meta")).and_then(|m| m.get("itemStatus"))
        .and_then(|s| s.get("type")).and_then(|t| t.as_str()).unwrap_or("");
    match status_type {
        "sndNew" | "sndCreated" => MessageStatus::Sending,
        "sndSent" => MessageStatus::Sent,
        "sndRcvd" | "sndDelivered" => MessageStatus::Delivered,
        "sndRead" => MessageStatus::Read,
        "sndError" | "sndFailed" | "sndErrorAuth" => MessageStatus::Failed,
        _ => MessageStatus::Sent,
    }
}

fn parse_contacts(arr: &[serde_json::Value]) -> Vec<Contact> {
    arr.iter().filter_map(|c| {
        let name = c.get("localDisplayName").and_then(|n| n.as_str())?;
        Some(Contact::new(name.to_string()))
    }).collect()
}

fn parse_chat_history(items: &[serde_json::Value]) -> Vec<ChatMessage> {
    items.iter().filter_map(|item| {
        let ci = item.get("chatItem")?;
        let dir = ci.get("chatDir").and_then(|d| d.get("type")).and_then(|t| t.as_str()).unwrap_or("");
        let time = ci.get("meta").and_then(|m| m.get("itemTs")).and_then(|t| t.as_str()).and_then(|t| t.split('T').nth(1)).map(|t| t.chars().take(5).collect()).unwrap_or_default();
        let mine = dir == "directSnd";
        
        // Text-Nachricht
        let text = ci.get("content")
            .and_then(|c| c.get("msgContent"))
            .and_then(|mc| mc.get("text"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());
        
        // Datei
        let file_info = ci.get("file").map(|f| {
            let name = f.get("fileName").and_then(|n| n.as_str()).unwrap_or("file");
            let size = f.get("fileSize").and_then(|s| s.as_u64()).unwrap_or(0);
            let id = f.get("fileId").and_then(|i| i.as_u64()).unwrap_or(0);
            if mine {
                format!("📎 {} ({} bytes)", name, size)
            } else {
                format!("📎 {} ({} bytes) [/fr {} ./]", name, size, id)
            }
        });
        
        // Kombiniere Text und/oder Datei
        let content = match (text, file_info) {
            (Some(t), Some(f)) => format!("{}\n{}", t, f),
            (Some(t), None) => t,
            (None, Some(f)) => f,
            (None, None) => return None,
        };
        
        Some(ChatMessage { 
            sender: if mine { "You" } else { "Contact" }.into(), 
            content, 
            time, 
            mine, 
            status: if mine { get_item_status(Some(ci)) } else { MessageStatus::Delivered } 
        })
    }).collect()
}