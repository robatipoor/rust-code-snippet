#[derive(Debug)]
struct Example<'a> {
    inner: &'a str,
}

impl<'a> Example<'a> {
    pub fn test_method(){
        println!("{}","test method");
    }
}

fn main() {
    let s: String = String::from("Ali");
    let example: Example = Example { inner: s.as_str() };
    println!("{:?}", example);
}
