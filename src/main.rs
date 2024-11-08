mod market_data;
mod order_book;
mod trading_engine;
mod risk_manager;
mod utils;

use tokio;
use market_data::MarketDataProcessor;
use order_book::OrderBook;
use trading_engine::TradingEngine;
use risk_manager::RiskManager;

#[tokio::main]
async fn main() {
    println!("Starting RustHFT...");

    let market_data_processor = MarketDataProcessor::new();
    let order_book = OrderBook::new();
    let risk_manager = RiskManager::new();
    let trading_engine = TradingEngine::new(order_book, risk_manager);

    // Simulate market data feed
    let market_data_feed = market_data_processor.process_market_data();

    // Process market data and execute trades
    trading_engine.run(market_data_feed).await;

    println!("RustHFT shutting down.");
}