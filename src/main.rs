#[macro_use(lazy_static)]
extern crate lazy_static;


//mod translate;
//use translate::{init, rtr};
mod rtr;
use rtr::rtr;

fn main() {

    rtr::init(&"fr".to_string());
    println!("{}", rtr("hello"));
    println!("{}", rtr("worlds"));
}