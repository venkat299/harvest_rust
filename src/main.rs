extern crate rustc_serialize;
use rustc_serialize::json;


extern crate harvest;


use harvest::executor;
use harvest::data::*;

fn main() {

    println!("Hello in English: {}", executor::execute("buy yesbank 30"));
    
    let order = Order{ exchange: Exchange::NSE,
    				 transaction_type:TransactionType::BUY,
    				 order_type:OrderType::MARKET,
    				 product:Product::CNC,
    				 quantity:100 
    				};
    println!("order info {:?}", order);

	// Serialize using `json::encode`
    let encoded = json::encode(&order).unwrap();

    println!("json_str {:?}", encoded);
}