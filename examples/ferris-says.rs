extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout());
    let s = "Hello Rust World!".as_bytes();
    say(s, 10, &mut out).unwrap();
}
