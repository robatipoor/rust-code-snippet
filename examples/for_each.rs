fn main() {
    let v = vec![1, 2, 3, 4, 5];
    v.iter().for_each(|x| {
        println!("{}", x);
    });
}
