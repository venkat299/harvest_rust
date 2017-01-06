// read config file
use std::fs::File;
use std::io::{Read, BufReader};
use std::collections::BTreeMap;
extern crate toml;
use helper::toml::Value;
// read_config

pub fn read_file(abs_path: &str) -> String {
    let mut data = String::new();
    // println!("trying to open file at :{:?}", &abs_path);
    let mut f = File::open(abs_path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    // println!("{}", data);
    data
}

pub fn read_file_buff(abs_path: &str) -> String {
    let mut data = String::new();
    // println!("trying to open file at :{:?}", &abs_path);
    let f = File::open(abs_path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    // println!("{}", data);
    data

}

pub fn read_config(abs_path: &str) -> BTreeMap<String, Value> {
    let config_input = read_file_buff(&abs_path);
    let mut parser = toml::Parser::new(&config_input);
    parser.parse().unwrap()
}
