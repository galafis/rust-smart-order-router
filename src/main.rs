use anyhow::Result;
use rust_decimal_macros::dec;
use smart_order_router::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("=== Smart Order Router Demo ===\n");

    // Create exchanges
    let exchanges = exchanges::create_exchanges();
    println!("Connected to {} exchanges", exchanges.len());

    // Create router
    let router = router::SmartOrderRouter::new(exchanges);

    // Example 1: Buy order
    println!("\n--- Example 1: Buy Order ---");
    let buy_order = Order {
        pair: TradingPair::new("BTC", "USD"),
        side: OrderSide::Buy,
        order_type: types::OrderType::Market,
        quantity: dec!(2.5),
        limit_price: None,
    };

    match router.route_order(&buy_order).await {
        Ok(routing) => {
            println!("Order routed successfully!");
            println!("Total quantity: {}", routing.total_quantity);
            println!("Average price: ${:.2}", routing.average_price);
            println!("Estimated slippage: {:.4}%", routing.estimated_slippage);
            println!("\nSplits:");
            for (i, split) in routing.splits.iter().enumerate() {
                println!(
                    "  {}. {} - Quantity: {}, Price: ${:.2}",
                    i + 1,
                    split.exchange,
                    split.quantity,
                    split.expected_price
                );
            }

            // Simulate execution
            println!("\n--- Simulating Execution ---");
            let mut analytics = analytics::ExecutionAnalytics::new();
            
            for split in &routing.splits {
                let execution = backtesting::simulator::simulate_execution(split);
                println!(
                    "Executed {} {} on {} at ${:.2} (fees: ${:.2})",
                    execution.executed_quantity,
                    buy_order.pair.base,
                    execution.exchange,
                    execution.executed_price,
                    execution.fees
                );
                analytics.add_result(execution);
            }

            println!("\n{}", analytics.generate_report(&routing));
        }
        Err(e) => println!("Error routing order: {}", e),
    }

    // Example 2: Sell order
    println!("\n\n--- Example 2: Sell Order ---");
    let sell_order = Order {
        pair: TradingPair::new("BTC", "USD"),
        side: OrderSide::Sell,
        order_type: types::OrderType::Market,
        quantity: dec!(1.0),
        limit_price: None,
    };

    match router.route_order(&sell_order).await {
        Ok(routing) => {
            println!("Order routed successfully!");
            println!("Total quantity: {}", routing.total_quantity);
            println!("Average price: ${:.2}", routing.average_price);
            println!("Estimated slippage: {:.4}%", routing.estimated_slippage);
            println!("\nSplits:");
            for (i, split) in routing.splits.iter().enumerate() {
                println!(
                    "  {}. {} - Quantity: {}, Price: ${:.2}",
                    i + 1,
                    split.exchange,
                    split.quantity,
                    split.expected_price
                );
            }
        }
        Err(e) => println!("Error routing order: {}", e),
    }

    // Example 3: Backtesting
    println!("\n\n--- Example 3: Backtesting ---");
    let mut backtest = backtesting::BacktestEngine::new();
    
    for i in 0..5 {
        let order = Order {
            pair: TradingPair::new("BTC", "USD"),
            side: if i % 2 == 0 { OrderSide::Buy } else { OrderSide::Sell },
            order_type: types::OrderType::Market,
            quantity: dec!(0.5) + dec!(0.1) * rust_decimal::Decimal::from(i),
            limit_price: None,
        };
        
        if let Ok(routing) = router.route_order(&order).await {
            let executions: Vec<_> = routing
                .splits
                .iter()
                .map(backtesting::simulator::simulate_execution)
                .collect();
            
            backtest.add_order(order);
            backtest.add_result(routing, executions);
        }
    }

    println!("{}", backtest.summary());

    println!("\n=== Demo Complete ===");
    Ok(())
}
