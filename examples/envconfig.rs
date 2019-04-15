#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
struct Config {
    #[envconfig(from = "HOST")]
    host: String,
    #[envconfig(from = "PORT", default = "8080")]
    port: u32,
}

fn main() {
    std::env::set_var("HOST", "127.0.0.1");
    std::env::set_var("PORT", "4000");
    let config = Config::init().unwrap();
    println!("{:?}", config);
}
