fn main() {
    let s = "Hello World";
    let ss = s.split(" ").collect::<Vec<&str>>();
    println!("{:?}", ss);
}