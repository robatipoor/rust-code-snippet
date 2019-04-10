fn main() {
    let s = "1234";
    let i: i32 = s.parse().unwrap(); // convert str to i32
    let i2 = s.parse::<i32>().unwrap(); // convert str to i32
    println!("{}", i);
    println!("{}", i2);
}
