#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chain_connection() {
        let config = ChainConfig::default();
        let connection = ChainConnection::new(config);
        assert!(connection.connect().await.is_ok());
    }

    #[tokio::test]
    async fn test_connection_timeout() {
        let config = ChainConfig {
            timeout: Duration::from_millis(1),
            ..Default::default()
        };
        let connection = ChainConnection::new(config);
        assert!(matches!(
            connection.connect().await,
            Err(ChainError::ConnectionTimeout)
        ));
    }
}