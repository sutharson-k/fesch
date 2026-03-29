//! Dracula theme color palette

use ratatui::style::Color;

/// Dracula color scheme
#[derive(Debug, Clone, Default)]
pub struct DraculaTheme;

impl DraculaTheme {
    pub fn new() -> Self {
        Self
    }
    pub const BG: Color = Color::Rgb(0x28, 0x2a, 0x36);      // #282a36 - Background
    pub const SURFACE: Color = Color::Rgb(0x44, 0x47, 0x5a); // #44475a - Surface
    pub const COMMENT: Color = Color::Rgb(0x62, 0x72, 0xa4); // #6272a4 - Comment
    pub const PINK: Color = Color::Rgb(0xff, 0x79, 0xc6);    // #ff79c6 - Pink
    pub const PURPLE: Color = Color::Rgb(0xbd, 0x93, 0xf9);  // #bd93f9 - Purple
    pub const CYAN: Color = Color::Rgb(0x8b, 0xe9, 0xfd);    // #8be9fd - Cyan
    pub const GREEN: Color = Color::Rgb(0x50, 0xfa, 0x7b);   // #50fa7b - Green
    pub const ORANGE: Color = Color::Rgb(0xff, 0xb8, 0x6c);  // #ffb86c - Orange
    pub const FG: Color = Color::Rgb(0xf8, 0xf8, 0xf2);      // #f8f8f2 - Foreground
    pub const RED: Color = Color::Rgb(0xff, 0x55, 0x55);     // #ff5555 - Red
    pub const YELLOW: Color = Color::Rgb(0xf1, 0xfa, 0x8c);  // #f1fa8c - Yellow

    /// Get color semantics for different UI elements
    pub fn agent_response() -> Color {
        Self::PINK
    }

    pub fn user_input() -> Color {
        Self::CYAN
    }

    pub fn header() -> Color {
        Self::PURPLE
    }

    pub fn success() -> Color {
        Self::GREEN
    }

    pub fn warning() -> Color {
        Self::ORANGE
    }

    pub fn error() -> Color {
        Self::RED
    }

    pub fn secondary() -> Color {
        Self::COMMENT
    }

    pub fn surface() -> Color {
        Self::SURFACE
    }

    pub fn background() -> Color {
        Self::BG
    }

    pub fn foreground() -> Color {
        Self::FG
    }
}
