use std::ops::Deref;

struct MyString<'a>(&'a str);

impl<'a> Deref for MyString<'a> {
    type Target = &'a str;
    /// Dereferences the value.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let my_string = MyString("some string ....");
    let s: &str = &my_string; // or &*my_string;
    println!("{}", s);
}
