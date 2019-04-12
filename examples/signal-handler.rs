extern crate graceful;

use graceful::SignalGuard;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let signal = SignalGuard::new();

    thread::spawn(move || {
        let mut i = 0;
        loop {
            println!("LOOP = {}", i);
            i += 1;
            sleep(Duration::from_secs(1));
        }
    });

    signal.at_exit(move |_sig| {
        println!("{}", "✋ ✋ ✋   Bye ...");
        std::process::exit(0);
    });
}
