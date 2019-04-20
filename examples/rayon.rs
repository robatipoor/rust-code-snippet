extern crate rayon;

use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let instant = Instant::now();
    let v = (1..1000_000).collect::<Vec<i32>>();
    let _result = v
        .par_iter()
        .map(|x| x * 3)
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>();
    println!("{:?}", instant.elapsed())
}
