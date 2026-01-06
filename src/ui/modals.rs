//! Modal dialogs
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::App;
use crate::colors;

// Button positions for mouse clicks
pub static mut BUTTON_REFRESH: Option<Rect> = None;
pub static mut BUTTON_CREATE: Option<Rect> = None;
pub static mut BUTTON_CLOSE: Option<Rect> = None;

pub fn render_help(frame: &mut Frame) {
    let area = frame.area();
    frame.render_widget(Clear, area);
    
    let block = Block::default()
        .title(Span::styled(" Help ", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL).border_style(Style::default().fg(colors::BLUE))
        .style(Style::default().bg(colors::BG));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(" Navigation", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD))),
        Line::from(""),
        kl("Tab", "Switch panel"),
        kl("j / ↓", "Move down / Scroll"),
        kl("k / ↑", "Move up / Scroll"),
        kl("Enter", "Select contact / Send message"),
        kl("Esc", "Back / Close modal"),
        Line::from(""),
        Line::from(Span::styled(" Actions", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD))),
        Line::from(""),
        kl("i", "Add contact"),
        kl("r", "Refresh contacts & chat"),
        kl("?", "Toggle this help"),
        kl("q", "Quit application"),
        Line::from(""),
        Line::from(Span::styled(" Emergency", Style::default().fg(colors::DANGER).add_modifier(Modifier::BOLD))),
        Line::from(""),
        klc("F12", "PANIC MODE - Emergency actions", colors::DANGER),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(" Press [Esc] or [?] to close", Style::default().fg(colors::TEXT_DIM))),
    ];
    
    frame.render_widget(Paragraph::new(lines), inner);
}

pub fn render_panic(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(Clear, area);
    
    let flash = if (app.tick / 4) % 2 == 0 { colors::DANGER } else { colors::TEXT_DIM };
    let block = Block::default()
        .title(Span::styled(" ⚠ PANIC MODE ⚠ ", Style::default().fg(flash).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL).border_style(Style::default().fg(flash))
        .style(Style::default().bg(colors::BG));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(" Emergency Actions:", Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(""),
        po("1", "LOCK", "Lock screen immediately", colors::BLUE),
        Line::from(""),
        po("2", "CLOSE", "Exit application", colors::BLUE),
        Line::from(""),
        po("3", "NUKE", "Destroy all local data", colors::DANGER),
        Line::from(""),
        po("4", "SSH", "Enter stealth SSH mode", colors::BLUE),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(" Press [Esc] to cancel", Style::default().fg(colors::TEXT_DIM))),
    ];
    
    frame.render_widget(Paragraph::new(lines), inner);
}

pub fn render_add_contact(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(Clear, area);
    
    let block = Block::default()
        .title(Span::styled(" Add Contact ", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL).border_style(Style::default().fg(colors::BLUE))
        .style(Style::default().bg(colors::BG));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(" Your SimpleX Address:", Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    if let Some(link) = &app.invite_link {
        let cw = (inner.width as usize).saturating_sub(4);
        for chunk in link.chars().collect::<Vec<_>>().chunks(cw) {
            lines.push(Line::from(Span::styled(
                format!(" {}", chunk.iter().collect::<String>()),
                Style::default().fg(colors::BLUE)
            )));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(" ✓ Share this link with your contact", Style::default().fg(colors::SUCCESS))));
    } else {
        lines.push(Line::from(Span::styled(" Loading address...", Style::default().fg(colors::TEXT_DIM))));
        lines.push(Line::from(Span::styled(" Press [g] to refresh or [c] to create new", Style::default().fg(colors::TEXT_DIM))));
    }
    
    let sep: String = "─".repeat((inner.width as usize).saturating_sub(2));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(format!(" {}", sep), Style::default().fg(colors::BORDER))));
    lines.push(Line::from(""));
    
    lines.push(Line::from(Span::styled(" Connect to someone:", Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));
    
    // Input field
    let input_w = (inner.width as usize).saturating_sub(4);
    let top = format!(" ┌{}┐", "─".repeat(input_w));
    let bot = format!(" └{}┘", "─".repeat(input_w));
    
    lines.push(Line::from(Span::styled(top, Style::default().fg(colors::BLUE))));
    
    let cursor = if (app.tick / 5) % 2 == 0 { "▌" } else { " " };
    
    if app.connect_input.is_empty() {
        let placeholder = "Paste invite link here...";
        let pad = input_w.saturating_sub(placeholder.len() + 2);
        lines.push(Line::from(vec![
            Span::styled(" │ ".to_string(), Style::default().fg(colors::BLUE)),
            Span::styled(placeholder.to_string(), Style::default().fg(colors::TEXT_DIM)),
            Span::styled(cursor.to_string(), Style::default().fg(colors::BLUE)),
            Span::styled(" ".repeat(pad), Style::default()),
            Span::styled("│".to_string(), Style::default().fg(colors::BLUE)),
        ]));
    } else {
        let display: String = app.connect_input.chars().take(input_w - 4).collect();
        let pad = input_w.saturating_sub(display.len() + 2);
        lines.push(Line::from(vec![
            Span::styled(" │ ".to_string(), Style::default().fg(colors::BLUE)),
            Span::styled(display, Style::default().fg(colors::TEXT)),
            Span::styled(cursor.to_string(), Style::default().fg(colors::BLUE)),
            Span::styled(" ".repeat(pad), Style::default()),
            Span::styled("│".to_string(), Style::default().fg(colors::BLUE)),
        ]));
    }
    
    lines.push(Line::from(Span::styled(bot, Style::default().fg(colors::BLUE))));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Press [Enter] to connect", Style::default().fg(colors::TEXT_DIM))));
    lines.push(Line::from(""));
    
    // Store button Y position
    let button_y = inner.y + lines.len() as u16;
    
    unsafe {
        BUTTON_REFRESH = Some(Rect { x: inner.x + 2, y: button_y, width: 14, height: 3 });
        BUTTON_CREATE = Some(Rect { x: inner.x + 18, y: button_y, width: 16, height: 3 });
        BUTTON_CLOSE = Some(Rect { x: inner.x + 36, y: button_y, width: 14, height: 3 });
    }
    
    // Buttons - symmetrisch
    lines.push(Line::from(Span::styled(
        " ┌────────────┐  ┌──────────────┐  ┌────────────┐".to_string(),
        Style::default().fg(colors::BLUE)
    )));
    lines.push(Line::from(Span::styled(
        " │[g] Refresh │  │[c] Create New│  │  [Esc] X   │".to_string(),
        Style::default().fg(colors::BLUE)
    )));
    lines.push(Line::from(Span::styled(
        " └────────────┘  └──────────────┘  └────────────┘".to_string(),
        Style::default().fg(colors::BLUE)
    )));
    
    frame.render_widget(Paragraph::new(lines), inner);
}

fn kl(k: &str, d: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!(" {:12}", k), Style::default().fg(colors::BLUE)),
        Span::styled(d.to_string(), Style::default().fg(colors::TEXT))
    ])
}

fn klc(k: &str, d: &str, c: ratatui::style::Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!(" {:12}", k), Style::default().fg(c)),
        Span::styled(d.to_string(), Style::default().fg(c))
    ])
}

fn po(k: &str, n: &str, d: &str, c: ratatui::style::Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  [{}] ", k), Style::default().fg(c)),
        Span::styled(format!("{:8}", n), Style::default().fg(c).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" - {}", d), Style::default().fg(colors::TEXT_DIM)),
    ])
}
