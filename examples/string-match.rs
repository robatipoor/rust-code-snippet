fn main() {
    let s = format!("{} programming language", "python");
    match &s {
        r if r.starts_with("rust") => {
            println!("{} is Fun", r);
        }
        p if p.starts_with("python") => {
            println!("{} is so Socks", p);
        }
        _ => {
            println!("{}", "Default");
        }
    }
}
