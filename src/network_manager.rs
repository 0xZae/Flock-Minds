pub struct NetworkManager {
    connections: HashMap<ChainId, Connection>,
    config: NetworkConfig,
    status: NetworkStatus,
}

impl NetworkManager {
    pub async fn initialize(&mut self) -> Result<(), NetworkError> {
        for chain_id in self.config.enabled_chains.iter() {
            let connection = Connection::new(chain_id)
                .with_timeout(self.config.timeout)
                .with_retry(self.config.retry_policy);
            
            self.connections.insert(*chain_id, connection);
        }
        
        self.status = NetworkStatus::Ready;
        Ok(())
    }

    pub async fn broadcast_message(&self, message: Message) -> Result<(), NetworkError> {
        for connection in self.connections.values() {
            connection.send(message.clone()).await?;
        }
        Ok(())
    }
}
