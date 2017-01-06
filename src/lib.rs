// pub mod executor  {

// 	pub fn execute() -> String {
//     	"Goodbye.".to_string()
// 	}

// }
extern crate rustc_serialize;

pub mod executor;
pub mod data;
pub mod helper;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
