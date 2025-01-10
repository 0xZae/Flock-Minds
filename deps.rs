[package]
name = "node_frame"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.28", features = ["full"] }
async-trait = "0.1"
petgraph = "0.6"
ring = "0.16"
futures = "0.3"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
dashmap = "5.4"