// src/agent/mod.rs
mod types;
mod behavior;
mod error;

pub use types::{AgentMetadata, ExecutionRecord, ResourceMetrics};
pub use behavior::AgentBehavior;
pub use error::AgentError;

// src/agent/types.rs
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct AgentMetadata {
    pub id: String,
    pub capabilities: Vec<String>,
    pub trust_score: f64,
    pub execution_history: VecDeque<ExecutionRecord>,
}

#[derive(Clone, Debug)]
pub struct ExecutionRecord {
    pub timestamp: i64,
    pub operation: String,
    pub success: bool,
    pub resource_usage: ResourceMetrics,
}

#[derive(Clone, Debug)]
pub struct ResourceMetrics {
    pub compute_units: u64,
    pub memory_usage: u64,
    pub network_calls: u32,
}

// src/agent/behavior.rs
use async_trait::async_trait;
use super::error::AgentError;
use super::types::*;

#[async_trait]
pub trait AgentBehavior: Send + Sync {
    async fn initialize(&mut self) -> Result<(), AgentError>;
    async fn process_task(&self, task: Task) -> Result<TaskResult, AgentError>;
    async fn update_state(&mut self, state: AgentState) -> Result<(), AgentError>;
}

// src/agent/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("initialization failed: {0}")]
    InitializationError(String),
    
    #[error("task processing failed: {0}")]
    TaskProcessingError(String),
    
    #[error("state update failed: {0}")]
    StateUpdateError(String),
}