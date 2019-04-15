fn main() {
    let number = 1000_000_000;
    println!("{}", length_integer(number));
}

fn length_integer(i: i32) -> usize {
    return i.to_string().len();
}
