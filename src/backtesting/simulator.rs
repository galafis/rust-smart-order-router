use crate::types::{ExecutionResult, OrderSplit};
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Simulate order execution with realistic parameters
pub fn simulate_execution(split: &OrderSplit) -> ExecutionResult {
    // Simulate small price variation
    let price_variation = dec!(0.0001); // 0.01% variation
    let executed_price = split.expected_price * (dec!(1.0) + price_variation);

    // Simulate fees (0.1% maker/taker fee)
    let fee_rate = dec!(0.001);
    let fees = split.quantity * executed_price * fee_rate;

    ExecutionResult {
        order_id: format!("ORDER_{}", Utc::now().timestamp()),
        exchange: split.exchange.clone(),
        executed_quantity: split.quantity,
        executed_price,
        fees,
        timestamp: Utc::now(),
    }
}

/// Simulate execution with custom slippage
pub fn simulate_with_slippage(split: &OrderSplit, slippage_bps: Decimal) -> ExecutionResult {
    let slippage = slippage_bps / dec!(10000.0); // Convert basis points to decimal
    let executed_price = split.expected_price * (dec!(1.0) + slippage);

    let fee_rate = dec!(0.001);
    let fees = split.quantity * executed_price * fee_rate;

    ExecutionResult {
        order_id: format!("ORDER_{}", Utc::now().timestamp()),
        exchange: split.exchange.clone(),
        executed_quantity: split.quantity,
        executed_price,
        fees,
        timestamp: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_simulate_execution() {
        let split = OrderSplit {
            exchange: "TestExchange".to_string(),
            quantity: dec!(1.0),
            expected_price: dec!(50000.0),
        };

        let result = simulate_execution(&split);
        assert_eq!(result.exchange, "TestExchange");
        assert_eq!(result.executed_quantity, dec!(1.0));
        assert!(result.executed_price > dec!(50000.0));
    }
}
