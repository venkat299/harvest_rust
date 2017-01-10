extern crate rustc_serialize;

pub mod executor;
pub mod data;
pub mod helper;

#[cfg(test)]
mod test {
    // extern crate subprocess_communicate;
    // use std::process;
    // use std::process::{Command, Stdio, Child};
    // use self::encoding::all::UTF_8;

    // extern crate encoding;
    // use self::encoding::{Encoding, EncoderTrap, DecoderTrap};
    use std::str;
    #[test]
    fn test_subprocess_pipe() {
        use std::process::Command;

        let output = Command::new("/usr/bin/python2.7")
            .arg("/Users/venkat299/code/rust/harvest/src/python/hello.py")
            .output()
            .expect("failed to execute process");

        let hello = output.stdout;
        let out = str::from_utf8(&hello).unwrap();
        println!("{:?}", out);
        assert_eq!(out, "Hello world");
        // assert_eq!(1, 2);
        // assert_eq!(0usize, ret_stderr.len());
    }

}
