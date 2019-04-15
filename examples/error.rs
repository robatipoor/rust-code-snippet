use std::error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let re = read_file("not exist file");
    println!("{:?}", re);
}

fn read_file(path: &str) -> Result<String, Box<dyn error::Error>> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
