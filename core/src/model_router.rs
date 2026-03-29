//! Model router for OpenRouter with model rotation support

use anyhow::Result;
use serde::Deserialize;

/// Model configuration from models.toml
#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    pub defaults: DefaultsConfig,
    pub free_large_context: ContextConfig,
    pub modalities: ModalitiesConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultsConfig {
    pub api_base: String,
    pub default_model: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContextConfig {
    pub models: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModalitiesConfig {
    pub image_generation: ModalityConfig,
    pub video_generation: ModalityConfig,
    pub vision: ModalityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModalityConfig {
    pub models: Vec<String>,
}

/// Model Router for selecting models based on task type
pub struct ModelRouter {
    config: ModelConfig,
    current_model: String,
}

impl ModelRouter {
    /// Load model configuration from TOML
    pub fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ModelConfig = toml::from_str(&content)?;

        Ok(Self {
            config: config.clone(),
            current_model: config.defaults.default_model.clone(),
        })
    }

    /// Select the appropriate model for a given task
    pub fn select_for_task(&mut self, task: &str) -> &str {
        let task_lower = task.to_lowercase();

        // Image generation triggers
        if task_lower.contains("draw")
            || task_lower.contains("generate image")
            || task_lower.contains("picture")
            || task_lower.contains("flux")
        {
            self.current_model = self
                .config
                .modalities
                .image_generation
                .models
                .first()
                .unwrap_or(&self.config.defaults.default_model)
                .clone();
            return &self.current_model;
        }

        // Video generation triggers
        if task_lower.contains("video")
            || task_lower.contains("animate")
            || task_lower.contains("render video")
            || task_lower.contains("runway")
        {
            self.current_model = self
                .config
                .modalities
                .video_generation
                .models
                .first()
                .unwrap_or(&self.config.defaults.default_model)
                .clone();
            return &self.current_model;
        }

        // Vision/image analysis triggers
        if task_lower.contains("look at")
            || task_lower.contains("describe image")
            || task_lower.contains("what's in")
            || task_lower.contains("what is in")
        {
            self.current_model = self
                .config
                .modalities
                .vision
                .models
                .first()
                .unwrap_or(&self.config.defaults.default_model)
                .clone();
            return &self.current_model;
        }

        // Default: use free large context model
        self.current_model = self.config.defaults.default_model.clone();
        &self.current_model
    }

    /// Get the current model
    pub fn current_model(&self) -> &str {
        &self.current_model
    }

    /// Get all available models
    pub fn get_all_models(&self) -> Vec<String> {
        let mut models = Vec::new();
        models.push(self.config.defaults.default_model.clone());
        models.extend(self.config.free_large_context.models.clone());
        models.extend(self.config.modalities.image_generation.models.clone());
        models.extend(self.config.modalities.video_generation.models.clone());
        models.extend(self.config.modalities.vision.models.clone());
        models.sort();
        models.dedup();
        models
    }
}
