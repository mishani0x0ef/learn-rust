use std::{collections::BTreeSet, path::Path};
use walkdir::WalkDir;

fn find_all_mentions_of(root: &String, expected_entries: Vec<&str>) -> Vec<String> {
    let entries_set: BTreeSet<_> = expected_entries.into_iter().collect();

    WalkDir::new(root)
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entries_set.contains(entry.file_name().to_str().unwrap()))
        .map(|entry| entry.path().display().to_string())
        .collect()
}

fn find_all_package_locks(root: &String) -> Vec<String> {
    find_all_mentions_of(root, vec!["package-lock.json"])
}

fn find_all_node_modules(root: &String) -> Vec<String> {
    find_all_mentions_of(root, vec!["node_modules"])
}

fn find_all_front_end_junk(root: &String) -> Vec<String> {
    find_all_mentions_of(root, vec!["node_modules", "package-lock.json"])
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
