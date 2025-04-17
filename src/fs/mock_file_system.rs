use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::fs::FileSystem;

/// A mock filesystem implementation for testing purposes.
/// Stores files and their contents in memory using a HashMap.
pub struct MockFileSystem {
    /// Maps file paths to their contents. Directory entries are represented
    /// as empty strings.
    files: HashMap<PathBuf, String>,
}

impl MockFileSystem {
    /// Creates a new, empty mock filesystem.
    pub fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    /// Adds a file to the mock filesystem with the specified contents.
    ///
    /// # Arguments
    /// * `path` - Any type that can be converted to a Path
    /// * `contents` - Any type that can be converted to a String
    ///
    /// # Returns
    /// Self for method chaining in builder pattern
    pub fn with_file<P, C>(mut self, path: P, contents: C) -> Self
    where
        P: AsRef<Path>,
        C: Into<String>,
    {
        self.files.insert(path.as_ref().to_path_buf(), contents.into());
        self
    }

    /// Adds a directory to the mock filesystem.
    /// Currently implemented as an empty file, could be enhanced with proper
    /// directory semantics if needed.
    ///
    /// # Arguments
    /// * `path` - Any type that can be converted to a Path
    ///
    /// # Returns
    /// Self for method chaining in builder pattern
    pub fn with_dir<P: AsRef<Path>>(self, path: P) -> Self {
        self.with_file(path, "")
    }

    /// Test helper to check if a path exists in the mock filesystem.
    /// Only available in test configuration.
    #[cfg(test)]
    fn has_path<P: AsRef<Path>>(&self, path: P) -> bool {
        self.files.contains_key(path.as_ref())
    }
}

impl FileSystem for MockFileSystem {
    /// Implements the FileSystem trait by looking up the file contents
    /// in the internal HashMap.
    ///
    /// # Arguments
    /// * `path` - Path to the file to read
    ///
    /// # Returns
    /// * `Ok(String)` containing the file contents if found
    /// * `Err(io::Error)` with NotFound kind if the path doesn't exist
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> std::io::Result<String> {
        self.files
            .get(path.as_ref())
            .cloned()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.as_ref().display())
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_fluent_api() {
        let fs = MockFileSystem::new()
            .with_file("config/app.yml", "key: value")
            .with_file("data.txt", "hello world");

        // Check file contents
        assert_eq!(fs.read_to_string("config/app.yml").unwrap(), "key: value");
        assert_eq!(fs.read_to_string("data.txt").unwrap(), "hello world");
    }

    #[test]
    fn test_nonexistent_file() {
        let fs = MockFileSystem::new()
            .with_file("exists.txt", "content");

        let err = fs.read_to_string("does_not_exist.txt").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_nested_paths() {
        let fs = MockFileSystem::new()
            .with_file("config/env/dev.yml", "environment: development");

        assert_eq!(
            fs.read_to_string("config/env/dev.yml").unwrap(),
            "environment: development"
        );
    }
}