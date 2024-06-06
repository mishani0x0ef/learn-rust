use app_clear_cli::{delete_all, find_all_front_end_junk};
use std::process::Command;

const ROOT_DIR: &str = "./test-root";

fn install_packages() {
    #[cfg(windows)]
    const NPM: &str = "npm.cmd";
    #[cfg(not(windows))]
    const NPM: &str = "npm";

    Command::new(NPM)
        .arg("install")
        .current_dir(ROOT_DIR)
        .output()
        .expect("Failed to execute npm install");
}

#[test]
fn ensure_deleted_correct_files() {
    install_packages();

    let dir = String::from(ROOT_DIR);

    delete_all(find_all_front_end_junk(&dir));

    let junk_left = find_all_front_end_junk(&dir);

    install_packages();

    assert_eq!(junk_left.len(), 0);
}
