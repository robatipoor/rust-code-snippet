extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
            .unwrap();
}

fn main() {
    println!("{}", EMAIL_REGEX.is_match("test@gmail.com"));
}
