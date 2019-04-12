fn main() {
    let string = String::from("this is test string");
    let _s1 = string.as_str(); //same to &*
    let _s2 = &*string; //same to as_str
}
