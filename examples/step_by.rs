fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    v.iter().step_by(2).for_each(|x| {
        println!("{}", x);
    });
}
