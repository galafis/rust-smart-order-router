pub mod binance;
pub mod coinbase;
pub mod kraken;

use crate::types::{Liquidity, TradingPair};
use anyhow::Result;
use async_trait::async_trait;

/// Trait for exchange connectors
#[async_trait]
pub trait Exchange: Send + Sync {
    /// Get the exchange name
    fn name(&self) -> &str;

    /// Fetch current liquidity for a trading pair
    async fn get_liquidity(&self, pair: &TradingPair) -> Result<Liquidity>;

    /// Check if the exchange supports a trading pair
    async fn supports_pair(&self, pair: &TradingPair) -> bool;
}

/// Factory to create exchange instances
pub fn create_exchanges() -> Vec<Box<dyn Exchange>> {
    vec![
        Box::new(binance::BinanceExchange::new()),
        Box::new(coinbase::CoinbaseExchange::new()),
        Box::new(kraken::KrakenExchange::new()),
    ]
}
