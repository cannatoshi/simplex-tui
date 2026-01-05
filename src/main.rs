use anyhow::Result;
use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures_util::{SinkExt, StreamExt};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::Serialize;
use std::io;
use std::sync::mpsc;
use std::thread;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod colors {
    use ratatui::style::Color;
    pub const BLUE: Color = Color::Rgb(0, 136, 255);
    pub const BG: Color = Color::Rgb(12, 12, 16);
    pub const BG_HL: Color = Color::Rgb(25, 30, 40);
    pub const TEXT: Color = Color::Rgb(195, 200, 210);
    pub const DIM: Color = Color::Rgb(70, 75, 85);
    pub const UNREAD: Color = Color::Rgb(255, 150, 0);
    pub const DANGER: Color = Color::Rgb(180, 50, 50);
}

#[derive(Serialize)]
struct ApiCommand {
    #[serde(rename = "corrId")]
    corr_id: String,
    cmd: String,
}

#[derive(Clone)]
struct Contact { name: String, unread: usize }

#[derive(Clone)]
struct ChatMessage { sender: String, content: String, time: String, mine: bool }

#[derive(Clone, Copy, PartialEq)]
enum Panel { Contacts, Chat, Input }

#[derive(Clone, Copy, PartialEq)]
enum Mode { Normal, Input, Panic }

enum SimplexEvent {
    Connected,
    Contacts(Vec<Contact>),
    Messages(Vec<ChatMessage>),
    NewMessage { sender: String, text: String },
    Status(String),
}

struct App {
    run: bool, mode: Mode, panel: Panel,
    contacts: Vec<Contact>, state: ListState,
    messages: Vec<ChatMessage>, scroll: usize,
    input: String, help: bool, tick: u64,
    status: String, connected: bool,
    current_contact: Option<String>,
    tx: Option<mpsc::Sender<String>>,
}

impl App {
    fn new() -> Self {
        let mut s = ListState::default(); s.select(Some(0));
        Self {
            run: true, mode: Mode::Normal, panel: Panel::Contacts,
            contacts: vec![], state: s, messages: vec![],
            scroll: 0, input: String::new(), help: false, tick: 0,
            status: "Connecting...".into(), connected: false,
            current_contact: None, tx: None,
        }
    }

    fn contact(&self) -> Option<&Contact> { self.state.selected().and_then(|i| self.contacts.get(i)) }
    fn next(&mut self) { let l = self.contacts.len(); if l == 0 { return; } let i = self.state.selected().map(|i| (i+1)%l).unwrap_or(0); self.state.select(Some(i)); }
    fn prev(&mut self) { let l = self.contacts.len(); if l == 0 { return; } let i = self.state.selected().map(|i| if i==0 {l-1} else {i-1}).unwrap_or(0); self.state.select(Some(i)); }
    fn cycle(&mut self) { self.panel = match self.panel { Panel::Contacts => Panel::Chat, Panel::Chat => Panel::Input, Panel::Input => Panel::Contacts }; self.mode = if self.panel == Panel::Input { Mode::Input } else { Mode::Normal }; }
    
    fn send_message(&mut self) {
        if self.input.is_empty() { return; }
        if let (Some(contact), Some(tx)) = (self.contact(), &self.tx) {
            let cmd = format!("@{} {}", contact.name, self.input);
            let _ = tx.send(cmd);
            self.messages.push(ChatMessage { 
                sender: "You".into(), 
                content: self.input.clone(), 
                time: Local::now().format("%H:%M").to_string(),
                mine: true 
            });
            self.input.clear();
        }
    }

    fn send_cmd(&self, cmd: &str) { if let Some(tx) = &self.tx { let _ = tx.send(cmd.to_string()); } }

    fn select_contact(&mut self) {
        if let Some(name) = self.contact().map(|c| c.name.clone()) {
            self.current_contact = Some(name.clone());
            self.messages.clear();
            self.status = format!("Loading {}...", name);
            self.send_cmd(&format!("/tail @{} 50", name));
        }
    }

    fn refresh_chat(&self) {
        if let Some(name) = &self.current_contact {
            self.send_cmd(&format!("/tail @{} 50", name));
        }
    }
}

