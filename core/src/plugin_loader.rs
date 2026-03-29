//! Plugin and skill loader for the agent system

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use tracing::info;

/// Agent configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub agent: AgentConfig,
    #[serde(default)]
    pub providers: ProvidersConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub instruction: String,
    #[serde(default)]
    pub max_turns: usize,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ProvidersConfig {
    #[serde(default)]
    pub openrouter: ProviderConfig,
    #[serde(default)]
    pub ollama: ProviderConfig,
    #[serde(default)]
    pub dashscope: ProviderConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ProviderConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub api_base: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

/// Discover and load skills from the skills directory
pub fn load_skills() -> Result<Vec<String>> {
    let skills_dir = Path::new("brain/skills");
    let mut skills = Vec::new();

    if !skills_dir.exists() {
        info!("Skills directory not found, skipping skill loading");
        return Ok(skills);
    }

    for entry in std::fs::read_dir(skills_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                skills.push(name.to_string());
                info!("📦 Discovered skill: {}", name);
            }
        }
    }

    info!("Loaded {} skills", skills.len());
    Ok(skills)
}

/// Discover and load plugins from the plugins directory
pub fn load_plugins() -> Result<Vec<String>> {
    let plugins_dir = Path::new("brain/plugins");
    let mut plugins = Vec::new();

    if !plugins_dir.exists() {
        info!("Plugins directory not found, skipping plugin loading");
        return Ok(plugins);
    }

    for entry in std::fs::read_dir(plugins_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                plugins.push(name.to_string());
                info!("🔌 Discovered plugin: {}", name);
            }
        }
    }

    info!("Loaded {} plugins", plugins.len());
    Ok(plugins)
}
