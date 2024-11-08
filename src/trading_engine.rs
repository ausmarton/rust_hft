use futures::StreamExt;
use crate::market_data::MarketData;
use crate::order_book::OrderBook;
use crate::risk_manager::RiskManager;
use crate::utils::Order;

pub struct TradingEngine {
    order_book: OrderBook,
    risk_manager: RiskManager,
}

impl TradingEngine {
    pub fn new(order_book: OrderBook, risk_manager: RiskManager) -> Self {
        TradingEngine {
            order_book,
            risk_manager,
        }
    }

    pub async fn run(&self, mut market_data_feed: impl StreamExt<Item = MarketData> + Unpin) {
        while let Some(market_data) = market_data_feed.next().await {
            self.process_market_data(market_data);
        }
    }

    fn process_market_data(&self, market_data: MarketData) {
        println!("Processing market data: {:?}", market_data);

        if self.risk_manager.check_risk(&market_data) {
            let order = Order {
                symbol: market_data.symbol.clone(),
                price: market_data.price,
                quantity: 100,
                is_buy: true,
            };

            if self.risk_manager.approve_order(&order) {
                self.order_book.add_order(order);
                println!("Order added to the book");
            } else {
                println!("Order rejected by risk manager");
            }
        } else {
            println!("Market data rejected by risk manager");
        }
    }
}