fn spawn_ws(event_tx: mpsc::Sender<SimplexEvent>, cmd_rx: mpsc::Receiver<String>) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match connect_async("ws://127.0.0.1:5225").await {
                Ok((ws, _)) => {
                    let _ = event_tx.send(SimplexEvent::Connected);
                    let (mut write, mut read) = ws.split();

                    // Get contacts on start
                    let cmd = ApiCommand { corr_id: "init".into(), cmd: "/contacts".into() };
                    let _ = write.send(Message::Text(serde_json::to_string(&cmd).unwrap())).await;

                    // Command sender task
                    tokio::spawn(async move {
                        loop {
                            if let Ok(cmd) = cmd_rx.try_recv() {
                                let api = ApiCommand { 
                                    corr_id: format!("{}", chrono::Utc::now().timestamp_millis()), 
                                    cmd 
                                };
                                let _ = write.send(Message::Text(serde_json::to_string(&api).unwrap())).await;
                            }
                            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        }
                    });

                    // Response/Event reader
                    while let Some(Ok(Message::Text(txt))) = read.next().await {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&txt) {
                            // Check if this is a push event (no corrId) or command response (has corrId)
                            let has_corr_id = json.get("corrId").is_some();
                            
                            let resp = json.get("resp").unwrap_or(&json);
                            let resp_type = resp.get("type").and_then(|t| t.as_str()).unwrap_or("");
                            
                            // ========== PUSH EVENTS (no corrId field) ==========
                            if !has_corr_id {
                                // New incoming message - REAL TIME!
                                if resp_type == "newChatItems" {
                                    if let Some(items) = resp.get("chatItems").and_then(|v| v.as_array()) {
                                        for item in items {
                                            // Get sender from chatInfo.contact.localDisplayName
                                            let sender = item.get("chatInfo")
                                                .and_then(|ci| ci.get("contact"))
                                                .and_then(|c| c.get("localDisplayName"))
                                                .and_then(|n| n.as_str())
                                                .unwrap_or("Contact");
                                            
                                            // Get chatItem
                                            let ci = item.get("chatItem");
                                            
                                            // Get direction
                                            let dir_type = ci
                                                .and_then(|c| c.get("chatDir"))
                                                .and_then(|d| d.get("type"))
                                                .and_then(|t| t.as_str())
                                                .unwrap_or("");
                                            
                                            // Skip sent messages (we already show them)
                                            if dir_type == "directSnd" {
                                                continue;
                                            }
                                            
                                            // Get text
                                            let text = ci
                                                .and_then(|c| c.get("content"))
                                                .and_then(|c| c.get("msgContent"))
                                                .and_then(|mc| mc.get("text"))
                                                .and_then(|t| t.as_str());
                                            
                                            if let Some(text) = text {
                                                let _ = event_tx.send(SimplexEvent::NewMessage { 
                                                    sender: sender.to_string(), 
                                                    text: text.to_string() 
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // ========== COMMAND RESPONSES (has corrId) ==========
                            else {
                                // Contacts list
                                if resp_type == "contactsList" {
                                    if let Some(arr) = resp.get("contacts").and_then(|v| v.as_array()) {
                                        let contacts: Vec<Contact> = arr.iter().filter_map(|c| {
                                            c.get("localDisplayName").and_then(|n| n.as_str())
                                                .map(|n| Contact { name: n.to_string(), unread: 0 })
                                        }).collect();
                                        let _ = event_tx.send(SimplexEvent::Contacts(contacts));
                                    }
                                }

                                // Chat history
                                if resp_type == "chatItems" {
                                    if let Some(items) = resp.get("chatItems").and_then(|v| v.as_array()) {
                                        let messages = parse_chat_history(items);
                                        let _ = event_tx.send(SimplexEvent::Messages(messages));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => { let _ = event_tx.send(SimplexEvent::Status(format!("Error: {}", e))); }
            }
        });
    });
}

fn parse_chat_history(items: &[serde_json::Value]) -> Vec<ChatMessage> {
    items.iter().filter_map(|item| {
        let ci = item.get("chatItem")?;
        
        let text = ci.get("content")
            .and_then(|c| c.get("msgContent"))
            .and_then(|mc| mc.get("text"))
            .and_then(|t| t.as_str())?;
        
        let dir_type = ci.get("chatDir")
            .and_then(|d| d.get("type"))
            .and_then(|t| t.as_str())
            .unwrap_or("");
        
        let time = ci.get("meta")
            .and_then(|m| m.get("itemTs"))
            .and_then(|t| t.as_str())
            .and_then(|t| t.split('T').nth(1))
            .map(|t| t[..5].to_string())
            .unwrap_or_default();

        let mine = dir_type == "directSnd";
        
        // Get sender from chatInfo for received messages
        let sender = if mine { 
            "You".to_string() 
        } else {
            item.get("chatInfo")
                .and_then(|ci| ci.get("contact"))
                .and_then(|c| c.get("localDisplayName"))
                .and_then(|n| n.as_str())
                .unwrap_or("Contact")
                .to_string()
        };
        
        Some(ChatMessage {
            sender,
            content: text.to_string(),
            time,
            mine,
        })
    }).collect()
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let outer = Block::default().borders(Borders::ALL).border_style(Style::default().fg(colors::BLUE)).style(Style::default().bg(colors::BG));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let cols = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(30), Constraint::Length(1), Constraint::Min(1)]).split(inner);
    let left = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(1), Constraint::Length(1)]).split(cols[0]);

    render_contacts(f, left[0], app);
    render_actions(f, left[1]);
    render_divider(f, cols[1]);

    let right = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1), Constraint::Length(1)]).split(cols[2]);

    render_header(f, right[0], app);
    render_messages(f, right[1], app);
    render_input(f, right[2], app);
    render_status(f, right[3], app);

    if app.help { render_help(f); }
    if app.mode == Mode::Panic { render_panic(f, app); }
}

