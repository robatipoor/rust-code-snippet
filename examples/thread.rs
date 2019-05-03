use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![0]));
    let mut th = Vec::new();
    for i in 1..=30 {
        let cloned = v.clone();
        th.push(
            thread::spawn(move || {
            cloned.lock().unwrap().push(i);
        }));
    }
    for t in th {
        t.join().unwrap();
    }
    println!("{:?}", v);
}
