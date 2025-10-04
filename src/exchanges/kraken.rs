use super::Exchange;
use crate::types::{Liquidity, TradingPair};
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal_macros::dec;

pub struct KrakenExchange {
    #[allow(dead_code)]
    client: reqwest::Client,
}

impl KrakenExchange {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Exchange for KrakenExchange {
    fn name(&self) -> &str {
        "Kraken"
    }

    async fn get_liquidity(&self, pair: &TradingPair) -> Result<Liquidity> {
        // Mock implementation - in production, this would call Kraken API
        Ok(Liquidity {
            exchange: self.name().to_string(),
            pair: pair.clone(),
            bid_price: dec!(49980.0),
            bid_quantity: dec!(2.5),
            ask_price: dec!(50030.0),
            ask_quantity: dec!(1.5),
        })
    }

    async fn supports_pair(&self, _pair: &TradingPair) -> bool {
        true
    }
}
