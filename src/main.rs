mod api;
mod blockchain;
mod indexer;
mod solana;

use anyhow::Result;
use blockchain::BlockchainClient;
use indexer::Indexer;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    let config = Config::load()?;

    // Create single DB pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await?;

    // Initialize Ethereum
    let eth_client = Arc::new(BlockchainClient::new(&config.eth_rpc_url)?);

    // Initialize Solana
    let sol_client = solana::SolanaClient::new(
        config.solana_rpc_http_url.clone(),
        config.solana_rpc_ws_url.clone(),
    );

    // Start Ethereum indexer with cloned pool
    let eth_indexer = Indexer::new(eth_client.clone(), db_pool.clone());
    tokio::spawn(async move {
        eth_indexer.start().await.expect("Eth indexer failed");
    });

    // Start Solana indexer with cloned pool
    let sol_indexer = solana::indexer::SolanaIndexer::new(sol_client, db_pool.clone());
    tokio::spawn(async move {
        sol_indexer.start().await.expect("Solana indexer failed");
    });

    // Start API server with the original pool
    api::start_api_server(db_pool).await?;

    Ok(())
}