fn render_contacts(f: &mut Frame, area: Rect, app: &mut App) {
    if app.contacts.is_empty() {
        f.render_widget(Paragraph::new(Line::from(Span::styled("  No contacts", Style::default().fg(colors::DIM)))), area);
        return;
    }
    let items: Vec<ListItem> = app.contacts.iter().enumerate().map(|(i, c)| {
        let sel = app.state.selected() == Some(i);
        let bg = if sel { colors::BG_HL } else { colors::BG };
        let marker = if sel { "▸ " } else { "  " };
        let style = if sel { Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD) } else { Style::default().fg(colors::TEXT) };
        let unread = if c.unread > 0 { Span::styled(format!(" ({})", c.unread), Style::default().fg(colors::UNREAD)) } else { Span::raw("") };
        ListItem::new(Line::from(vec![Span::styled(marker, Style::default().fg(colors::BLUE)), Span::styled(&c.name, style), unread])).style(Style::default().bg(bg))
    }).collect();
    f.render_stateful_widget(List::new(items), area, &mut app.state);
}

fn render_actions(f: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled(" [i]", Style::default().fg(colors::BLUE)), Span::styled(" Add ", Style::default().fg(colors::DIM)),
        Span::styled("[r]", Style::default().fg(colors::BLUE)), Span::styled(" Refresh ", Style::default().fg(colors::DIM)),
        Span::styled("[?]", Style::default().fg(colors::BLUE)), Span::styled(" Help", Style::default().fg(colors::DIM)),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn render_divider(f: &mut Frame, area: Rect) {
    let lines: Vec<Line> = (0..area.height).map(|_| Line::from(Span::styled("│", Style::default().fg(colors::BLUE)))).collect();
    f.render_widget(Paragraph::new(lines), area);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let name = app.current_contact.as_ref().map(|s| s.as_str()).unwrap_or("Select contact");
    f.render_widget(Paragraph::new(Line::from(Span::styled(format!(" {}", name), Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD)))), area);
}

fn render_messages(f: &mut Frame, area: Rect, app: &App) {
    if app.messages.is_empty() {
        f.render_widget(Paragraph::new(Line::from(Span::styled("  No messages", Style::default().fg(colors::DIM)))), area);
        return;
    }
    let mut lines: Vec<Line> = Vec::new();
    for m in &app.messages {
        let color = if m.mine { colors::BLUE } else { colors::TEXT };
        lines.push(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(&m.sender, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("  {}", m.time), Style::default().fg(colors::DIM)),
        ]));
        lines.push(Line::from(Span::styled(format!(" {}", m.content), Style::default().fg(colors::TEXT))));
        lines.push(Line::from(""));
    }
    
    // Auto-scroll to bottom
    let total_lines = lines.len() as u16;
    let visible_height = area.height;
    let scroll = if total_lines > visible_height { total_lines - visible_height } else { 0 };
    
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((scroll, 0)), area);
}

fn render_input(f: &mut Frame, area: Rect, app: &App) {
    let active = app.mode == Mode::Input;
    let cursor = if active && app.tick % 10 < 5 { "█" } else { "" };
    let txt = if app.input.is_empty() && !active { Span::styled(" Type message...", Style::default().fg(colors::DIM)) } 
              else { Span::styled(format!(" {}{}", app.input, cursor), Style::default().fg(colors::TEXT)) };
    f.render_widget(Paragraph::new(Line::from(txt)), area);
}

fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let dot = if app.connected { "●" } else { "○" };
    let color = if app.connected { colors::BLUE } else { colors::UNREAD };
    f.render_widget(Paragraph::new(Line::from(vec![
        Span::styled(format!(" {} ", dot), Style::default().fg(color)),
        Span::styled(&app.status, Style::default().fg(colors::DIM)),
    ])), area);
}

