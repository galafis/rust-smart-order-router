use crate::types::{Order, OrderSplit};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Split order using VWAP (Volume Weighted Average Price) strategy
pub fn vwap_split(order: &Order, num_splits: usize) -> Vec<OrderSplit> {
    let quantity_per_split = order.quantity / Decimal::from(num_splits);
    
    (0..num_splits)
        .map(|i| OrderSplit {
            exchange: format!("Exchange_{}", i + 1),
            quantity: quantity_per_split,
            expected_price: dec!(50000.0), // Mock price
        })
        .collect()
}

/// Split order using TWAP (Time Weighted Average Price) strategy
pub fn twap_split(order: &Order, time_intervals: usize) -> Vec<OrderSplit> {
    let quantity_per_interval = order.quantity / Decimal::from(time_intervals);
    
    (0..time_intervals)
        .map(|i| OrderSplit {
            exchange: format!("Interval_{}", i + 1),
            quantity: quantity_per_interval,
            expected_price: dec!(50000.0), // Mock price
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OrderSide, OrderType, TradingPair};

    #[test]
    fn test_vwap_split() {
        let order = Order {
            pair: TradingPair::new("BTC", "USD"),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            quantity: dec!(10.0),
            limit_price: None,
        };

        let splits = vwap_split(&order, 5);
        assert_eq!(splits.len(), 5);
        assert_eq!(splits[0].quantity, dec!(2.0));
    }
}
