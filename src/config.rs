#[derive(Debug)]
pub struct Config {
    // Ethereum
    pub eth_rpc_url: String,
    pub eth_ws_url: String,

    // Solana
    pub solana_rpc_http_url: String,
    pub solana_rpc_ws_url: String,

    // Common
    pub db_url: String,
    pub max_blocks_per_batch: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            eth_rpc_url: env::var("ETH_RPC_URL")?,
            eth_ws_url: env::var("ETH_WS_URL")?,
            solana_rpc_http_url: env::var("SOLANA_RPC_HTTP_URL")?,
            solana_rpc_ws_url: env::var("SOLANA_RPC_WS_URL")?,
            db_url: env::var("DATABASE_URL")?,
            max_blocks_per_batch: env::var("MAX_BLOCKS_PER_BATCH")
                .unwrap_or("100".into())
                .parse()?,
        })
    }
}
