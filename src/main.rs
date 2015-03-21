extern crate rexchange;

use rexchange::account::Account;
use rexchange::limit_order_book::LimitOrderBook;

fn main() {
    let mut account = Account::new();

    account.deposit_currency(37.0);

    let mut lob = LimitOrderBook::new();
    lob.add_bid_order(account.get_id(), 37.0, 37.0);

    for order in lob.iter_bid_orders() {
        println!("{} {} {}", order.get_account_id(), order.get_currency_price(), order.get_commodity_amount());
    }
}
