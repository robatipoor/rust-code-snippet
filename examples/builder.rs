#[derive(Debug, Default)]
struct User {
    name: String,
    last: String,
}
impl User {
    fn new() -> Self {
        User::default()
    }
    fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn set_last(mut self, last: String) -> Self {
        self.last = last;
        self
    }
}

fn main() {
    let ali = User::new()
        .set_name("ali".to_owned())
        .set_last("keshavarz".to_owned());
    println!("{:?}", ali);
}
