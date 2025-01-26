use ethers::prelude::*;
use anyhow::Result;

pub struct BlockchainClient {
    provider: Provider<Http>,
}

impl BlockchainClient {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        Ok(Self { provider })
    }

    pub async fn get_latest_block(&self) -> Result<u64> {
        Ok(self.provider.get_block_number().await?.as_u64())
    }
}