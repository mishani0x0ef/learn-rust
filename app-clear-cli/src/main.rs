mod config;

use crate::config::Config;
use app_clear_cli::{delete_all, find_all_front_end_junk};
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

    let items_to_clean = find_all_front_end_junk(&config.path);

    if items_to_clean.len() < 1 {
        println!("No items to clean. Skipping cleanup process.");
        exit(0);
    }

    println!(
        "Found {} items to clean: {:?}",
        items_to_clean.len(),
        items_to_clean
    );

    delete_all(items_to_clean);

    println!("Cleanup was successfully completed!");
}
