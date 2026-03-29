//! TUI module for the Dracula-themed terminal interface

mod theme;
mod layout;
mod widgets;
pub mod multi_agent;

pub use theme::DraculaTheme;
pub use layout::TuiLayout;

use crate::model_router::ModelRouter;
use crate::scheduler::TaskScheduler;
use crate::plugin_loader::Config;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::info;

/// Main TUI application
pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    theme: DraculaTheme,
    layout: TuiLayout,
    input_buffer: String,
    chat_history: Vec<(String, String)>,  // (role, message)
    running: bool,
}

impl Tui {
    /// Create a new TUI instance
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            theme: DraculaTheme::new(),
            layout: TuiLayout::default(),
            input_buffer: String::new(),
            chat_history: Vec::new(),
            running: false,
        })
    }

    /// Initialize the terminal
    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.running = true;
        info!("TUI initialized");
        Ok(())
    }

    /// Run the main event loop
    pub async fn run(
        &mut self,
        config: &Config,
        scheduler: &mut TaskScheduler,
        model_router: &ModelRouter,
    ) -> Result<()> {
        // Start the scheduler
        scheduler.start().await?;

        // Welcome message
        self.chat_history.push((
            "agent".to_string(),
            format!("🤖 Welcome to {} v0.1.0 | Model: {}", 
                config.agent.name, 
                model_router.current_model())
        ));

        while self.running {
            self.terminal.draw(|f| {
                let chunks = self.layout.split(f.size());
                
                // Draw header
                widgets::draw_header(f, chunks[0], &self.theme, model_router.current_model());
                
                // Draw chat panel
                widgets::draw_chat(f, chunks[1], &self.theme, &self.chat_history);
                
                // Draw tools panel
                widgets::draw_tools(f, chunks[2], &self.theme);
                
                // Draw memory panel
                widgets::draw_memory(f, chunks[3], &self.theme);
                
                // Draw log panel
                widgets::draw_log(f, chunks[4], &self.theme);
                
                // Draw input box
                widgets::draw_input(f, chunks[5], &self.theme, &self.input_buffer);
            })?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key)?;
                }
            }
        }

        // Stop the scheduler
        scheduler.stop().await?;

        Ok(())
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Enter => {
                if !self.input_buffer.is_empty() {
                    // Process the input
                    let input = self.input_buffer.clone();
                    self.chat_history.push(("user".to_string(), input.clone()));
                    
                    // Call ADK brain (simplified - actual implementation would be async)
                    // For now, just echo
                    self.chat_history.push((
                        "agent".to_string(),
                        format!("Processing: {}", input)
                    ));
                    
                    self.input_buffer.clear();
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    /// Cleanup the terminal
    pub fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        info!("TUI cleaned up");
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
