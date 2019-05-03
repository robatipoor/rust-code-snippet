fn main() {
    let a: (Vec<u32>, Vec<u32>) = (1..1000)
        .into_iter()
        .partition(|x| *x % 2 == 0);
    for i in a.1 {
        println!("{}", i);
    }
}
