use prc1::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing program : {}", err);
        process::exit(1);
    });
    println!("Query: param:{:?}", config.query);
    println!("FileName : {:?}", config.filename);
    if let Err(e) = prc1::run(config) {
        println!("Application Error:{}", e);
        process::exit(1);
    }
}
