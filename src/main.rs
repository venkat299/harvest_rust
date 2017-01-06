extern crate rustc_serialize;
use rustc_serialize::json;

extern crate harvest;
use harvest::executor;
use harvest::data::*;

extern crate toml;

fn main() {

    println!("Hello in English: {}", executor::execute("buy yesbank 30"));

    let order = RegularOrder {
        tradingsymbol: Symbol::YESBANK,
        exchange: Exchange::NSE,
        transaction_type: TransactionType::BUY,
        order_type: OrderType::LIMIT,
        product: Product::CNC,
        quantity: 100,
        price: 34.34,
        trigger_price: 0.0,
        disclosed_quantity: 0,
        validity: Validity::DAY,
        tag: "52_DAY".to_string(),
    };
    println!("order info {:?}", order);

    // Serialize using `json::encode`
    let encoded = json::encode(&order).unwrap();

    println!("json_str {:?}", encoded);



    let toml = r#"
    db_path = "/Users/venkat299/code/rust/harvest/db/harvest.db"
    "#;

    #[derive(Debug)]
    let mut value: toml::Value = toml.parse().unwrap();
    let db_path = value.lookup_mut("db_path").unwrap();

    println!("{}", db_path.to_string());




}
