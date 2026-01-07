//! Input field rendering
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
use crate::types::Mode;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let active = app.mode == Mode::Input;
    let bc = if active { colors::BLUE } else { colors::BORDER };
    
    let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(bc)).style(Style::default().bg(colors::BG_SECONDARY));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let btn_w = 4;
    let inp_w = inner.width.saturating_sub(btn_w);
    
    let content = if app.input.is_empty() && !active {
        Line::from(Span::styled(" Type a message...", Style::default().fg(colors::TEXT_DIM)))
    } else {
        let cursor_on = active && (app.tick / 5) % 2 == 0;
        let before: String = app.input.chars().take(app.cursor).collect();
        let after: String = app.input.chars().skip(app.cursor).collect();
        let mut spans = vec![Span::styled(" ", Style::default()), Span::styled(before, Style::default().fg(colors::TEXT))];
        if cursor_on { spans.push(Span::styled("▌", Style::default().fg(colors::BLUE).add_modifier(Modifier::BOLD))); }
        spans.push(Span::styled(after, Style::default().fg(colors::TEXT)));
        Line::from(spans)
    };
    
    frame.render_widget(Paragraph::new(content), Rect { x: inner.x, y: inner.y, width: inp_w, height: inner.height });
    
    let sc = if !app.input.is_empty() { colors::BLUE } else { colors::TEXT_DIM };
    frame.render_widget(Paragraph::new(Line::from(Span::styled(" ➤ ", Style::default().fg(sc).add_modifier(Modifier::BOLD)))), Rect { x: inner.x + inp_w, y: inner.y, width: btn_w, height: inner.height });
}
