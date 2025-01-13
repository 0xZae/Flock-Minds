#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let config = AgentConfig::default();
        let agent = Agent::new(config);
        assert!(agent.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_agent_message_handling() {
        let agent = Agent::default();
        let message = Message::new("test_data");
        agent.handle_message(message).await?;
        assert_eq!(agent.message_count(), 1);
    }
}
