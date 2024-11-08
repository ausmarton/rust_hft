use tokio::sync::mpsc;
use futures::stream::Stream;
use std::pin::Pin;
use serde::{Serialize, Deserialize};
use rand::{Rng, SeedableRng, rngs::StdRng};
use chrono::Utc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub volume: u64,
    pub timestamp: i64,
}

pub struct MarketDataProcessor;

impl MarketDataProcessor {
    pub fn new() -> Self {
        MarketDataProcessor
    }

    pub fn process_market_data(&self) -> Pin<Box<dyn Stream<Item = MarketData> + Send>> {
        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            // Use a thread-safe random number generator
            let mut rng = StdRng::from_entropy();
            
            loop {
                let market_data = MarketData {
                    symbol: "AAPL".to_string(),
                    price: rng.gen_range(150.0..200.0),
                    volume: rng.gen_range(100..1000),
                    timestamp: Utc::now().timestamp_millis(),
                };
                if tx.send(market_data).await.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });

        Box::pin(ReceiverStream::new(rx))
    }
}