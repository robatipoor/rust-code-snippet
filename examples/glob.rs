extern crate glob;

fn main() {
    for entry in glob::glob("./**/*.rs").unwrap(){
        println!("{:?}", entry.unwrap());
    }
}