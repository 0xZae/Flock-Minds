// src/graph/mod.rs
mod state;
mod types;
mod error;

pub use state::SharedState;
pub use types::{NodeData, EdgeData, NodeType};
pub use error::GraphError;

// src/graph/state.rs
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use super::types::*;
use super::error::GraphError;

pub struct SharedState {
    knowledge_graph: RwLock<DiGraph<NodeData, EdgeData>>,
    context_store: RwLock<HashMap<String, ContextData>>,
    active_sessions: RwLock<HashSet<String>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            knowledge_graph: RwLock::new(DiGraph::new()),
            context_store: RwLock::new(HashMap::new()),
            active_sessions: RwLock::new(HashSet::new()),
        }
    }

    pub async fn add_node(&self, data: NodeData) -> Result<NodeIndex, GraphError> {
        let mut graph = self.knowledge_graph.write().await;
        Ok(graph.add_node(data))
    }
}

// src/graph/types.rs
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct NodeData {
    pub node_type: NodeType,
    pub attributes: HashMap<String, String>,
    pub last_updated: i64,
}

#[derive(Clone, Debug)]
pub struct EdgeData {
    pub relationship_type: String,
    pub weight: f64,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeType {
    Agent,
    User,
    Resource,
    Context,
    Transaction,
}

#[derive(Clone, Debug)]
pub struct ContextData {
    pub context_type: String,
    pub data: HashMap<String, serde_json::Value>,
    pub created_at: i64,
    pub updated_at: i64,
}

// src/graph/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("node operation failed: {0}")]
    NodeError(String),
    
    #[error("edge operation failed: {0}")]
    EdgeError(String),
    
    #[error("context operation failed: {0}")]
    ContextError(String),
}