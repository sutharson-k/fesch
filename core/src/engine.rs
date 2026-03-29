//! Core engine for task scheduling and event loop
//! Bridges Rust core with Python ADK brain via PyO3

use anyhow::Result;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

/// Event types for the agent system
#[derive(Debug, Clone)]
pub enum AgentEvent {
    UserMessage { session_id: String, message: String },
    ToolCall { tool_name: String, args: serde_json::Value },
    ToolResult { tool_name: String, result: String },
    ScheduledTask { task_name: String },
    ModelRotation { old_model: String, new_model: String },
}

/// Shared state for the agent engine
#[derive(Debug, Clone)]
pub struct EngineState {
    pub session_id: String,
    pub user_id: String,
    pub current_model: String,
    pub event_history: Vec<AgentEvent>,
}

impl Default for EngineState {
    fn default() -> Self {
        Self {
            session_id: format!("session-{}", uuid_simple()),
            user_id: "default-user".to_string(),
            current_model: "google/gemini-2.0-flash-exp:free".to_string(),
            event_history: Vec::new(),
        }
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_millis())
}

/// Call the Python ADK brain
pub fn call_adk_brain(user_id: &str, session_id: &str, message: &str) -> Result<String> {
    Python::with_gil(|py| {
        // Import the brain module
        let brain = py.import("brain.agent")?;
        
        // Call chat_sync function
        let result: String = brain
            .call_method1("chat_sync", (user_id, session_id, message))?
            .extract()?;
        
        Ok(result)
    })
    .map_err(|e: pyo3::PyErr| anyhow::anyhow!("Python ADK call failed: {}", e))
}

/// Process a user message through the ADK brain
pub fn process_message(state: &Arc<Mutex<EngineState>>, message: &str) -> Result<String> {
    let state_guard = state.lock().unwrap();
    
    // Record event
    let event = AgentEvent::UserMessage {
        session_id: state_guard.session_id.clone(),
        message: message.to_string(),
    };
    
    // Drop lock before calling Python
    drop(state_guard);
    
    // Call ADK brain
    let response = call_adk_brain(
        &state.lock().unwrap().user_id,
        &state.lock().unwrap().session_id,
        message,
    )?;
    
    // Record response event
    {
        let mut state_guard = state.lock().unwrap();
        state_guard.event_history.push(event);
    }
    
    Ok(response)
}

/// Get the current session state
pub fn get_session_state(state: &Arc<Mutex<EngineState>>) -> EngineState {
    state.lock().unwrap().clone()
}

/// Update the current model
pub fn set_current_model(state: &Arc<Mutex<EngineState>>, model: &str) {
    let mut state_guard = state.lock().unwrap();
    let old_model = state_guard.current_model.clone();
    state_guard.current_model = model.to_string();
    
    state_guard.event_history.push(AgentEvent::ModelRotation {
        old_model,
        new_model: model.to_string(),
    });
}
