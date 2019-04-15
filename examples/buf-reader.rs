use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let file = File::open("file.txt").unwrap();
    let read = BufReader::new(file);
    let lines: Vec<String> = read.lines().map(|x| x.unwrap()).collect();
    for line in lines {
        println!("{}", line);
    }
}
