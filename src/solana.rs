use anyhow::Result;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct SolanaClient {
    rpc_client: RpcClient,
    ws_url: String,
}

impl SolanaClient {
    pub fn new(http_url: String, ws_url: String) -> Self {
        Self {
            rpc_client: RpcClient::new_with_commitment(http_url, CommitmentConfig::confirmed()),
            ws_url,
        }
    }

    pub async fn get_latest_slot(&self) -> Result<u64> {
        Ok(self.rpc_client.get_slot().await?)
    }

    pub async fn process_solana_block(&self, slot: u64) -> Result<()> {
        let block = self
            .rpc_client
            .get_block_with_config(
                slot,
                RpcTransactionConfig {
                    encoding: Some(solana_transaction_status::UiTransactionEncoding::Base64),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(1),
                },
            )
            .await?;

        for tx in block.transactions {
            self.process_solana_transaction(tx).await?;
        }

        Ok(())
    }

    async fn process_solana_transaction(
        &self,
        tx: solana_transaction_status::TransactionStatus,
    ) -> Result<()> {
        // Add DEX-specific parsing for Raydium/Orca/etc.
        // Example: Detect Raydium swaps
        if let Some(program_id) = &tx.transaction.meta.as_ref().unwrap().err {
            if program_id.to_string() == "RaydiumSwap" {
                self.process_raydium_swap(tx).await?;
            }
        }

        Ok(())
    }

    async fn process_raydium_swap(
        &self,
        tx: solana_transaction_status::TransactionStatus,
    ) -> Result<()> {
        // Implement Raydium swap parsing
        // Extract token amounts and prices
        Ok(())
    }

    pub async fn start_websocket(&self) -> Result<()> {
        let (mut socket, _) = tokio_tungstenite::connect_async(&self.ws_url).await?;

        loop {
            let msg = socket.next().await.ok_or("WebSocket disconnected")??;
            if let Message::Text(text) = msg {
                // Process real-time Solana updates
                self.handle_ws_message(text).await?;
            }
        }
    }

    async fn handle_ws_message(&self, msg: String) -> Result<()> {
        // Handle account updates or program subscriptions
        Ok(())
    }
}
