fn main() {
    let s1 = String::from("Yes");
    let s2 = "No";
    print_str(s1);
    print_str(s2);
}

fn print_str<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref());
}
