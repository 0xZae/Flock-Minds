#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consensus_reaching() {
        let mut consensus = ConsensusEngine::new();
        let proposal = Proposal::new("test_proposal");
        let result = consensus.reach_consensus(proposal).await?;
        assert!(result.is_consensus_reached());
    }
}