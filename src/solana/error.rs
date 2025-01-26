use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("WebSocket error: {0}")]
    WsError(String),

    #[error("Transaction parsing error")]
    TransactionParseError,

    #[error("Invalid DEX data format")]
    InvalidDexData,
}

impl From<solana_client::client_error::ClientError> for SolanaError {
    fn from(e: solana_client::client_error::ClientError) -> Self {
        SolanaError::RpcError(e.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for SolanaError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        SolanaError::WsError(e.to_string())
    }
}
