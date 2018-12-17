extern crate zksnark;
extern crate tiny_keccak;
extern crate itertools;
#[macro_use] extern crate log;

pub mod client;
pub mod common_reference;

use self::client::{Attributes, TargetRange};
use self::common_reference::CommonReference;

fn main () {
    CommonReference::new();
    loop {
        match CommonReference::check_comparator() {
            Ok(true) => {
                let crs = CommonReference::read();
                println!("Found operable CRS.. Breaking loop.");
                break   
            },
            Ok(false) => {
                CommonReference::new();
                println!("Retrying CRS...");
            },
            _ => panic!()
        }
    }
}