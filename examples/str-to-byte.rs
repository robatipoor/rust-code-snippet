use std::str;

fn main() {
    let string = "some string 1";
    let arry_byte = string.as_bytes(); //conver str to array bye u8
    let arry_byte2: &[u8] = b"some string 2";
    println!("{}", str::from_utf8(arry_byte).unwrap()); // convert array bye u8 to &str
    println!("{}", str::from_utf8(arry_byte2).unwrap());
}
