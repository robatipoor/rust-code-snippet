extern crate crossterm;

use crossterm::cursor;

use std::io::{prelude::*, stdout};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut count = 0;
    loop {
        print!("{}", count);
        count += 1;
        sleep(Duration::from_secs(1));
        stdout().flush().expect("Could not flush stdout");
        cursor().goto(0, cursor().pos().1).unwrap();
    }
}