fn render_help(f: &mut Frame) {
    let a = centered(45, 50, f.area());
    f.render_widget(Clear, a);
    let help = vec![
        Line::from(""), Line::from(Span::styled(" KEYS", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD))), Line::from(""),
        Line::from(vec![Span::styled(" Tab   ", Style::default().fg(colors::BLUE)), Span::styled("Switch panel", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" j/k   ", Style::default().fg(colors::BLUE)), Span::styled("Navigate/Scroll", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" Enter ", Style::default().fg(colors::BLUE)), Span::styled("Select/Send", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" r     ", Style::default().fg(colors::BLUE)), Span::styled("Refresh", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" Esc   ", Style::default().fg(colors::BLUE)), Span::styled("Back", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" q     ", Style::default().fg(colors::BLUE)), Span::styled("Quit", Style::default().fg(colors::TEXT))]),
        Line::from(""), Line::from(vec![Span::styled(" F12   ", Style::default().fg(colors::DANGER)), Span::styled("PANIC", Style::default().fg(colors::DANGER))]), Line::from(""),
    ];
    f.render_widget(Paragraph::new(help).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(colors::BLUE)).style(Style::default().bg(colors::BG))), a);
}

fn render_panic(f: &mut Frame, app: &App) {
    let a = centered(35, 40, f.area());
    f.render_widget(Clear, a);
    let flash = if app.tick % 8 < 4 { colors::DANGER } else { colors::DIM };
    let text = vec![
        Line::from(""), Line::from(Span::styled(" PANIC", Style::default().fg(flash).add_modifier(Modifier::BOLD))), Line::from(""),
        Line::from(vec![Span::styled(" [1] ", Style::default().fg(colors::BLUE)), Span::styled("LOCK", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" [2] ", Style::default().fg(colors::BLUE)), Span::styled("CLOSE", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" [3] ", Style::default().fg(colors::DANGER)), Span::styled("NUKE", Style::default().fg(colors::DANGER))]),
        Line::from(""), Line::from(Span::styled(" [Esc] Cancel", Style::default().fg(colors::DIM))), Line::from(""),
    ];
    f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(flash)).style(Style::default().bg(colors::BG))), a);
}

fn centered(px: u16, py: u16, r: Rect) -> Rect {
    let v = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage((100-py)/2), Constraint::Percentage(py), Constraint::Percentage((100-py)/2)]).split(r);
    Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage((100-px)/2), Constraint::Percentage(px), Constraint::Percentage((100-px)/2)]).split(v[1])[1]
}

fn handle(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(k) = event::read()? {
            match app.mode {
                Mode::Normal => match k.code {
                    KeyCode::Char('q') => app.run = false,
                    KeyCode::Char('?') => app.help = !app.help,
                    KeyCode::Char('r') => { app.send_cmd("/contacts"); app.refresh_chat(); }
                    KeyCode::Tab => app.cycle(),
                    KeyCode::Char('j') | KeyCode::Down => if app.panel == Panel::Contacts { app.next(); } else { app.scroll = app.scroll.saturating_add(3); },
                    KeyCode::Char('k') | KeyCode::Up => if app.panel == Panel::Contacts { app.prev(); } else { app.scroll = app.scroll.saturating_sub(3); },
                    KeyCode::Enter => if app.panel == Panel::Contacts { app.select_contact(); app.panel = Panel::Input; app.mode = Mode::Input; },
                    KeyCode::Esc => app.help = false,
                    KeyCode::F(12) => app.mode = Mode::Panic,
                    _ => {}
                },
                Mode::Input => match k.code {
                    KeyCode::Esc => { app.mode = Mode::Normal; app.panel = Panel::Chat; }
                    KeyCode::Enter => app.send_message(),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Tab => { app.mode = Mode::Normal; app.cycle(); }
                    _ => {}
                },
                Mode::Panic => match k.code {
                    KeyCode::Esc | KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') => app.mode = Mode::Normal,
                    _ => {}
                },
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut out = io::stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let mut term = Terminal::new(CrosstermBackend::new(out))?;

    let mut app = App::new();
    let (event_tx, event_rx) = mpsc::channel();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    app.tx = Some(cmd_tx);

    spawn_ws(event_tx, cmd_rx);

    while app.run {
        app.tick = app.tick.wrapping_add(1);

        while let Ok(ev) = event_rx.try_recv() {
            match ev {
                SimplexEvent::Connected => { app.connected = true; app.status = "Connected - Real-time!".into(); }
                SimplexEvent::Contacts(c) => { app.contacts = c; app.status = format!("{} contacts", app.contacts.len()); }
                SimplexEvent::Messages(msgs) => { 
                    app.messages = msgs; 
                    app.status = format!("{} messages", app.messages.len()); 
                }
                SimplexEvent::NewMessage { sender, text } => {
                    // Real-time incoming message!
                    app.messages.push(ChatMessage { 
                        sender: sender.clone(), 
                        content: text, 
                        time: Local::now().format("%H:%M").to_string(),
                        mine: false 
                    });
                    app.status = format!("📩 {} ", sender);
                }
                SimplexEvent::Status(s) => app.status = s,
            }
        }

        term.draw(|f| ui(f, &mut app))?;
        handle(&mut app)?;
    }

    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;
    Ok(())
}
