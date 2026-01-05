use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

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

#[derive(Clone)]
struct Contact { name: String, unread: usize, preview: String }

#[derive(Clone)]
struct Message { sender: String, content: String, time: DateTime<Local>, mine: bool }

#[derive(Clone, Copy, PartialEq)]
enum Panel { Contacts, Chat, Input }

#[derive(Clone, Copy, PartialEq)]
enum Mode { Normal, Input, Panic }

struct App {
    run: bool, mode: Mode, panel: Panel,
    contacts: Vec<Contact>, state: ListState,
    messages: Vec<Message>, scroll: usize,
    input: String, help: bool, tick: u64,
}

impl App {
    fn new() -> Self {
        let mut s = ListState::default(); s.select(Some(0));
        Self {
            run: true, mode: Mode::Normal, panel: Panel::Contacts,
            contacts: vec![
                Contact { name: "Alice".into(), unread: 2, preview: "Don't forget: activate Tor!".into() },
                Contact { name: "Bob".into(), unread: 0, preview: "See you tomorrow".into() },
                Contact { name: "SecureDrop".into(), unread: 0, preview: "Files received".into() },
                Contact { name: "Whistleblower".into(), unread: 5, preview: "Urgent: check docs".into() },
                Contact { name: "Anonymous".into(), unread: 0, preview: "Connection closed".into() },
            ],
            state: s,
            messages: vec![
                Message { sender: "Alice".into(), content: "Hey! Are you there?".into(), time: Local::now() - chrono::Duration::minutes(30), mine: false },
                Message { sender: "You".into(), content: "Yes, I'm online. What's up?".into(), time: Local::now() - chrono::Duration::minutes(28), mine: true },
                Message { sender: "Alice".into(), content: "Did you get the documents?".into(), time: Local::now() - chrono::Duration::minutes(25), mine: false },
                Message { sender: "You".into(), content: "Yes, all received and decrypted!".into(), time: Local::now() - chrono::Duration::minutes(20), mine: true },
                Message { sender: "Alice".into(), content: "Perfect. Meet tomorrow at 15:00?".into(), time: Local::now() - chrono::Duration::minutes(15), mine: false },
                Message { sender: "You".into(), content: "Sure, I'll be there.".into(), time: Local::now() - chrono::Duration::minutes(10), mine: true },
                Message { sender: "Alice".into(), content: "Don't forget: activate Tor!".into(), time: Local::now() - chrono::Duration::minutes(5), mine: false },
            ],
            scroll: 0, input: String::new(), help: false, tick: 0,
        }
    }
    fn contact(&self) -> Option<&Contact> { self.state.selected().and_then(|i| self.contacts.get(i)) }
    fn next(&mut self) { let l = self.contacts.len(); let i = self.state.selected().map(|i| (i+1)%l).unwrap_or(0); self.state.select(Some(i)); }
    fn prev(&mut self) { let l = self.contacts.len(); let i = self.state.selected().map(|i| if i==0 {l-1} else {i-1}).unwrap_or(0); self.state.select(Some(i)); }
    fn cycle(&mut self) { 
        self.panel = match self.panel { Panel::Contacts => Panel::Chat, Panel::Chat => Panel::Input, Panel::Input => Panel::Contacts }; 
        self.mode = if self.panel == Panel::Input { Mode::Input } else { Mode::Normal }; 
    }
    fn send(&mut self) { 
        if !self.input.is_empty() { 
            self.messages.push(Message { sender: "You".into(), content: self.input.clone(), time: Local::now(), mine: true }); 
            self.input.clear(); 
        } 
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();
    
    // Outer frame - one consistent border
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BLUE))
        .style(Style::default().bg(colors::BG));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    // Split inner area
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Left: contacts + actions
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(cols[0]);

    render_contacts(f, left[0], app);
    render_actions(f, left[1], app);

    // Divider
    render_divider(f, cols[1]);

    // Right: header + messages + input
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(cols[2]);

    render_header(f, right[0], app);
    render_messages(f, right[1], app);
    render_input(f, right[2], app);

    if app.help { render_help(f); }
    if app.mode == Mode::Panic { render_panic(f, app); }
}

fn render_contacts(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app.contacts.iter().enumerate().map(|(i, c)| {
        let sel = app.state.selected() == Some(i);
        let bg = if sel { colors::BG_HL } else { colors::BG };
        
        let marker = if sel { "▸ " } else { "  " };
        let name_style = if sel { 
            Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD) 
        } else { 
            Style::default().fg(colors::TEXT) 
        };
        
        let unread = if c.unread > 0 {
            Span::styled(format!(" ({})", c.unread), Style::default().fg(colors::UNREAD))
        } else { Span::raw("") };

        let line1 = Line::from(vec![
            Span::styled(marker, Style::default().fg(colors::BLUE)),
            Span::styled(&c.name, name_style),
            unread,
        ]);
        let line2 = Line::from(Span::styled(format!("   {}", c.preview), Style::default().fg(colors::DIM)));

        ListItem::new(vec![line1, line2]).style(Style::default().bg(bg))
    }).collect();

    f.render_stateful_widget(List::new(items), area, &mut app.state);
}

fn render_actions(f: &mut Frame, area: Rect, app: &App) {
    let acts = Line::from(vec![
        Span::styled(" [i]", Style::default().fg(colors::BLUE)),
        Span::styled(" Add ", Style::default().fg(colors::DIM)),
        Span::styled("[c]", Style::default().fg(colors::BLUE)),
        Span::styled(" Link ", Style::default().fg(colors::DIM)),
        Span::styled("[?]", Style::default().fg(colors::BLUE)),
        Span::styled(" Help", Style::default().fg(colors::DIM)),
    ]);
    f.render_widget(Paragraph::new(acts), area);
}

