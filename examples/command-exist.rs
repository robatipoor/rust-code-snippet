use std::env;
use std::fs;
use std::path::Path;

fn command_exist(cmd: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, cmd);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn main() {
    assert_eq!(true,  command_exist("rustc"))
}
