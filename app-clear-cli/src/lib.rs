use std::{collections::BTreeSet, fs, path::Path};
use walkdir::WalkDir;

pub enum DeletionResult {
    OK(String),
    NotFound(String),
    NotRemoved(String),
}

impl DeletionResult {
    fn ok(path: &Path) -> Self {
        DeletionResult::OK(path.display().to_string())
    }

    fn not_found(path: &Path) -> Self {
        DeletionResult::NotFound(path.display().to_string())
    }

    fn not_removed(path: &Path) -> Self {
        DeletionResult::NotRemoved(path.display().to_string())
    }
}

fn find_all_mentions_of(root: &String, expected_entries: Vec<&str>) -> Vec<String> {
    let entries_set: BTreeSet<_> = expected_entries.into_iter().collect();

    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entries_set.contains(entry.file_name().to_str().unwrap()))
        .map(|entry| entry.path().display().to_string())
        .collect()
}

pub fn find_all_front_end_junk(root: &String) -> Vec<String> {
    find_all_mentions_of(root, vec!["node_modules", "package-lock.json", "yarn.lock"])
}

pub fn delete_all(paths: Vec<String>) -> Vec<DeletionResult> {
    paths
        .iter()
        .map(|path| Path::new(path))
        .map(|path| {
            if !path.exists() {
                return DeletionResult::not_found(path);
            }

            let delete = if path.is_dir() {
                fs::remove_dir_all
            } else {
                fs::remove_file
            };

            match delete(path) {
                Ok(_) => DeletionResult::ok(path),
                Err(_) => DeletionResult::not_removed(path),
            }
        })
        .collect()
}

// This tests are more like integration tests, but for now I need get into stubs and mock in Rust to change it.
// In integration tests I want to test full flow, so I will keep this tests as unit for now
#[cfg(test)]
mod tests {
    use super::*;

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
