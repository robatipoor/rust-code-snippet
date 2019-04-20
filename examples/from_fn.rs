fn main() {
    let mut count = 0;
    let counter: Vec<i32> = std::iter::from_fn(move || {
        // Increment our count. This is why we started at zero.
        count += 2;
        // Check to see if we've finished counting or not.
        if count < 10 {
            Some(count)
        } else {
            None
        }
    })
    .collect();
    println!("{:?}", counter);
}
