// train , predict
// use std::str;
// // python shell call
// extern crate env_logger;

extern crate rusqlite;
// extern crate time;
// use time::Timespec;
use self::rusqlite::Connection;

// extern crate harvest;
use data::*;

use std::path::Path;
use std::str;
use std::process::Command;

pub fn train(path: &str) {
    // call python scripts which runs training program
    call_script();
    // connecting to sqlite
    let conn = Connection::open(Path::new(path)).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM strategy_stk").unwrap();
    let stock_iter = stmt.query_map(&[], |row| {
            StrategyStk {
                strategy: row.get(0),
                stock: row.get(1),
                norm_score: row.get(2),
                exit: row.get(4),
            }
        })
        .unwrap();

    for stock in stock_iter {
        println!("{:?}", stock);
    }
    // println!("stock_iter {:?}", stock_iter);
    // println!("stock length {:?}", &stock_iter.count());
}


fn call_script() -> String {
    let output = Command::new("/usr/bin/python2.7")
        .arg("/Users/venkat299/code/rust/harvest/src/python/low52d/train_strategy.py")
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    // println!("stdout: {:?}", hello);
    let out = str::from_utf8(&hello).unwrap();
    println!("output: {:?}", out);
    out.to_string()
}


#[cfg(test)]
mod test {
    use std::str;
    use std::process::Command;
    // python shell call
    extern crate env_logger;

    #[test]
    fn low52d_train() {
        let output = Command::new("/usr/bin/python2.7")
            .arg("/Users/venkat299/code/rust/harvest/src/python/low52d/daily.py")
            .arg("YESBANK")
            .output()
            .expect("failed to execute process");

        let hello = output.stdout;
        debug!("{:?}", hello);
        let out = str::from_utf8(&hello).unwrap();
        debug!("{:?}", out);
        debug!("can log from the test too");
        // assert_eq!(out, "Hello world");
        // assert_eq!(1, 2);
        // assert_eq!(0usize, ret_stderr.len());
    }
}
