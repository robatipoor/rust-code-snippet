use std::io::{self, BufRead};

fn main() {
    let content = io::stdin().lock().lines().next().map(|x| match x {
        Ok(c) => c,
        Err(e) => panic!(e),
    });
    println!("{:?}", content);
}