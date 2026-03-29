//! Task scheduler with cron support and IPC to Python ADK

use anyhow::Result;
use serde::Deserialize;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{info, warn};

/// Task definition from schedule.toml
#[derive(Debug, Deserialize, Clone)]
pub struct ScheduledTask {
    pub name: String,
    pub cron: String,
    pub action: String,
    #[serde(default)]
    pub args: serde_json::Value,
    #[serde(default)]
    pub enabled: bool,
}

/// Task Scheduler wrapper
pub struct TaskScheduler {
    scheduler: JobScheduler,
    tasks: Vec<ScheduledTask>,
}

impl TaskScheduler {
    pub async fn new() -> Self {
        let scheduler = JobScheduler::new().await.expect("Failed to create job scheduler");
        Self {
            scheduler,
            tasks: Vec::new(),
        }
    }

    /// Load tasks from TOML config
    pub async fn load_from_config(&mut self, path: &str) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let config: toml::Value = toml::from_str(&content)?;

        if let Some(tasks) = config.get("tasks").and_then(|t| t.as_array()) {
            for task_value in tasks {
                let task: ScheduledTask = task_value.clone().try_into()?;
                if task.enabled {
                    self.register_task(&task).await?;
                    self.tasks.push(task);
                }
            }
        }

        info!("Loaded {} scheduled tasks", self.tasks.len());
        Ok(())
    }

    /// Register a single task with the scheduler
    async fn register_task(&mut self, task: &ScheduledTask) -> Result<()> {
        let cron = task.cron.clone();
        let action = task.action.clone();
        let args = task.args.clone();
        let name = task.name.clone();

        let job = Job::new_async(cron.as_str(), move |uuid, mut l| {
            let action = action.clone();
            let args = args.clone();
            let name = name.clone();

            Box::pin(async move {
                info!("📅 Running scheduled task: {}", name);
                
                // Send IPC to Python ADK brain
                // For now, just log - actual implementation would call Python
                info!("🔧 Executing action: {} with args: {:?}", action, args);
                
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next tick for job {}: {:?}", uuid, ts),
                    Ok(None) => warn!("No next tick for job {}", uuid),
                    Err(e) => warn!("Error getting next tick: {}", e),
                }
            })
        })?;

        self.scheduler.add(job).await?;
        info!("Registered task: {} (cron: {})", task.name, task.cron);
        Ok(())
    }

    /// Start the scheduler
    pub async fn start(&self) -> Result<()> {
        info!("⏰ Starting task scheduler...");
        self.scheduler.start().await?;
        Ok(())
    }

    /// Stop the scheduler
    pub async fn stop(&mut self) -> Result<()> {
        info!("⏰ Stopping task scheduler...");
        self.scheduler.shutdown().await?;
        Ok(())
    }

    /// Get all tasks
    pub fn get_tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }
}
