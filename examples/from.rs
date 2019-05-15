#[derive(Debug)]
struct User {
    name: String,
}

impl From<String> for User {
    fn from(t: String) -> Self {
        User { name: t }
    }
}
impl<'a> From<&'a str> for User {
    fn from(t: &'a str) -> Self {
        User { name: t.to_owned() }
    }
}

fn main() {
    let _saman = User::from("saman");
    let _ali: User = String::from("ali").into();
}