fn render_divider(f: &mut Frame, area: Rect) {
    let mut lines = Vec::new();
    for _ in 0..area.height {
        lines.push(Line::from(Span::styled("│", Style::default().fg(colors::BLUE))));
    }
    f.render_widget(Paragraph::new(lines), area);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let name = app.contact().map(|c| c.name.clone()).unwrap_or_default();
    let header = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(&name, Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(Paragraph::new(header), area);
}

fn render_messages(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();
    
    for m in &app.messages {
        let time = m.time.format("%H:%M").to_string();
        let name_color = if m.mine { colors::BLUE } else { colors::TEXT };
        
        lines.push(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(&m.sender, Style::default().fg(name_color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("  {}", time), Style::default().fg(colors::DIM)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(&m.content, Style::default().fg(colors::TEXT)),
        ]));
        lines.push(Line::from(""));
    }

    f.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((app.scroll as u16, 0)),
        area
    );
}

fn render_input(f: &mut Frame, area: Rect, app: &App) {
    let active = app.panel == Panel::Input || app.mode == Mode::Input;
    let cursor = if active && app.tick % 10 < 5 { "█" } else { "" };
    
    let txt = if app.input.is_empty() && !active {
        Span::styled(" Type a message...", Style::default().fg(colors::DIM))
    } else {
        Span::styled(format!(" {}{}", app.input, cursor), Style::default().fg(colors::TEXT))
    };
    
    f.render_widget(Paragraph::new(Line::from(txt)), area);
}

fn render_help(f: &mut Frame) {
    let area = centered(45, 50, f.area());
    f.render_widget(Clear, area);

    let help = vec![
        Line::from(""),
        Line::from(Span::styled(" KEYBOARD", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![Span::styled(" Tab    ", Style::default().fg(colors::BLUE)), Span::styled("Switch panel", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" j/k    ", Style::default().fg(colors::BLUE)), Span::styled("Navigate", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" Enter  ", Style::default().fg(colors::BLUE)), Span::styled("Select / Send", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" Esc    ", Style::default().fg(colors::BLUE)), Span::styled("Back", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" q      ", Style::default().fg(colors::BLUE)), Span::styled("Quit", Style::default().fg(colors::TEXT))]),
        Line::from(""),
        Line::from(vec![Span::styled(" F12    ", Style::default().fg(colors::DANGER)), Span::styled("PANIC", Style::default().fg(colors::DANGER))]),
        Line::from(""),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BLUE))
        .style(Style::default().bg(colors::BG));

    f.render_widget(Paragraph::new(help).block(block), area);
}

fn render_panic(f: &mut Frame, app: &App) {
    let area = centered(35, 40, f.area());
    f.render_widget(Clear, area);

    let flash = if app.tick % 8 < 4 { colors::DANGER } else { colors::DIM };

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(" PANIC MODE", Style::default().fg(flash).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![Span::styled(" [1] ", Style::default().fg(colors::BLUE)), Span::styled("LOCK", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" [2] ", Style::default().fg(colors::BLUE)), Span::styled("CLOSE", Style::default().fg(colors::TEXT))]),
        Line::from(vec![Span::styled(" [3] ", Style::default().fg(colors::DANGER)), Span::styled("NUKE", Style::default().fg(colors::DANGER))]),
        Line::from(""),
        Line::from(Span::styled(" [Esc] Cancel", Style::default().fg(colors::DIM))),
        Line::from(""),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(flash))
        .style(Style::default().bg(colors::BG));

    f.render_widget(Paragraph::new(text).block(block), area);
}

fn centered(px: u16, py: u16, r: Rect) -> Rect {
    let v = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100-py)/2), Constraint::Percentage(py), Constraint::Percentage((100-py)/2)]).split(r);
    Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100-px)/2), Constraint::Percentage(px), Constraint::Percentage((100-px)/2)]).split(v[1])[1]
}

fn handle(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(k) = event::read()? {
            match app.mode {
                Mode::Normal => match k.code {
                    KeyCode::Char('q') => app.run = false,
                    KeyCode::Char('?') => app.help = !app.help,
                    KeyCode::Tab => app.cycle(),
                    KeyCode::Char('j') | KeyCode::Down => { if app.panel == Panel::Contacts { app.next(); } else { app.scroll = app.scroll.saturating_add(1); } }
                    KeyCode::Char('k') | KeyCode::Up => { if app.panel == Panel::Contacts { app.prev(); } else { app.scroll = app.scroll.saturating_sub(1); } }
                    KeyCode::Enter => { if app.panel == Panel::Contacts { app.panel = Panel::Input; app.mode = Mode::Input; } }
                    KeyCode::Esc => app.help = false,
                    KeyCode::F(12) => app.mode = Mode::Panic,
                    _ => {}
                },
                Mode::Input => match k.code {
                    KeyCode::Esc => { app.mode = Mode::Normal; app.panel = Panel::Chat; }
                    KeyCode::Enter => app.send(),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Tab => { app.mode = Mode::Normal; app.cycle(); }
                    _ => {}
                },
                Mode::Panic => match k.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') => app.mode = Mode::Normal,
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
    while app.run {
        app.tick = app.tick.wrapping_add(1);
        term.draw(|f| ui(f, &mut app))?;
        handle(&mut app)?;
    }

    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;
    Ok(())
}
