fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        println!("{}", arg);
    }
}
