pub mod simulator;

use crate::types::{ExecutionResult, Order, RoutingResult};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Backtesting engine for routing strategies
pub struct BacktestEngine {
    orders: Vec<Order>,
    results: Vec<(RoutingResult, Vec<ExecutionResult>)>,
}

impl BacktestEngine {
    pub fn new() -> Self {
        Self {
            orders: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn add_result(&mut self, routing: RoutingResult, executions: Vec<ExecutionResult>) {
        self.results.push((routing, executions));
    }

    /// Calculate average slippage across all backtested orders
    pub fn average_slippage(&self) -> Decimal {
        if self.results.is_empty() {
            return dec!(0);
        }

        let total_slippage: Decimal = self
            .results
            .iter()
            .map(|(routing, _)| routing.estimated_slippage)
            .sum();

        total_slippage / Decimal::from(self.results.len())
    }

    /// Calculate total fees paid
    pub fn total_fees(&self) -> Decimal {
        self.results
            .iter()
            .flat_map(|(_, executions)| executions)
            .map(|e| e.fees)
            .sum()
    }

    /// Generate backtest summary
    pub fn summary(&self) -> String {
        format!(
            "Backtest Summary:\n\
             - Total Orders: {}\n\
             - Average Slippage: {:.4}%\n\
             - Total Fees: {:.2}\n\
             - Total Executions: {}",
            self.orders.len(),
            self.average_slippage(),
            self.total_fees(),
            self.results.iter().map(|(_, e)| e.len()).sum::<usize>()
        )
    }
}
