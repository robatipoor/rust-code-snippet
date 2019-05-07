fn main() {
    let s = "Rust is a multi-paradigm
    systems programming language
    focused on safety
    "
    .to_owned();
    // let lines: Vec<&str> = s.split('\n').map(|s| s.trim()).collect();
    let lines: Vec<&str> = s.lines().map(|s| s.trim()).collect();
    for line in lines {
        println!("{}", line);
    }
}
