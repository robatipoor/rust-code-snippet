#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Person<'a> {
    name: &'a str,
    last_name: &'a str,
    age: i32,
}

fn main() {
    let person = Person {
        name: "Mahdi",
        last_name: "Geekoff",
        age: 30,
    };
    let string = serde_json::to_string(&person).unwrap();
    println!("{}", string);
    let person: Person = serde_json::from_str(&*string).unwrap();
    println!("{:?}", person);
}
