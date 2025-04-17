pub mod mock_file_system;

use std::path::Path;
use std::io;

/// Provides a common interface for filesystem operations.
/// Currently focused on read operations to support configuration loading.
pub trait FileSystem: Send + Sync {
    /// Reads a file into a string.
    /// 
    /// The path can be any type that can be converted into a Path.
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String>;
}

/// Implementation of FileSystem that uses the local filesystem.
/// 
/// This struct is zero-sized as it only provides static methods,
/// making it efficient to copy and pass around.
#[derive(Debug, Clone, Default)]
pub struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        // Delegate to std::fs for actual implementation
        // We could add additional logging or metrics here if needed
        std::fs::read_to_string(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_existing_file() {
        // Create a temporary file that's automatically cleaned up
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "test content").unwrap();

        let fs = LocalFileSystem::default();
        let content = fs.read_to_string(file.path()).unwrap();
        
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_read_nonexistent_file() {
        let fs = LocalFileSystem::default();
        let result = fs.read_to_string("definitely_not_exists.txt");
        
        // Ensure we get the expected error kind for missing files
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }
}