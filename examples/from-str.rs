use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl FromStr for Person {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<String> = s.split_whitespace().map(String::from).collect();
        let p: Person;
        if v.len() != 3 {
            return Err("parse str failed");
        } else {
            p = Person {
                name: v.get(0).unwrap().clone(),
                last_name: v.get(1).unwrap().clone(),
                age: v.get(2).unwrap().parse().unwrap(),
            };
        }
        Ok(p)
    }
}

fn main() {
    let ali: Person = "ali abedi 34".parse().unwrap();
    let hassan: Person = "hassan aboli 22".parse().unwrap();
    let kazem: Person = "kazem kar 45".parse().unwrap();
    println!("{:?}", ali);
    println!("{:?}", hassan);
    println!("{:?}", kazem);
}
