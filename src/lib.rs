//! # Smart Order Router
//!
//! A high-performance smart order router for optimal trade execution across multiple exchanges.
//!
//! ## Features
//!
//! - Multi-exchange connectivity (Binance, Coinbase, Kraken)
//! - Intelligent order routing algorithms (VWAP, TWAP, Implementation Shortfall)
//! - Real-time liquidity analysis
//! - Order splitting to minimize slippage
//! - Comprehensive execution analytics
//! - Backtesting framework
//!
//! ## Example
//!
//! ```no_run
//! use smart_order_router::*;
//! use rust_decimal_macros::dec;
//!
//! #[tokio::main]
//! async fn main() {
//!     let exchanges = exchanges::create_exchanges();
//!     let router = router::SmartOrderRouter::new(exchanges);
//!     
//!     let order = types::Order {
//!         pair: types::TradingPair::new("BTC", "USD"),
//!         side: types::OrderSide::Buy,
//!         order_type: types::OrderType::Market,
//!         quantity: dec!(1.0),
//!         limit_price: None,
//!     };
//!     
//!     let result = router.route_order(&order).await.unwrap();
//!     println!("Routing result: {:?}", result);
//! }
//! ```

pub mod analytics;
pub mod backtesting;
pub mod exchanges;
pub mod router;
pub mod types;

// Re-export commonly used types
pub use types::{Order, OrderSide, OrderType, RoutingResult, TradingPair};
