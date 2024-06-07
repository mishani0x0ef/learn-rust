use app_clear_cli::{delete_all, find_all_front_end_junk};
use std::{env, path::Path, process::exit};

fn main() {
    println!("Start cleanup process");

    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(path) => Path::new(path),
        None => {
            eprintln!("Usage: {} <path-to-dir>", args[0]);
            exit(1);
        }
    };

    if !path.exists() || !path.is_dir() {
        eprintln!(
            "Dir does not exists. Make sure to provide proper path to folder to clean: {:?}",
            path
        );
        exit(1);
    }

    let path_to_clean = path.display().to_string();
    let items_to_clean = find_all_front_end_junk(&path_to_clean);

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
