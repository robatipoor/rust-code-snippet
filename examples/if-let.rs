fn main() {
    if let Err(e) = inner() {
        eprintln!("Error {}", e);
    };
}

fn inner() -> Result<(), &'static str> {
    Err("Failed !")
}
