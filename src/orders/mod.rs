struct Order {
    account_id: String,
    currency_price: f64,
    commodity_amount: f64,
}

impl Order {
    pub fn new(account_id: String, currency_price: f64, commodity_amount: f64) -> Order {
        return Order {
            account_id: account_id,
            currency_price: currency_price,
            commodity_amount: commodity_amount,
        }
    }

    pub fn get_currency_price(&self) -> f64 {
        return self.currency_price;
    }

    pub fn get_commodity_amount(&self) -> f64 {
        return self.commodity_amount;
    }

    pub fn get_account_id(&self) -> String {
        return self.account_id.clone();
    }
}

pub struct BidOrder {
    order: Order,
}

impl BidOrder {
    pub fn new(account_id: String, currency_price: f64, commodity_amount: f64) -> BidOrder {
        return BidOrder {
            order: Order::new(account_id, currency_price, commodity_amount),
        }
    }

    pub fn get_currency_price(&self) -> f64 {
        return self.order.get_currency_price();
    }

    pub fn get_commodity_amount(&self) -> f64 {
        return self.order.get_commodity_amount();
    }

    pub fn get_account_id(&self) -> String {
        return self.order.get_account_id();
    }
}

pub struct AskOrder {
    order: Order,
}

impl AskOrder {
    pub fn new(account_id: String, currency_price: f64, commodity_amount: f64) -> AskOrder {
        return AskOrder {
            order: Order::new(account_id, currency_price, commodity_amount),
        }
    }
    pub fn get_currency_price(&self) -> f64 {
        return self.order.get_currency_price();
    }

    pub fn get_commodity_amount(&self) -> f64 {
        return self.order.get_commodity_amount();
    }

    pub fn get_account_id(&self) -> String {
        return self.order.get_account_id();
    }
}
