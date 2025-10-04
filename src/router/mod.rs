pub mod optimizer;
pub mod splitter;

use crate::exchanges::Exchange;
use crate::types::{Order, OrderSide, RoutingResult};
use anyhow::Result;

/// Smart Order Router
pub struct SmartOrderRouter {
    exchanges: Vec<Box<dyn Exchange>>,
}

impl SmartOrderRouter {
    pub fn new(exchanges: Vec<Box<dyn Exchange>>) -> Self {
        Self { exchanges }
    }

    /// Route an order across multiple exchanges
    pub async fn route_order(&self, order: &Order) -> Result<RoutingResult> {
        log::info!("Routing order: {:?}", order);

        // Fetch liquidity from all exchanges
        let mut liquidities = Vec::new();
        for exchange in &self.exchanges {
            if exchange.supports_pair(&order.pair).await {
                match exchange.get_liquidity(&order.pair).await {
                    Ok(liquidity) => liquidities.push(liquidity),
                    Err(e) => log::warn!("Failed to get liquidity from {}: {}", exchange.name(), e),
                }
            }
        }

        // Use optimizer to find best routing
        let routing = match order.side {
            OrderSide::Buy => optimizer::optimize_buy_order(order, &liquidities)?,
            OrderSide::Sell => optimizer::optimize_sell_order(order, &liquidities)?,
        };

        log::info!("Routing complete: {} splits", routing.splits.len());
        Ok(routing)
    }

    /// Get number of connected exchanges
    pub fn exchange_count(&self) -> usize {
        self.exchanges.len()
    }
}
