use crate::types::{Liquidity, Order, OrderSplit, RoutingResult};
use anyhow::{Context, Result};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Optimize routing for a buy order
pub fn optimize_buy_order(order: &Order, liquidities: &[Liquidity]) -> Result<RoutingResult> {
    // Sort exchanges by best ask price (lowest first)
    let mut sorted_liquidities = liquidities.to_vec();
    sorted_liquidities.sort_by(|a, b| a.ask_price.cmp(&b.ask_price));

    let mut splits = Vec::new();
    let mut remaining_quantity = order.quantity;
    let mut total_cost = dec!(0);

    // Greedy algorithm: fill from cheapest exchange first
    for liquidity in sorted_liquidities {
        if remaining_quantity <= dec!(0) {
            break;
        }

        let available = liquidity.ask_quantity;
        let fill_quantity = remaining_quantity.min(available);

        splits.push(OrderSplit {
            exchange: liquidity.exchange.clone(),
            quantity: fill_quantity,
            expected_price: liquidity.ask_price,
        });

        total_cost += fill_quantity * liquidity.ask_price;
        remaining_quantity -= fill_quantity;
    }

    if remaining_quantity > dec!(0) {
        anyhow::bail!("Insufficient liquidity to fill order");
    }

    let average_price = total_cost / order.quantity;
    let best_price = liquidities
        .iter()
        .map(|l| l.ask_price)
        .min()
        .context("No liquidity available")?;
    let estimated_slippage = ((average_price - best_price) / best_price) * dec!(100);

    Ok(RoutingResult {
        original_order: order.clone(),
        splits,
        total_quantity: order.quantity,
        average_price,
        estimated_slippage,
    })
}

/// Optimize routing for a sell order
pub fn optimize_sell_order(order: &Order, liquidities: &[Liquidity]) -> Result<RoutingResult> {
    // Sort exchanges by best bid price (highest first)
    let mut sorted_liquidities = liquidities.to_vec();
    sorted_liquidities.sort_by(|a, b| b.bid_price.cmp(&a.bid_price));

    let mut splits = Vec::new();
    let mut remaining_quantity = order.quantity;
    let mut total_revenue = dec!(0);

    // Greedy algorithm: sell to highest bidder first
    for liquidity in sorted_liquidities {
        if remaining_quantity <= dec!(0) {
            break;
        }

        let available = liquidity.bid_quantity;
        let fill_quantity = remaining_quantity.min(available);

        splits.push(OrderSplit {
            exchange: liquidity.exchange.clone(),
            quantity: fill_quantity,
            expected_price: liquidity.bid_price,
        });

        total_revenue += fill_quantity * liquidity.bid_price;
        remaining_quantity -= fill_quantity;
    }

    if remaining_quantity > dec!(0) {
        anyhow::bail!("Insufficient liquidity to fill order");
    }

    let average_price = total_revenue / order.quantity;
    let best_price = liquidities
        .iter()
        .map(|l| l.bid_price)
        .max()
        .context("No liquidity available")?;
    let estimated_slippage = ((best_price - average_price) / best_price) * dec!(100);

    Ok(RoutingResult {
        original_order: order.clone(),
        splits,
        total_quantity: order.quantity,
        average_price,
        estimated_slippage,
    })
}
