#[macro_use]
extern crate strum_macros;
extern crate strum;

use strum::IntoEnumIterator;

#[derive(Display,EnumIter)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    for color in Color::iter(){
        println!("My favorite color is {}", color);
    }
}
