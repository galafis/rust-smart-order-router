pub mod metrics;

use crate::types::{ExecutionResult, RoutingResult};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Calculate execution quality metrics
pub struct ExecutionAnalytics {
    results: Vec<ExecutionResult>,
}

impl ExecutionAnalytics {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: ExecutionResult) {
        self.results.push(result);
    }

    /// Calculate average execution price
    pub fn average_execution_price(&self) -> Decimal {
        if self.results.is_empty() {
            return dec!(0);
        }

        let total_value: Decimal = self
            .results
            .iter()
            .map(|r| r.executed_price * r.executed_quantity)
            .sum();

        let total_quantity: Decimal = self
            .results
            .iter()
            .map(|r| r.executed_quantity)
            .sum();

        if total_quantity > dec!(0) {
            total_value / total_quantity
        } else {
            dec!(0)
        }
    }

    /// Calculate total fees paid
    pub fn total_fees(&self) -> Decimal {
        self.results.iter().map(|r| r.fees).sum()
    }

    /// Calculate fill rate (percentage of order filled)
    pub fn fill_rate(&self, original_quantity: Decimal) -> Decimal {
        let filled: Decimal = self.results.iter().map(|r| r.executed_quantity).sum();
        if original_quantity > dec!(0) {
            (filled / original_quantity) * dec!(100)
        } else {
            dec!(0)
        }
    }

    /// Generate execution report
    pub fn generate_report(&self, routing: &RoutingResult) -> String {
        let avg_price = self.average_execution_price();
        let total_fees = self.total_fees();
        let fill_rate = self.fill_rate(routing.total_quantity);

        format!(
            "Execution Report:\n\
             - Average Price: {:.2}\n\
             - Expected Price: {:.2}\n\
             - Total Fees: {:.2}\n\
             - Fill Rate: {:.2}%\n\
             - Estimated Slippage: {:.4}%\n\
             - Exchanges Used: {}",
            avg_price,
            routing.average_price,
            total_fees,
            fill_rate,
            routing.estimated_slippage,
            routing.splits.len()
        )
    }
}
