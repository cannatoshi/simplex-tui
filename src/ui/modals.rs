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
use crate::types::ContactOption;

pub static mut BUTTON_REFRESH: Option<Rect> = None;
pub static mut BUTTON_CREATE: Option<Rect> = None;
pub static mut BUTTON_CLOSE: Option<Rect> = None;
pub static mut CONTACT_OPTION_BUTTONS: [Option<Rect>; 4] = [None; 4];

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
        kl("o", "Contact options (or double-click)"),
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
    
    let button_y = inner.y + lines.len() as u16;
    
    unsafe {
        BUTTON_REFRESH = Some(Rect { x: inner.x + 2, y: button_y, width: 14, height: 3 });
        BUTTON_CREATE = Some(Rect { x: inner.x + 18, y: button_y, width: 16, height: 3 });
        BUTTON_CLOSE = Some(Rect { x: inner.x + 36, y: button_y, width: 14, height: 3 });
    }
    
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

pub fn render_contact_options(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(Clear, area);
    
    let contact_name = app.contact_for_options.as_deref().unwrap_or("Unknown");
    let title = format!(" Contact: {} ", contact_name);
    
    let avatar_color = colors::avatar_color(contact_name);
    
    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(avatar_color).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(avatar_color))
        .style(Style::default().bg(colors::BG));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(" Contact Options", Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    let initials = colors::get_initials(contact_name);
    lines.push(Line::from(vec![
        Span::styled("    [", Style::default().fg(avatar_color)),
        Span::styled(&initials, Style::default().fg(avatar_color).add_modifier(Modifier::BOLD)),
        Span::styled("] ", Style::default().fg(avatar_color)),
        Span::styled(contact_name, Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    
    let sep: String = "─".repeat((inner.width as usize).saturating_sub(2));
    lines.push(Line::from(Span::styled(format!(" {}", sep), Style::default().fg(colors::BORDER))));
    lines.push(Line::from(""));
    
    if let Some(action) = &app.confirm_action {
        let warning_color = if (app.tick / 4) % 2 == 0 { colors::DANGER } else { colors::WARNING };
        lines.push(Line::from(Span::styled(
            format!(" ⚠ Confirm: {} ?", action.label()),
            Style::default().fg(warning_color).add_modifier(Modifier::BOLD)
        )));
        lines.push(Line::from(Span::styled(
            " Press [Enter] again to confirm, [Esc] to cancel",
            Style::default().fg(colors::TEXT_DIM)
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(format!(" {}", sep), Style::default().fg(colors::BORDER))));
        lines.push(Line::from(""));
    }
    
    let button_start_y = inner.y + lines.len() as u16;
    
    let options = ContactOption::all();
    for (i, option) in options.iter().enumerate() {
        let is_selected = i == app.option_selection;
        let is_destructive = option.is_destructive();
        
        let btn_y = button_start_y + (i as u16 * 3);
        unsafe {
            CONTACT_OPTION_BUTTONS[i] = Some(Rect { 
                x: inner.x + 2, 
                y: btn_y, 
                width: 40, 
                height: 3 
            });
        }
        
        let (border_color, text_color, marker) = if is_selected {
            if is_destructive {
                (colors::DANGER, colors::DANGER, "▸")
            } else {
                (colors::BLUE, colors::BLUE, "▸")
            }
        } else {
            if is_destructive {
                (colors::TEXT_DIM, colors::DANGER, " ")
            } else {
                (colors::TEXT_DIM, colors::TEXT, " ")
            }
        };
        
        let key_char = option.key();
        let label = option.label();
        
        lines.push(Line::from(vec![
            Span::styled(format!("  {}┌", marker), Style::default().fg(border_color)),
            Span::styled("─".repeat(36), Style::default().fg(border_color)),
            Span::styled("┐", Style::default().fg(border_color)),
        ]));
        
        let content = format!("[{}] {}", key_char, label);
        let padding = 36 - content.len();
        lines.push(Line::from(vec![
            Span::styled("   │ ", Style::default().fg(border_color)),
            Span::styled(format!("[{}]", key_char), Style::default().fg(border_color)),
            Span::styled(format!(" {}", label), Style::default().fg(text_color)),
            Span::styled(" ".repeat(padding.saturating_sub(1)), Style::default()),
            Span::styled("│", Style::default().fg(border_color)),
        ]));
        
        lines.push(Line::from(vec![
            Span::styled("   └", Style::default().fg(border_color)),
            Span::styled("─".repeat(36), Style::default().fg(border_color)),
            Span::styled("┘", Style::default().fg(border_color)),
        ]));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Use ↑↓ or j/k to navigate, Enter to select",
        Style::default().fg(colors::TEXT_DIM)
    )));
    lines.push(Line::from(Span::styled(
        " Or press the key in brackets for quick access",
        Style::default().fg(colors::TEXT_DIM)
    )));
    lines.push(Line::from(Span::styled(
        " Double-click opens this menu",
        Style::default().fg(colors::TEXT_DIM)
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
