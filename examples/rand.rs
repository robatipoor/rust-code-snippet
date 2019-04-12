extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen::<u32>());
    println!("{}", rng.gen_range(0, 10));
}
