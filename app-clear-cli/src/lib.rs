use std::path::Path;
use walkdir::WalkDir;

fn find_all_mentions_of(root: &String, expected_entry: &str) -> Vec<String> {
    WalkDir::new(root)
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_name() == expected_entry)
        .map(|entry| entry.path().display().to_string())
        .collect()
}

fn find_all_package_locks(root: &String) -> Vec<String> {
    find_all_mentions_of(root, "package-lock.json")
}

fn find_all_node_modules(root: &String) -> Vec<String> {
    find_all_mentions_of(root, "node_modules")
}

// TODO: sub-optimal. Currently too many iterations. Make it a single iteration.
fn find_all_front_end_junk(root: &String) -> Vec<String> {
    let mut front_end_items = find_all_package_locks(&root);
    front_end_items.append(&mut find_all_node_modules(&root));

    front_end_items
}

// This tests are more like integration tests, but for now I need get into stubs and mock in Rust to change it.
// In integration tests I want to test full flow, so I will keep this tests as unit for now
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_has_lock_files_should_return_all_mentions() {
        let root = String::from(".");
        let expected = Path::new(".")
            .join("test-root")
            .join("package-lock.json")
            .display()
            .to_string();

        let actual = find_all_package_locks(&root);

        assert_eq!(actual, vec![expected]);
    }

    #[test]
    fn when_has_node_modules_should_return_all_mentions() {
        let root = String::from(".");
        let expected = Path::new(".")
            .join("test-root")
            .join("node_modules")
            .display()
            .to_string();

        let actual = find_all_node_modules(&root);

        assert_eq!(actual, vec![expected]);
    }

    #[test]
    fn when_js_junk_should_return_all_mentions() {
        let root = String::from(".");

        let mut actual = find_all_front_end_junk(&root);

        let mut expected = vec![
            Path::new(".")
                .join("test-root")
                .join("node_modules")
                .display()
                .to_string(),
            Path::new(".")
                .join("test-root")
                .join("package-lock.json")
                .display()
                .to_string(),
        ];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }
}
