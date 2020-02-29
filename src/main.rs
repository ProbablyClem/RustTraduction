#[macro_use(lazy_static)]
extern crate lazy_static;

mod rtr;
use crate::rtr::init;
use rtr::rtr;

fn main() {
    init(&"fr".to_string());
    println!("{}", rtr("hello"));
    println!("{}", rtr("worlds"));
}
