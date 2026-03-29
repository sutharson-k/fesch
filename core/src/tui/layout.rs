//! TUI layout management

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
};

/// TUI layout structure
#[derive(Debug, Clone, Default)]
pub struct TuiLayout;

impl TuiLayout {
    /// Split the screen into panels
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        // Main vertical split: top (chat+tools) and bottom (memory+log+input)
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(area);

        // Top horizontal split: chat and tools
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(main_chunks[0]);

        // Bottom horizontal split: memory and log
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[1]);

        // Split bottom further for input
        let bottom_with_input = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(main_chunks[1]);

        // Return all panels in order:
        // [0] Chat, [1] Tools, [2] Memory, [3] Log, [4] (unused), [5] Input
        vec![
            top_chunks[0],      // Chat panel
            top_chunks[1],      // Tools panel
            bottom_with_input[0], // Memory panel (left half of bottom)
            bottom_chunks[1],   // Log panel (right half of bottom)
            Rect::default(),    // Placeholder
            bottom_with_input[1], // Input box
        ]
    }
}
