//! Multi-agent neural network visualization widget for TUI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::tui::theme::DraculaTheme;

/// Agent node in the neural network visualization
#[derive(Debug, Clone)]
pub struct AgentNode {
    pub name: String,
    pub role: String,
    pub is_active: bool,
    pub confidence: f32,
}

/// Connection between agents
#[derive(Debug, Clone)]
pub struct AgentConnection {
    pub from: String,
    pub to: String,
    pub weight: f32,
}

/// Multi-agent network state
#[derive(Debug, Clone, Default)]
pub struct MultiAgentNetwork {
    pub agents: Vec<AgentNode>,
    pub connections: Vec<AgentConnection>,
    pub current_round: usize,
    pub total_rounds: usize,
    pub consensus_reached: bool,
    pub problem: String,
}

impl MultiAgentNetwork {
    pub fn new(problem: &str) -> Self {
        Self {
            problem: problem.to_string(),
            agents: vec![
                AgentNode {
                    name: "analyst".to_string(),
                    role: "Logical Analysis".to_string(),
                    is_active: false,
                    confidence: 0.0,
                },
                AgentNode {
                    name: "critic".to_string(),
                    role: "Critical Review".to_string(),
                    is_active: false,
                    confidence: 0.0,
                },
                AgentNode {
                    name: "innovator".to_string(),
                    role: "Creative Solutions".to_string(),
                    is_active: false,
                    confidence: 0.0,
                },
                AgentNode {
                    name: "validator".to_string(),
                    role: "Solution Testing".to_string(),
                    is_active: false,
                    confidence: 0.0,
                },
            ],
            connections: vec![
                AgentConnection { from: "analyst".to_string(), to: "critic".to_string(), weight: 1.0 },
                AgentConnection { from: "analyst".to_string(), to: "innovator".to_string(), weight: 1.0 },
                AgentConnection { from: "critic".to_string(), to: "validator".to_string(), weight: 1.0 },
                AgentConnection { from: "innovator".to_string(), to: "validator".to_string(), weight: 1.0 },
            ],
            current_round: 0,
            total_rounds: 3,
            consensus_reached: false,
        }
    }

    pub fn activate_agent(&mut self, name: &str) {
        for agent in &mut self.agents {
            agent.is_active = agent.name == name;
        }
    }

    pub fn update_confidence(&mut self, name: &str, confidence: f32) {
        for agent in &mut self.agents {
            if agent.name == name {
                agent.confidence = confidence;
                break;
            }
        }
    }
}

/// Draw the multi-agent network visualization
pub fn draw_multi_agent_network(
    frame: &mut Frame,
    area: Rect,
    network: &MultiAgentNetwork,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Network diagram
            Constraint::Length(5),  // Status bar
        ])
        .split(area);

    // Draw header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("🧠 ", Style::default().fg(DraculaTheme::PURPLE)),
        Span::styled(
            format!("Multi-Agent Debate: {}", network.problem.chars().take(40).collect::<String>()),
            Style::default().fg(DraculaTheme::FG).add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(DraculaTheme::PURPLE)),
    )
    .wrap(Wrap { trim: true });
    frame.render_widget(header, chunks[0]);

    // Draw network diagram
    draw_network_diagram(frame, chunks[1], network);

    // Draw status bar
    let status = Paragraph::new(Line::from(vec![
        Span::styled("Round: ", Style::default().fg(DraculaTheme::COMMENT)),
        Span::styled(
            format!("{}/{}", network.current_round, network.total_rounds),
            Style::default().fg(DraculaTheme::CYAN),
        ),
        Span::raw(" | "),
        Span::styled("Consensus: ", Style::default().fg(DraculaTheme::COMMENT)),
        Span::styled(
            if network.consensus_reached { "✓ Reached" } else { "⟳ In Progress" },
            if network.consensus_reached {
                Style::default().fg(DraculaTheme::GREEN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(DraculaTheme::ORANGE)
            },
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(DraculaTheme::CYAN)),
    );
    frame.render_widget(status, chunks[2]);
}

/// Draw the network diagram with agents and connections
fn draw_network_diagram(frame: &mut Frame, area: Rect, network: &MultiAgentNetwork) {
    let mut lines = Vec::new();

    // Top row: Analyst and Critic
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("    "),
        render_agent_node(&network.agents[0]),  // Analyst
        Span::raw("         "),
        render_agent_node(&network.agents[1]),  // Critic
    ]));

    // Connection lines
    lines.push(Line::from(""));
    lines.push(Line::from("         ╲               ╱"));
    lines.push(Line::from("          ╲             ╱"));

    // Consensus node
    let consensus_style = if network.consensus_reached {
        Style::default().fg(DraculaTheme::GREEN).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(DraculaTheme::ORANGE)
    };
    lines.push(Line::from(vec![
        Span::raw("          "),
        Span::styled("┌─────────────────┐", consensus_style),
    ]));
    lines.push(Line::from(vec![
        Span::raw("          "),
        Span::styled("│   CONSENSUS     │", consensus_style),
    ]));
    lines.push(Line::from(vec![
        Span::raw("          "),
        Span::styled("└────────┬────────┘", consensus_style),
    ]));

    // Connection lines
    lines.push(Line::from(""));
    lines.push(Line::from("          ╱             ╲"));
    lines.push(Line::from("         ╱               ╲"));

    // Bottom row: Innovator and Validator
    lines.push(Line::from(vec![
        Span::raw("    "),
        render_agent_node(&network.agents[2]),  // Innovator
        Span::raw("        "),
        render_agent_node(&network.agents[3]),  // Validator
    ]));
    lines.push(Line::from(""));

    // Legend
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("● ", Style::default().fg(DraculaTheme::GREEN)),
        Span::styled("Active  ", Style::default().fg(DraculaTheme::FG)),
        Span::styled("○ ", Style::default().fg(DraculaTheme::COMMENT)),
        Span::styled("Inactive  ", Style::default().fg(DraculaTheme::FG)),
        Span::styled("█ ", Style::default().fg(DraculaTheme::PURPLE)),
        Span::styled("High Confidence", Style::default().fg(DraculaTheme::FG)),
    ]));

    let diagram = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Agent Network ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DraculaTheme::CYAN))
                .style(Style::default().bg(DraculaTheme::BG)),
        )
        .style(Style::default().fg(DraculaTheme::FG));

    frame.render_widget(diagram, area);
}

/// Render an agent node with appropriate styling
fn render_agent_node(agent: &AgentNode) -> Span {
    let (border_color, fill) = if agent.is_active {
        (DraculaTheme::GREEN, "█")
    } else {
        (DraculaTheme::COMMENT, "░")
    };

    let confidence_bar = get_confidence_bar(agent.confidence);

    Span::styled(
        format!(
            "┌{}─ {} ─{}┐\n│{} {:>8} {}│\n│{} {:>8} {}│\n└──────────────┘",
            "─".repeat(6 - agent.name.len() / 2),
            agent.name,
            "─".repeat(6 - agent.name.len() / 2),
            fill,
            "",
            fill,
            fill,
            confidence_bar,
            fill,
        ),
        Style::default().fg(border_color),
    )
}

/// Get a visual confidence indicator
fn get_confidence_bar(confidence: f32) -> String {
    let filled = (confidence * 5.0) as usize;
    let empty = 5 - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
