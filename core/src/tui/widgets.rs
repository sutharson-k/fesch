//! TUI widgets for the Dracula theme

use crate::tui::theme::DraculaTheme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Draw the header bar
pub fn draw_header(frame: &mut Frame, area: Rect, _theme: &DraculaTheme, current_model: &str) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled("🤖 FESCH AGENT", Style::default().fg(DraculaTheme::PURPLE).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(current_model, Style::default().fg(DraculaTheme::CYAN)),
    ]))
    .block(
        Block::default()
            .title(" AGENT ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(DraculaTheme::PURPLE))
            .style(Style::default().bg(DraculaTheme::BG)),
    )
    .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(header, area);
}

/// Draw the chat panel
pub fn draw_chat(frame: &mut Frame, area: Rect, _theme: &DraculaTheme, history: &[(String, String)]) {
    let mut lines = Vec::new();

    for (role, message) in history {
        let style = match role.as_str() {
            "user" => Style::default().fg(DraculaTheme::CYAN),
            "agent" => Style::default().fg(DraculaTheme::PINK),
            _ => Style::default().fg(DraculaTheme::FG),
        };

        let prefix = match role.as_str() {
            "user" => "> ",
            "agent" => "🤖 ",
            _ => "",
        };

        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, message),
            style,
        )));
    }

    let chat = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Chat ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DraculaTheme::CYAN))
                .style(Style::default().bg(DraculaTheme::BG)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(chat, area);
}

/// Draw the tools panel
pub fn draw_tools(frame: &mut Frame, area: Rect, _theme: &DraculaTheme) {
    let tools = vec![
        ListItem::new(Line::from(vec![
            Span::styled("✔ ", Style::default().fg(DraculaTheme::GREEN)),
            Span::styled("search_web", Style::default().fg(DraculaTheme::CYAN)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("✔ ", Style::default().fg(DraculaTheme::GREEN)),
            Span::styled("browse", Style::default().fg(DraculaTheme::CYAN)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("⚙ ", Style::default().fg(DraculaTheme::ORANGE)),
            Span::styled("launch", Style::default().fg(DraculaTheme::CYAN)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("○ ", Style::default().fg(DraculaTheme::COMMENT)),
            Span::styled("schedule_task", Style::default().fg(DraculaTheme::CYAN)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("○ ", Style::default().fg(DraculaTheme::COMMENT)),
            Span::styled("screenshot", Style::default().fg(DraculaTheme::CYAN)),
        ])),
    ];

    let tools_list = List::new(tools)
        .block(
            Block::default()
                .title(" Tools ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DraculaTheme::GREEN))
                .style(Style::default().bg(DraculaTheme::BG)),
        )
        .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(tools_list, area);
}

/// Draw the memory panel
pub fn draw_memory(frame: &mut Frame, area: Rect, _theme: &DraculaTheme) {
    let memory_info = vec![
        Line::from(vec![
            Span::styled("Session: ", Style::default().fg(DraculaTheme::PURPLE)),
            Span::styled("default-session", Style::default().fg(DraculaTheme::FG)),
        ]),
        Line::from(vec![
            Span::styled("Turns: ", Style::default().fg(DraculaTheme::PURPLE)),
            Span::styled("0 / ∞", Style::default().fg(DraculaTheme::FG)),
        ]),
        Line::from(vec![
            Span::styled("Tool calls: ", Style::default().fg(DraculaTheme::PURPLE)),
            Span::styled("0", Style::default().fg(DraculaTheme::FG)),
        ]),
    ];

    let memory = Paragraph::new(memory_info)
        .block(
            Block::default()
                .title(" ADK Session Memory ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DraculaTheme::PURPLE))
                .style(Style::default().bg(DraculaTheme::BG)),
        )
        .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(memory, area);
}

/// Draw the log panel
pub fn draw_log(frame: &mut Frame, area: Rect, _theme: &DraculaTheme) {
    let logs = vec![
        ListItem::new(Line::from(vec![
            Span::styled("[INFO] ", Style::default().fg(DraculaTheme::COMMENT)),
            Span::styled("Agent initialized", Style::default().fg(DraculaTheme::FG)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("[INFO] ", Style::default().fg(DraculaTheme::GREEN)),
            Span::styled("Model → gemini-2.0-flash-exp:free", Style::default().fg(DraculaTheme::FG)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("[INFO] ", Style::default().fg(DraculaTheme::ORANGE)),
            Span::styled("Plugins loaded", Style::default().fg(DraculaTheme::FG)),
        ])),
    ];

    let log_list = List::new(logs)
        .block(
            Block::default()
                .title(" Log ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DraculaTheme::COMMENT))
                .style(Style::default().bg(DraculaTheme::BG)),
        )
        .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(log_list, area);
}

/// Draw the input box
pub fn draw_input(frame: &mut Frame, area: Rect, _theme: &DraculaTheme, input: &str) {
    let input_widget = Paragraph::new(Line::from(vec![
        Span::styled("> ", Style::default().fg(DraculaTheme::PINK)),
        Span::styled(input, Style::default().fg(DraculaTheme::FG)),
    ]))
    .block(
        Block::default()
            .title(" Input (q to quit) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(DraculaTheme::SURFACE))
            .style(Style::default().bg(DraculaTheme::SURFACE)),
    )
    .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(input_widget, area);
}
