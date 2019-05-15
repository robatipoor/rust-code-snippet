fn main() {
    let v = vec!["Hello World", "Rust Language"];
    let ss = v.iter().flat_map(|x| x.split(" ")).collect::<Vec<&str>>();
    println!("{:?}", ss);
}
