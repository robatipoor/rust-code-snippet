fn main() {
    let strings = ["P", "122", "23", "55", "45", "34", "R"];
    let result: Vec<i32> = strings
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    println!("{:?}", result);
    // same to 
    let result: Vec<i32> = strings
        .into_iter()
        .map(|x| x.parse::<i32>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    println!("{:?}", result);
}
