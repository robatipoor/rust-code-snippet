use std::ops::Deref;

#[derive(Debug)]
struct Array<T>(Vec<T>);

impl<T> Deref for Array<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let array = Array(vec![1, 2, 3, 4]);
    println!("{:?}", *array);
}
