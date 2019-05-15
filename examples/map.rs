fn main() {
    let i:Vec<_> = (1..10).into_iter().map(u8::from).collect();
    println!("{:?}", i);
}