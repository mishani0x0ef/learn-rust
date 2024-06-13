use app_clear_cli::config::Config;
use app_clear_cli::run;
use std::{env, process::exit};

fn main() {
    println!("Start cleanup process");

    let args: Vec<String> = env::args().collect();

    let config = match Config::from_args(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error occurred during parsing args: {err}");
            exit(1);
        }
    };

    if let Err(err) = run(config) {
        eprintln!("Error occurred during parsing args: {err}");
        exit(1);
    }

    println!("Cleanup was successfully completed!");
}
