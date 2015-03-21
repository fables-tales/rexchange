extern crate uuid;

pub struct Account {
    id: String,
    balance_currency: f64,
    balance_commodity: f64,
}

impl Account {
    pub fn get_id(self) -> String {
        return self.id;
    }

    pub fn get_balance_currency(self) -> f64 {
        return self.balance_currency;
    }

    pub fn get_balance_commodity(self) -> f64 {
        return self.balance_commodity;
    }

    pub fn deposit_currency(&mut self, amount: f64) {
        self.balance_currency += amount;
    }

    pub fn deposit_commodity(&mut self, amount: f64) {
        self.balance_commodity += amount;
    }

    pub fn new() -> Account {
        return Account{
            id: uuid::Uuid::new_v4().to_string(),
            balance_currency: 0.0,
            balance_commodity: 0.0,
        };
    }
}
