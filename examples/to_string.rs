use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "{} {}", self.name, self.age)?;
        Ok(())
    }
}

fn main() {
    let p = Person {
        name: "Ali".to_owned(),
        age: 35,
    };
    let pp = p.to_string();
    println!("{}", pp);
}
