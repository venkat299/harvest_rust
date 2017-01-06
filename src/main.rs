extern crate rustc_serialize;
use rustc_serialize::json;

extern crate harvest;
use harvest::executor;
use harvest::data::*;
use harvest::helper;
use std::path::Path;

extern crate toml;

extern crate rusqlite;
// extern crate time;
// use time::Timespec;
use rusqlite::Connection;

fn main() {

    println!("Hello in English: {}", executor::execute("buy yesbank 30"));


    // order structure
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
        tag: "LOW52D".to_string(),
    };
    println!("order info {:?}", order);

    // Serialize using `json::encode`
    let encoded = json::encode(&order).unwrap();
    println!("json_str {:?}", encoded);


    // getting absolute path
    let rel_path = "./config/config.toml";
    let path = Path::new(&rel_path).canonicalize().unwrap();
    let config_path = path.to_str().unwrap();

    // getting config
    let app_config = helper::read_config(&config_path);
    let db_path_toml = app_config.get("db_path").unwrap();
    let db_path = db_path_toml.as_str().unwrap();

    println!("{:?}", &db_path);

    // connecting to sqlite
    let conn = Connection::open(Path::new(&db_path)).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM eod").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
            harvest::data::Eod {
                symbol: row.get(0),
                series: row.get(1),
                open: row.get(2),
                high: row.get(3),
                low: row.get(4),
                close: row.get(5),
                last: row.get(6),
                prevclose: row.get(7),
                tottrdqty: row.get(8),
                tottrdval: row.get(9),
                timestamp: row.get(10),
                totaltrades: row.get(11),
                isin: row.get(12),
            }
        })
        .unwrap();
    println!("person length {:?}", &person_iter.count());
    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }



}
