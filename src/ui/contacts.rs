//! Contact list rendering
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use ratatui::{
    Frame, layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::colors;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(colors::BG));
    
    frame.render_widget(block, area);
    
    let mut lines: Vec<Line> = Vec::new();
    let w = area.width as usize;
    let sep: String = "─".repeat(w);
    
    lines.push(Line::from(Span::styled(
        " Contacts",
        Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD)
    )));
    lines.push(Line::from(Span::styled(sep.clone(), Style::default().fg(colors::BORDER))));
    
    if app.contacts.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  No contacts", Style::default().fg(colors::TEXT_DIM))));
        lines.push(Line::from(Span::styled("  Press [i] to add", Style::default().fg(colors::TEXT_DIM))));
        lines.push(Line::from(Span::styled("  Double-click for options", Style::default().fg(colors::TEXT_DIM))));
    } else {
        for (i, contact) in app.contacts.iter().enumerate() {
            let selected = app.contact_state.selected() == Some(i);
            let bg = if selected { colors::BG_HIGHLIGHT } else { colors::BG };
            
            let marker = if selected { "▸ " } else { "  " };
            let marker_color = if selected { colors::BLUE } else { colors::TEXT_DIM };
            
            let initials = colors::get_initials(&contact.name);
            let avatar_color = colors::avatar_color(&contact.name);
            
            let name_style = if selected {
                Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(colors::TEXT)
            };
            
            let mut spans = vec![
                Span::styled(marker.to_string(), Style::default().fg(marker_color).bg(bg)),
                Span::styled("[".to_string(), Style::default().fg(avatar_color).bg(bg)),
                Span::styled(initials.clone(), Style::default().fg(avatar_color).add_modifier(Modifier::BOLD).bg(bg)),
                Span::styled("] ".to_string(), Style::default().fg(avatar_color).bg(bg)),
                Span::styled(contact.name.clone(), name_style.bg(bg)),
            ];
            
            let unread_text = if contact.unread > 0 {
                format!(" ({})", contact.unread)
            } else {
                String::new()
            };
            
            if contact.unread > 0 {
                spans.push(Span::styled(
                    unread_text.clone(),
                    Style::default().fg(colors::WARNING).add_modifier(Modifier::BOLD).bg(bg)
                ));
            }
            
            let used_len = marker.len() + 1 + initials.len() + 2 + contact.name.len() + unread_text.len();
            let remaining = w.saturating_sub(used_len);
            spans.push(Span::styled(" ".repeat(remaining), Style::default().bg(bg)));
            
            lines.push(Line::from(spans));
            lines.push(Line::from(Span::styled(sep.clone(), Style::default().fg(colors::BORDER))));
        }
    }
    
    frame.render_widget(Paragraph::new(lines), area);
}

pub fn render_actions(frame: &mut Frame, area: Rect) {
    if area.height < 2 { return; }
    
    let w = area.width as usize;
    let sep: String = "─".repeat(w);
    
    let content = "[i]Add  [o]Opts  [r]Rfsh  [?]Help";
    let pad = w.saturating_sub(content.len()) / 2;
    let padding: String = " ".repeat(pad);
    
    let lines = vec![
        Line::from(Span::styled(sep, Style::default().fg(colors::BORDER))),
        Line::from(vec![
            Span::styled(padding, Style::default()),
            Span::styled("[", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("i", Style::default().fg(colors::BLUE)),
            Span::styled("]Add  ", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("[", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("o", Style::default().fg(colors::BLUE)),
            Span::styled("]Opts  ", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("[", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("r", Style::default().fg(colors::BLUE)),
            Span::styled("]Rfsh  ", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("[", Style::default().fg(colors::TEXT_DIM)),
            Span::styled("?", Style::default().fg(colors::BLUE)),
            Span::styled("]Help", Style::default().fg(colors::TEXT_DIM)),
        ]),
    ];
    
    frame.render_widget(
        Paragraph::new(lines).style(Style::default().bg(colors::BG_SECONDARY)),
        area
    );
}
