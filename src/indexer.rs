// src/indexer.rs
use crate::error::IndexerError;
use anyhow::{anyhow, Result};
use ethers::{
    abi::{AbiDecode, Detokenize},
    prelude::*,
    types::{Address, Log, Transaction, H160, U256},
};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info, warn};

// Add DEX constants (NEW)
const UNISWAP_V2_FACTORY: H160 = H160([
    0x5C, 0x69, 0x5e, 0x5a, 0x5a, 0x5d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
]);

// Add SwapEvent struct (NEW)
#[derive(Debug, ethabi::Event)]
struct SwapEvent {
    #[ethevent(indexed)]
    sender: Address,
    amount0_in: U256,
    amount1_out: U256,
    to: Address,
}

pub struct Indexer {
    blockchain_client: Arc<BlockchainClient>,
    db_pool: PgPool,
    current_block: u64,
}

impl Indexer {
    pub fn new(blockchain_client: Arc<BlockchainClient>, db_pool: PgPool) -> Self {
        Self {
            blockchain_client,
            db_pool,
            current_block: 0,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        self.current_block = self.blockchain_client.get_latest_block().await?;

        loop {
            let latest_block = self.blockchain_client.get_latest_block().await?;

            if latest_block > self.current_block {
                self.process_blocks(self.current_block + 1, latest_block)
                    .await?;
                self.current_block = latest_block;
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    async fn process_blocks(&self, start: u64, end: u64) -> Result<()> {
        for block_number in start..=end {
            if let Some(block) = self
                .blockchain_client
                .provider
                .get_block(block_number)
                .await?
            {
                self.process_transactions(block.transactions).await?;
            }
        }
        Ok(())
    }

    // UPDATED with proper error handling
    async fn process_transactions(&self, transactions: Vec<Transaction>) -> Result<()> {
        for tx in transactions {
            match self.process_transaction(tx).await {
                Ok(_) => info!("Processed transaction successfully"),
                Err(e) => error!("Error processing transaction: {:?}", e),
            }
        }
        Ok(())
    }

    // NEW METHOD: Process individual transaction
    async fn process_transaction(&self, tx: Transaction) -> Result<(), IndexerError> {
        let receipt = self
            .blockchain_client
            .provider
            .get_transaction_receipt(tx.hash)
            .await?
            .ok_or_else(|| anyhow!("No receipt for tx {}", tx.hash))?;

        for log in receipt.logs {
            if log.address == UNISWAP_V2_FACTORY {
                self.process_swap_log(log).await?;
            }
        }

        Ok(())
    }

    // NEW METHOD: Process swap events
    async fn process_swap_log(&self, log: Log) -> Result<(), IndexerError> {
        let swap_event = SwapEvent::decode_log(&log.into())?;

        sqlx::query!(
            r#"
            INSERT INTO price_feeds (pair, price, block_number, timestamp)
            VALUES ($1, $2, $3, $4)
            "#,
            log.address.to_string(),
            self.calculate_price(swap_event.amount0_in, swap_event.amount1_out),
            log.block_number.unwrap().as_u64() as i64,
            chrono::Utc::now().naive_utc()
        )
        .execute(&self.db_pool)
        .await?;

        info!(
            "Processed swap: Pair {} @ {}",
            log.address,
            swap_event.amount1_out.as_u128() as f64 / swap_event.amount0_in.as_u128() as f64
        );

        Ok(())
    }

    // NEW METHOD: Price calculation
    fn calculate_price(&self, amount_in: U256, amount_out: U256) -> f64 {
        amount_out.as_u128() as f64 / amount_in.as_u128() as f64
    }
}
