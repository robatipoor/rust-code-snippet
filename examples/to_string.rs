use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: "Ali".to_owned(),
        age: 35,
    };
    let s = p.to_string();
    println!("{}", s);
}
