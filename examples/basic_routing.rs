use anyhow::Result;
use rust_decimal_macros::dec;
use smart_order_router::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Create exchanges
    let exchanges = exchanges::create_exchanges();
    
    // Create router
    let router = router::SmartOrderRouter::new(exchanges);

    // Create a buy order
    let order = Order {
        pair: TradingPair::new("BTC", "USD"),
        side: OrderSide::Buy,
        order_type: types::OrderType::Market,
        quantity: dec!(1.5),
        limit_price: None,
    };

    // Route the order
    let routing = router.route_order(&order).await?;

    // Display results
    println!("Order routed successfully!");
    println!("Average price: ${:.2}", routing.average_price);
    println!("Estimated slippage: {:.4}%", routing.estimated_slippage);
    println!("\nOrder splits:");
    
    for split in &routing.splits {
        println!(
            "- {}: {} BTC @ ${:.2}",
            split.exchange, split.quantity, split.expected_price
        );
    }

    Ok(())
}
