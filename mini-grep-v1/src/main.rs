use std::{env, process};
use mini_grep_v1::{Config,run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_config = Config::new(&args).unwrap_or_else(|err:&str| {
        println!("Problem Parsing config - {}",err);
        process::exit(1);
    });

    if let Err(e) = run(file_config) {
        println!("Application Error -  {}",e);
        process::exit(1);
    }

}
