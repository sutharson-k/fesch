mod engine;
mod scheduler;
mod memory;
mod model_router;
mod plugin_loader;
mod tui;

use anyhow::Result;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("🤖 Agent Core starting...");

    // Initialize the TUI
    let mut tui = tui::Tui::new()?;
    tui.init()?;

    tracing::info!("🎨 TUI initialized with Dracula theme");

    // Load configuration
    let config = plugin_loader::Config::load("config/agent.toml")?;
    tracing::info!("📄 Configuration loaded");

    // Initialize scheduler
    let mut scheduler = scheduler::TaskScheduler::new().await;
    scheduler.load_from_config("config/schedule.toml").await?;
    tracing::info!("⏰ Task scheduler initialized");

    // Initialize model router
    let model_router = model_router::ModelRouter::load("config/models.toml")?;
    tracing::info!("🔄 Model router configured");

    // Run the main event loop
    tui.run(&config, &mut scheduler, &model_router).await?;

    tracing::info!("👋 Agent Core shutting down");
    Ok(())
}
