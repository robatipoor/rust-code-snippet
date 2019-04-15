#[derive(Debug,PartialEq)]
struct Book<'a> {
    name: &'a str,
    page: i32,
}

fn main() {
    let book1 = Book {
        name: "RustLang",
        page: 55,
    };
    let book2 = Book {
        name: "RustLang",
        page: 55,
    };
    println!("{}", book1 == book2);
}
