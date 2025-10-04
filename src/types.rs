use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a trading pair (e.g., BTC/USD)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
}

impl TradingPair {
    pub fn new(base: impl Into<String>, quote: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            quote: quote.into(),
        }
    }
}

impl fmt::Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

/// Order side: Buy or Sell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

/// Represents an order to be routed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub pair: TradingPair,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: Decimal,
    pub limit_price: Option<Decimal>,
}

/// Represents liquidity available at an exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Liquidity {
    pub exchange: String,
    pub pair: TradingPair,
    pub bid_price: Decimal,
    pub bid_quantity: Decimal,
    pub ask_price: Decimal,
    pub ask_quantity: Decimal,
}

/// Represents a split order sent to a specific exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSplit {
    pub exchange: String,
    pub quantity: Decimal,
    pub expected_price: Decimal,
}

/// Routing result with optimal splits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingResult {
    pub original_order: Order,
    pub splits: Vec<OrderSplit>,
    pub total_quantity: Decimal,
    pub average_price: Decimal,
    pub estimated_slippage: Decimal,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub order_id: String,
    pub exchange: String,
    pub executed_quantity: Decimal,
    pub executed_price: Decimal,
    pub fees: Decimal,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
