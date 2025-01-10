// src/main.rs
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

mod agent;
mod chain;
mod protocols;
mod identity;
mod graph;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    let state = Arc::new(Mutex::new(graph::SharedState::new()));
    
    let identity_manager = identity::IdentityManager::new(state.clone());
    let protocol_handler = protocols::ProtocolHandler::new(state.clone());
    let chain_router = chain::ChainRouter::new(state.clone());
    
    runtime.block_on(async {
        tokio::try_join!(
            identity_manager.run(),
            protocol_handler.run(),
            chain_router.run()
        )
    })?;
    
    Ok(())
}