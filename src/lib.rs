extern crate rustc_serialize;

#[macro_use]
extern crate log;


pub mod executor;
pub mod data;
pub mod helper;
pub mod strategy;

#[cfg(test)]
mod test {
    use std::str;
    // python shell call
    extern crate env_logger;

    #[test]
    fn subprocess_pipe() {
        let _ = env_logger::init();
        use std::process::Command;
        let output = Command::new("/usr/bin/python2.7")
            .arg("/Users/venkat299/code/rust/harvest/src/python/hello.py")
            .output()
            .expect("failed to execute process");

        let hello = output.stdout;
        let out = str::from_utf8(&hello).unwrap();
        debug!("{:?}", out);
        debug!("can log from the test too");
        assert_eq!(out, "Hello world");
        // assert_eq!(1, 2);
        // assert_eq!(0usize, ret_stderr.len());
    }

}
