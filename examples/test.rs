fn main() {
    
}

fn _plus(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn plus_test() {
    assert_eq!(_plus(10, 20), 30);
    assert_ne!(_plus(10, 20), 32);
}
