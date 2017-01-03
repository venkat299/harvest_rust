use rustc_serialize::json::{ToJson, Json};

#[derive(Debug)]
pub enum OrderStatus {
    OPEN,
    CANCELLED,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum Exchange {
    NSE,
    BSE,
}
impl Exchange {
    fn get_str(&self) -> &'static str {
        match *self {
            Exchange::NSE => "NSE",
            Exchange::BSE => "BSE",
        }
    }
}
impl ToJson for Exchange {
    fn to_json(&self) -> Json {
        to_json_impl("exchange".to_string(), self.get_str())
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum TransactionType {
    BUY,
    SELL,
}
impl TransactionType {
    fn get_str(&self) -> &'static str {
        match *self {
            TransactionType::BUY => "BUY",
            TransactionType::SELL => "SELL",
        }
    }
}
impl ToJson for TransactionType {
    fn to_json(&self) -> Json {
        to_json_impl("transaction_type".to_string(), self.get_str())
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum OrderType {
    MARKET,
    LIMIT,
}
impl OrderType {
    fn get_str(&self) -> &'static str {
        match *self {
            OrderType::MARKET => "MARKET",
            OrderType::LIMIT => "LIMIT",
        }
    }
}
impl ToJson for OrderType {
    fn to_json(&self) -> Json {
        to_json_impl("order_type".to_string(), self.get_str())
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum Product {
    MIS,
    CNC,
    NRML,
}
impl Product {
    fn get_str(&self) -> &'static str {
        match *self {
            Product::MIS => "MIS",
            Product::CNC => "CNC",
            Product::NRML => "NRML",
        }
    }
}
impl ToJson for Product {
    fn to_json(&self) -> Json {
        to_json_impl("product".to_string(), self.get_str())
    }
}


fn to_json_impl(key: String, val: &'static str) -> Json {
    Json::String(format!("{}:{}", key, val))
}
