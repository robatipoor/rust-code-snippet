fn main() {
    let s = String::from("Hello");
    get_str(s);
    get_str("Hi");
}

fn get_str<T: Into<String>>(s: T) -> String {
    s.into()
}
