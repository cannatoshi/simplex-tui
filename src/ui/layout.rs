//! Main layout rendering
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::colors;
use crate::types::Mode;

use super::{contacts, chat, input, status, modals};

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BORDER_ACTIVE))
        .style(Style::default().bg(colors::BG));
    
    let inner = outer.inner(area);
    frame.render_widget(outer, area);
    
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Min(1)])
        .split(inner);
    
    render_left(frame, cols[0], app);
    render_right(frame, cols[1], app);
    draw_divider(frame, area, 33);
    
    if app.show_help { modals::render_help(frame); }
    if app.mode == Mode::Panic { modals::render_panic(frame, app); }
    if app.mode == Mode::AddContact { modals::render_add_contact(frame, app); }
    if app.mode == Mode::ContactOptions { modals::render_contact_options(frame, app); }
    if app.mode == Mode::ContactInfo { modals::render_contact_info(frame, app); }
}

fn render_left(frame: &mut Frame, area: Rect, app: &mut App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    
    contacts::render(frame, rows[0], app);
    contacts::render_actions(frame, rows[1]);
}

fn render_right(frame: &mut Frame, area: Rect, app: &mut App) {
    let chat_area = Rect { x: area.x + 1, y: area.y, width: area.width.saturating_sub(1), height: area.height };
    
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(chat_area);
    
    app.last_input_y = Some(rows[2].y);
    
    chat::render_header(frame, rows[0], app);
    chat::render_messages(frame, rows[1], app);
    input::render(frame, rows[2], app);
    
    let sep = "─".repeat(rows[3].width as usize);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(sep, Style::default().fg(colors::BORDER))))
            .style(Style::default().bg(colors::BG_SECONDARY)),
        rows[3]
    );
    
    status::render(frame, rows[4], app);
}

fn draw_divider(frame: &mut Frame, area: Rect, x_off: u16) {
    let x = area.x + x_off;
    let style = Style::default().fg(colors::BORDER_ACTIVE);
    
    frame.render_widget(Paragraph::new(Line::from(Span::styled("┬", style))), Rect { x, y: area.y, width: 1, height: 1 });
    for row in 1..area.height.saturating_sub(1) {
        frame.render_widget(Paragraph::new(Line::from(Span::styled("│", style))), Rect { x, y: area.y + row, width: 1, height: 1 });
    }
    frame.render_widget(Paragraph::new(Line::from(Span::styled("┴", style))), Rect { x, y: area.y + area.height - 1, width: 1, height: 1 });
}
