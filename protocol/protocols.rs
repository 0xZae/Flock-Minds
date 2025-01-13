// src/protocols/mod.rs
mod types;
mod handler;
mod error;

pub use types::{MessageType, ProtocolMessage, ProtocolMetadata};
pub use handler::ProtocolHandler;
pub use error::ProtocolError;

// src/protocols/types.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ChainSync,
    CrossChainTransaction,
    IdentityVerification,
    StateUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub metadata: ProtocolMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMetadata {
    pub timestamp: i64,
    pub origin_chain: String,
    pub destination_chain: Option<String>,
    pub priority: u8,
}

// src/protocols/handler.rs
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::collections::HashMap;
use super::types::*;
use super::error::ProtocolError;

pub struct ProtocolHandler {
    state: Arc<Mutex<crate::graph::SharedState>>,
    active_protocols: HashMap<String, Protocol>,
    rate_limiter: Arc<Semaphore>,
}

impl ProtocolHandler {
    pub fn new(state: Arc<Mutex<crate::graph::SharedState>>) -> Self {
        Self {
            state,
            active_protocols: HashMap::new(),
            rate_limiter: Arc::new(Semaphore::new(100)),
        }
    }
}

// src/protocols/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("protocol initialization failed: {0}")]
    InitializationError(String),
    
    #[error("rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("protocol execution failed: {0}")]
    ExecutionError(String),
}