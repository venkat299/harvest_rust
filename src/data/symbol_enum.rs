use rustc_serialize::json::{ToJson, Json};

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum Symbol {
    YESBANK,
    SBIN,
}
impl Symbol {
    fn get_str(&self) -> &'static str {
        match *self {
            Symbol::YESBANK => "YESBANK",
            Symbol::SBIN => "SBIN",
        }
    }
}

impl ToJson for Symbol {
    fn to_json(&self) -> Json {
        to_json_impl("product".to_string(), self.get_str())
    }
}

fn to_json_impl(key: String, val: &'static str) -> Json {
    Json::String(format!("{}:{}", key, val))
}
