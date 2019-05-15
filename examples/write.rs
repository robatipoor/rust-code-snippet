use std::io::Write;

fn main() {
    std::io::stdout().write(b"\r Hi!").unwrap();
}
