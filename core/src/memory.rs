//! Memory IPC bridge between Rust and Python ADK

use anyhow::Result;
use std::sync::{Arc, Mutex};

/// Session memory state
#[derive(Debug, Clone)]
pub struct SessionMemory {
    pub session_id: String,
    pub turn_count: usize,
    pub tool_calls: usize,
    pub history: Vec<MemoryEntry>,
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub role: String,  // "user" or "agent"
    pub content: String,
    pub timestamp: u64,
}

impl SessionMemory {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            turn_count: 0,
            tool_calls: 0,
            history: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, role: &str, content: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.history.push(MemoryEntry {
            role: role.to_string(),
            content: content.to_string(),
            timestamp,
        });
        self.turn_count += 1;
    }

    pub fn increment_tool_calls(&mut self) {
        self.tool_calls += 1;
    }

    pub fn get_recent(&self, limit: usize) -> Vec<&MemoryEntry> {
        self.history.iter().rev().take(limit).collect()
    }
}

/// Shared memory bus for IPC
pub struct MemoryBus {
    sessions: Arc<Mutex<Vec<SessionMemory>>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn create_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.push(SessionMemory::new(session_id));
        Ok(())
    }

    pub fn get_session(&self, session_id: &str) -> Option<SessionMemory> {
        let sessions = self.sessions.lock().unwrap();
        sessions.iter().find(|s| s.session_id == session_id).cloned()
    }

    pub fn add_to_session(&self, session_id: &str, role: &str, content: &str) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.iter_mut().find(|s| s.session_id == session_id) {
            session.add_entry(role, content);
            Ok(())
        } else {
            anyhow::bail!("Session not found: {}", session_id)
        }
    }

    pub fn get_all_sessions(&self) -> Vec<SessionMemory> {
        self.sessions.lock().unwrap().clone()
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}
