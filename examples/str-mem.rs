fn main() {
    let _s1 = "In Static Memory Storage";
    let _s2 = &*String::from("In Heap Memory Storage");
    let _s3 = std::str::from_utf8(b"In Stack Momory Storage");
}