// src/chain/mod.rs
mod router;
mod connection;
mod error;

pub use router::ChainRouter;
pub use connection::ChainConnection;
pub use error::ChainError;

// src/chain/router.rs
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use crate::protocols::{ProtocolMessage, MessageType};
use super::connection::ChainConnection;
use super::error::ChainError;

pub struct ChainRouter {
    state: Arc<Mutex<crate::graph::SharedState>>,
    chain_connections: RwLock<HashMap<String, ChainConnection>>,
    message_queue: mpsc::UnboundedReceiver<ProtocolMessage>,
}

impl ChainRouter {
    pub fn new(state: Arc<Mutex<crate::graph::SharedState>>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            state,
            chain_connections: RwLock::new(HashMap::new()),
            message_queue: rx,
        }
    }

    pub async fn run(&self) -> Result<(), ChainError> {
        loop {
            if let Some(msg) = self.message_queue.recv().await {
                self.process_message(msg).await?;
            }
        }
    }

    async fn process_message(&self, msg: ProtocolMessage) -> Result<(), ChainError> {
        match msg.message_type {
            MessageType::ChainSync => self.handle_chain_sync(msg).await?,
            MessageType::CrossChainTransaction => self.handle_cross_chain_tx(msg).await?,
            _ => return Err(ChainError::UnsupportedMessageType),
        }
        Ok(())
    }
}

// src/chain/connection.rs
use tokio::sync::mpsc;
use super::error::ChainError;

pub struct ChainConnection {
    chain_id: String,
    tx: mpsc::UnboundedSender<Vec<u8>>,
    rx: mpsc::UnboundedReceiver<Vec<u8>>,
}

// src/chain/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChainError {
    #[error("unsupported message type")]
    UnsupportedMessageType,
    
    #[error("connection failed: {0}")]
    ConnectionError(String),
    
    #[error("message processing failed: {0}")]
    MessageProcessingError(String),
}