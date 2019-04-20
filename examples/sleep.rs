use std::thread::sleep;
use std::time::Duration;

const ONE_SEC: Duration = Duration::from_secs(1);

fn main() {
    sleep(ONE_SEC);
    println!("Finish !");
}
