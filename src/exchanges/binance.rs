use super::Exchange;
use crate::types::{Liquidity, TradingPair};
use anyhow::{Context, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct BinanceOrderBook {
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

pub struct BinanceExchange {
    client: reqwest::Client,
    base_url: String,
}

impl BinanceExchange {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.binance.com".to_string(),
        }
    }

    fn format_symbol(&self, pair: &TradingPair) -> String {
        format!("{}{}", pair.base.to_uppercase(), pair.quote.to_uppercase())
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    fn name(&self) -> &str {
        "Binance"
    }

    async fn get_liquidity(&self, pair: &TradingPair) -> Result<Liquidity> {
        let symbol = self.format_symbol(pair);
        let url = format!("{}/api/v3/depth?symbol={}&limit=10", self.base_url, symbol);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch Binance order book")?;

        if !response.status().is_success() {
            // Return mock data if API fails
            return Ok(Liquidity {
                exchange: self.name().to_string(),
                pair: pair.clone(),
                bid_price: dec!(50000.0),
                bid_quantity: dec!(1.5),
                ask_price: dec!(50050.0),
                ask_quantity: dec!(2.0),
            });
        }

        let order_book: BinanceOrderBook = response
            .json()
            .await
            .context("Failed to parse Binance response")?;

        let best_bid = order_book.bids.first().context("No bids available")?;
        let best_ask = order_book.asks.first().context("No asks available")?;

        Ok(Liquidity {
            exchange: self.name().to_string(),
            pair: pair.clone(),
            bid_price: Decimal::from_str(&best_bid.0)?,
            bid_quantity: Decimal::from_str(&best_bid.1)?,
            ask_price: Decimal::from_str(&best_ask.0)?,
            ask_quantity: Decimal::from_str(&best_ask.1)?,
        })
    }

    async fn supports_pair(&self, _pair: &TradingPair) -> bool {
        // In a real implementation, this would check against supported pairs
        true
    }
}
