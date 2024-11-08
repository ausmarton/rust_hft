use std::collections::BTreeMap;
use dashmap::DashMap;
use crate::utils::Order;
use ordered_float::OrderedFloat;

pub struct OrderBook {
    bids: DashMap<String, BTreeMap<OrderedFloat<f64>, Vec<Order>>>,
    asks: DashMap<String, BTreeMap<OrderedFloat<f64>, Vec<Order>>>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: DashMap::new(),
            asks: DashMap::new(),
        }
    }

    pub fn add_order(&self, order: Order) {
        let book = if order.is_buy {
            &self.bids
        } else {
            &self.asks
        };

        book.entry(order.symbol.clone())
            .or_insert_with(BTreeMap::new)
            .entry(OrderedFloat(order.price))
            .or_insert_with(Vec::new)
            .push(order);
    }

    pub fn get_best_bid(&self, symbol: &str) -> Option<f64> {
        self.bids.get(symbol)
            .and_then(|bids| bids.keys().next_back().map(|k| k.0))
    }

    pub fn get_best_ask(&self, symbol: &str) -> Option<f64> {
        self.asks.get(symbol)
            .and_then(|asks| asks.keys().next().map(|k| k.0))
    }
}