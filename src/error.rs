use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Blockchain error: {0}")]
    BlockchainError(#[from] ethers::providers::ProviderError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Solana RPC error: {0}")]
    SolanaRpcError(#[from] solana_client::client_error::ClientError),

    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
}

impl From<env::VarError> for IndexerError {
    fn from(e: env::VarError) -> Self {
        IndexerError::ConfigError(e.to_string())
    }
}
