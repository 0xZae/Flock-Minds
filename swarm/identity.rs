// src/identity/mod.rs
mod manager;
mod types;
mod error;

pub use manager::IdentityManager;
pub use types::{IdentityRecord, VerificationEvent, VerificationType};
pub use error::IdentityError;

// src/identity/manager.rs
use ring::{digest, signature};
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use super::types::*;
use super::error::IdentityError;

pub struct IdentityManager {
    state: Arc<Mutex<crate::graph::SharedState>>,
    identity_store: RwLock<HashMap<String, IdentityRecord>>,
    verification_queue: mpsc::UnboundedReceiver<VerificationRequest>,
}

impl IdentityManager {
    pub fn new(state: Arc<Mutex<crate::graph::SharedState>>) -> Self {
        let (_, rx) = mpsc::unbounded_channel();
        Self {
            state,
            identity_store: RwLock::new(HashMap::new()),
            verification_queue: rx,
        }
    }
}

// src/identity/types.rs
#[derive(Clone, Debug)]
pub struct IdentityRecord {
    pub public_key: Vec<u8>,
    pub metadata: IdentityMetadata,
    pub trust_score: f64,
    pub verification_history: Vec<VerificationEvent>,
}

#[derive(Clone, Debug)]
pub struct VerificationEvent {
    pub timestamp: i64,
    pub verifier: String,
    pub success: bool,
    pub verification_type: VerificationType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VerificationType {
    InitialVerification,
    PeriodicCheck,
    TransactionVerification,
    CrossChainVerification,
}

// src/identity/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("verification failed: {0}")]
    VerificationError(String),
    
    #[error("invalid identity data: {0}")]
    InvalidIdentityData(String),
    
    #[error("storage error: {0}")]
    StorageError(String),
}