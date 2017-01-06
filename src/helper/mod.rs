// read config file
use std::fs::File;
use std::io::Read;
// read_config

pub fn read_file(filename: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(filename).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
    data

}
