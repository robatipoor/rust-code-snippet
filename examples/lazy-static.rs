#[macro_use]
extern crate lazy_static;

use std::ops::Deref;

lazy_static! {
    static ref STATUS: i32 = get_status();
}

fn get_status() -> i32 {
    println!("{}", "Function Return Status");
    return 0;
}

fn main() {
    println!("{}","Start Program");
    let _ = STATUS.deref();
    println!("{}","End Program");
}
