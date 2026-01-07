//! Chat message rendering
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use ratatui::{
    Frame, layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use crate::colors;
use crate::types::MessageStatus;

pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let w = area.width as usize;
    let title = "SimpleX TUI (cyberdeck)";
    
    let header = if let Some(name) = &app.current_contact {
        let initials = colors::get_initials(name);
        let color = colors::avatar_color(name);
        
        let left_len = 1 + 1 + initials.len() + 2 + name.len();
        let right_len = title.len() + 1;
        let space = w.saturating_sub(left_len + right_len);
        
        Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled("[", Style::default().fg(color)),
            Span::styled(initials, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled("] ", Style::default().fg(color)),
            Span::styled(name, Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD)),
            Span::styled(" ".repeat(space), Style::default()),
            Span::styled(title, Style::default().fg(colors::BLUE)),
            Span::styled(" ", Style::default()),
        ])
    } else {
        let left_text = "No chat selected";
        let left_len = left_text.len() + 2;
        let right_len = title.len() + 1;
        let space = w.saturating_sub(left_len + right_len);
        
        Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(left_text, Style::default().fg(colors::TEXT_DIM)),
            Span::styled(" ".repeat(space), Style::default()),
            Span::styled(title, Style::default().fg(colors::BLUE)),
            Span::styled(" ", Style::default()),
        ])
    };
    
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(colors::BORDER))
        .style(Style::default().bg(colors::BG_SECONDARY));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(Paragraph::new(header), inner);
}

pub fn render_messages(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(colors::BG));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    if app.messages.is_empty() {
        let msg = if app.current_contact.is_some() { 
            "No messages yet. Say hello!" 
        } else { 
            "← Select a contact to start chatting" 
        };
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(format!("  {}", msg), Style::default().fg(colors::TEXT_DIM)))),
            inner
        );
        return;
    }
    
    let mut lines: Vec<Line> = Vec::new();
    
    for msg in &app.messages {
        let sender_color = if msg.mine { colors::BLUE } else { colors::TEXT };
        
        let mut header_spans = vec![
            Span::styled(" ", Style::default()),
            Span::styled(&msg.sender, Style::default().fg(sender_color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("  {}", msg.time), Style::default().fg(colors::TEXT_DIM)),
        ];
        
        if msg.mine {
            let (icon, color) = match msg.status {
                MessageStatus::Sending => ("  ○", colors::TEXT_DIM),
                MessageStatus::Sent => ("  ✓", colors::BLUE),
                MessageStatus::Delivered => ("  ✓✓", colors::BLUE),
                MessageStatus::Read => ("  ✓✓", colors::BLUE_LIGHT),
                MessageStatus::Failed => ("  ✗", colors::DANGER),
            };
            header_spans.push(Span::styled(icon, Style::default().fg(color)));
        }
        
        lines.push(Line::from(header_spans));
        lines.push(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(&msg.content, Style::default().fg(colors::TEXT)),
        ]));
        lines.push(Line::from(""));
    }
    
    let total = lines.len() as u16;
    let vis = inner.height;
    let scroll = if app.scroll == usize::MAX {
        if total > vis { total - vis } else { 0 }
    } else {
        (app.scroll as u16).min(total.saturating_sub(vis))
    };
    
    if app.scroll == usize::MAX { app.scroll = scroll as usize; }
    
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((scroll, 0)), inner);
}
