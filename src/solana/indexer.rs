use super::{SolanaClient, SolanaError};
use sqlx::PgPool;
use tracing::{error, info};

pub struct SolanaIndexer {
    client: SolanaClient,
    db_pool: PgPool,
    current_slot: u64,
}

impl SolanaIndexer {
    pub fn new(client: SolanaClient, db_pool: PgPool) -> Self {
        Self {
            client,
            db_pool,
            current_slot: 0,
        }
    }

    pub async fn start(&mut self) -> Result<(), SolanaError> {
        self.current_slot = self.client.get_latest_slot().await?;

        loop {
            let latest_slot = self.client.get_latest_slot().await?;

            if latest_slot > self.current_slot {
                self.process_slots(self.current_slot + 1, latest_slot)
                    .await?;
                self.current_slot = latest_slot;
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    async fn process_slots(&self, start: u64, end: u64) -> Result<(), SolanaError> {
        for slot in start..=end {
            match self.client.get_block(slot).await {
                Ok(block) => {
                    self.process_block(block).await?;
                }
                Err(e) => error!("Error processing slot {}: {:?}", slot, e),
            }
        }
        Ok(())
    }

    async fn process_block(
        &self,
        block: solana_transaction_status::UiConfirmedBlock,
    ) -> Result<(), SolanaError> {
        if let Some(transactions) = block.transactions {
            for tx in transactions {
                self.process_transaction(tx).await?;
            }
        }
        Ok(())
    }

    async fn process_transaction(
        &self,
        tx: solana_transaction_status::TransactionStatus,
    ) -> Result<(), SolanaError> {
        // DEX processing logic will go here
        info!("Processing Solana transaction: {:?}", tx.signature);
        Ok(())
    }
}
