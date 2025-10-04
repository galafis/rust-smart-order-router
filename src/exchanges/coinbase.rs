use super::Exchange;
use crate::types::{Liquidity, TradingPair};
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal_macros::dec;

pub struct CoinbaseExchange {
    #[allow(dead_code)]
    client: reqwest::Client,
}

impl CoinbaseExchange {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Exchange for CoinbaseExchange {
    fn name(&self) -> &str {
        "Coinbase"
    }

    async fn get_liquidity(&self, pair: &TradingPair) -> Result<Liquidity> {
        // Mock implementation - in production, this would call Coinbase API
        Ok(Liquidity {
            exchange: self.name().to_string(),
            pair: pair.clone(),
            bid_price: dec!(49950.0),
            bid_quantity: dec!(1.2),
            ask_price: dec!(50000.0),
            ask_quantity: dec!(1.8),
        })
    }

    async fn supports_pair(&self, _pair: &TradingPair) -> bool {
        true
    }
}
