extern crate notifica;

use notifica::notify;

fn main() {
    let msg = "Body Message";
    let title = "Title Message";
    notify(msg, title);
}
