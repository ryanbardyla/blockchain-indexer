use anyhow::{Context, Result};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcBlockConfig, RpcTransactionConfig},
};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone)]
pub struct SolanaClient {
    http_client: RpcClient,
    ws_url: String,
}

impl SolanaClient {
    pub fn new(http_url: String, ws_url: String) -> Self {
        Self {
            http_client: RpcClient::new_with_commitment(
                http_url,
                CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                },
            ),
            ws_url,
        }
    }

    pub async fn get_latest_slot(&self) -> Result<u64> {
        self.http_client
            .get_slot()
            .await
            .context("Failed to get latest slot")
    }

    pub async fn get_block(
        &self,
        slot: u64,
    ) -> Result<solana_transaction_status::UiConfirmedBlock> {
        let config = RpcBlockConfig {
            encoding: Some(solana_transaction_status::UiTransactionEncoding::Base64),
            transaction_details: Some(solana_transaction_status::TransactionDetails::Full),
            rewards: Some(false),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(1),
        };

        self.http_client
            .get_block_with_config(slot, config)
            .await
            .context("Failed to get block")
    }

    pub async fn connect_websocket(
        &self,
    ) -> Result<
        impl futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>,
    > {
        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.ws_url)
            .await
            .context("Failed to connect to Solana WS")?;

        Ok(ws_stream)
    }
}
