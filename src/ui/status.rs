//! Status bar rendering
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use ratatui::{Frame, layout::Rect, style::Style, text::{Line, Span}, widgets::Paragraph};
use crate::app::App;
use crate::colors;

const FOOTER_TEXTS: [&str; 3] = [
    "v0.1.2-alpha",
    "i(N) cod(E) w(E) trus(T)",
    "(c) by cannatoshi",
];

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let w = area.width as usize;
    
    let (dot, dc) = if app.connected { ("●", colors::BLUE) } else { ("○", colors::WARNING) };
    let (conn_txt, tc) = if app.connected { ("Connected", colors::BLUE) } else { ("Offline", colors::WARNING) };
    
    let footer_text = get_footer_text(app.tick);
    
    let left = format!(" {} {} │ {}", dot, conn_txt, app.status);
    let left_len = 3 + conn_txt.len() + 3 + app.status.len();
    
    let right_len = footer_text.len() + 2;
    
    let space = w.saturating_sub(left_len + right_len);
    
    let line = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(dot, Style::default().fg(dc)),
        Span::styled(" ", Style::default()),
        Span::styled(conn_txt, Style::default().fg(tc)),
        Span::styled(" │ ", Style::default().fg(colors::BORDER)),
        Span::styled(&app.status, Style::default().fg(colors::TEXT_MUTED)),
        Span::styled(" ".repeat(space), Style::default()),
        Span::styled(footer_text, Style::default().fg(colors::TEXT_DIM)),
        Span::styled(" ", Style::default()),
    ]);
    
    frame.render_widget(Paragraph::new(line).style(Style::default().bg(colors::BG_SECONDARY)), area);
}

fn get_footer_text(tick: u64) -> String {
    let ticks_per_phase = 200;
    let total_cycle = ticks_per_phase * FOOTER_TEXTS.len() as u64;
    
    let position = tick % total_cycle;
    let current_index = (position / ticks_per_phase) as usize;
    let phase_tick = position % ticks_per_phase;
    
    let text = FOOTER_TEXTS[current_index];
    let text_len = text.chars().count();
    
    if phase_tick < 40 {
        let chars_to_show = ((phase_tick as usize) * text_len / 40).min(text_len);
        text.chars().take(chars_to_show).collect()
    } else if phase_tick < 160 {
        text.to_string()
    } else {
        let elapsed = phase_tick - 160;
        let chars_to_hide = ((elapsed as usize) * text_len / 40).min(text_len);
        let chars_to_show = text_len.saturating_sub(chars_to_hide);
        text.chars().take(chars_to_show).collect()
    }
}
