use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Calculate slippage between expected and executed price
pub fn calculate_slippage(expected_price: Decimal, executed_price: Decimal) -> Decimal {
    if expected_price == dec!(0) {
        return dec!(0);
    }
    ((executed_price - expected_price) / expected_price).abs() * dec!(100)
}

/// Calculate implementation shortfall
pub fn implementation_shortfall(
    decision_price: Decimal,
    execution_price: Decimal,
    quantity: Decimal,
) -> Decimal {
    (execution_price - decision_price) * quantity
}

/// Calculate effective spread
pub fn effective_spread(mid_price: Decimal, execution_price: Decimal) -> Decimal {
    (execution_price - mid_price).abs() * dec!(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_slippage() {
        let expected = dec!(100.0);
        let executed = dec!(101.0);
        let slippage = calculate_slippage(expected, executed);
        assert_eq!(slippage, dec!(1.0));
    }

    #[test]
    fn test_implementation_shortfall() {
        let decision = dec!(100.0);
        let execution = dec!(101.0);
        let quantity = dec!(10.0);
        let shortfall = implementation_shortfall(decision, execution, quantity);
        assert_eq!(shortfall, dec!(10.0));
    }
}
