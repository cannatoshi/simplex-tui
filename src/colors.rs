//! Color palette for SimpleX TUI
//! 
//! Copyright (c) 2026 cannatoshi
//! GitHub: https://github.com/cannatoshi/simplex-tui
//! Licensed under AGPL-3.0

use ratatui::style::Color;

pub const BLUE: Color = Color::Rgb(0, 136, 255);
pub const BLUE_LIGHT: Color = Color::Rgb(100, 180, 255);

pub const BG: Color = Color::Rgb(12, 12, 16);
pub const BG_SECONDARY: Color = Color::Rgb(18, 18, 24);
pub const BG_HIGHLIGHT: Color = Color::Rgb(35, 45, 60);

pub const TEXT: Color = Color::Rgb(195, 200, 210);
pub const TEXT_MUTED: Color = Color::Rgb(100, 105, 115);
pub const TEXT_DIM: Color = Color::Rgb(70, 75, 85);

pub const SUCCESS: Color = Color::Rgb(80, 200, 120);
pub const WARNING: Color = Color::Rgb(255, 180, 0);
pub const DANGER: Color = Color::Rgb(220, 60, 60);
pub const UNREAD: Color = Color::Rgb(255, 150, 0);

pub const AVATAR_COLORS: [Color; 8] = [
    Color::Rgb(99, 102, 241),
    Color::Rgb(236, 72, 153),
    Color::Rgb(34, 197, 94),
    Color::Rgb(249, 115, 22),
    Color::Rgb(168, 85, 247),
    Color::Rgb(14, 165, 233),
    Color::Rgb(234, 179, 8),
    Color::Rgb(239, 68, 68),
];

pub const BORDER: Color = Color::Rgb(45, 50, 60);
pub const BORDER_ACTIVE: Color = BLUE;

pub fn avatar_color(name: &str) -> Color {
    let hash: usize = name.bytes().map(|b| b as usize).sum();
    AVATAR_COLORS[hash % AVATAR_COLORS.len()]
}

pub fn get_initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    match words.len() {
        0 => "?".to_string(),
        1 => words[0].chars().take(2).collect::<String>().to_uppercase(),
        _ => format!(
            "{}{}",
            words[0].chars().next().unwrap_or('?'),
            words[1].chars().next().unwrap_or('?')
        ).to_uppercase(),
    }
}
