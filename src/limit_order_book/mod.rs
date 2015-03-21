use super::orders;
extern crate core;

pub struct LimitOrderBook {
    bid_orders: Vec<orders::BidOrder>,
    ask_orders: Vec<orders::AskOrder>,
}

impl LimitOrderBook {
    pub fn new() -> LimitOrderBook {
        return LimitOrderBook{bid_orders: Vec::new(), ask_orders: Vec::new()};
    }

    pub fn iter_bid_orders(&self) -> core::slice::Iter<orders::BidOrder> {
        return self.bid_orders.iter();
    }

    pub fn iter_ask_orders(&self) -> core::slice::Iter<orders::AskOrder> {
        return self.ask_orders.iter();
    }

    pub fn add_bid_order(&mut self, account_id: String, currency_price: f64, commodity_amount: f64) {
        self.bid_orders.push(
            orders::BidOrder::new(
                account_id,
                currency_price,
                commodity_amount
            )
        );
    }
}
