use std::path::Path;
use std::error::Error;

use crate::config::Config;
use crate::utils::fs::FileSystem;
use crate::utils::merge::Merge;

/// Configuration source that loads settings from a YAML file
pub struct YamlConfigSource<FS: FileSystem> {
    /// Path to the YAML configuration file
    path: String,
    /// Filesystem implementation to use for reading the file
    fs: FS,
}

impl<FS: FileSystem> YamlConfigSource<FS> {
    /// Creates a new YAML configuration source
    /// 
    /// # Arguments
    /// * `path` - Path to the YAML configuration file
    /// * `fs` - Filesystem implementation to use
    pub fn new<P: Into<String>>(path: P, fs: FS) -> Self {
        Self {
            path: path.into(),
            fs,
        }
    }
}

impl<FS: FileSystem> super::ConfigSource for YamlConfigSource<FS> {
    /// Loads and applies YAML configuration to the provided Config instance
    /// 
    /// # Arguments
    /// * `config` - Configuration instance to update
    /// 
    /// # Returns
    /// * `Ok(())` if configuration was successfully loaded and applied
    /// * `Err` if file couldn't be read or parsed
    fn apply(&self, config: &mut Config) -> Result<(), Box<dyn Error>> {
        let contents = self.fs.read_to_string(&self.path)?;
        let yaml_config: Config = serde_yaml::from_str(&contents)?;
        config.merge(yaml_config);
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigSource;
    use crate::utils::fs::mock_file_system::MockFileSystem;

    #[test]
    fn test_valid_yaml_config() {
        // Create a mock filesystem with a valid YAML config
        let fs = MockFileSystem::new()
            .with_file("config.yml", r#"
server:
  bind_addr: '0.0.0.0'
  port: 8080
"#);

        let source = YamlConfigSource::new("config.yml", fs);
        let mut config = Config::default();

        // Apply the configuration
        source.apply(&mut config).unwrap();

        // Verify the config was updated
        assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(config.server.port, Some(8080));
    }

    #[test]
    fn test_partial_yaml_config() {
        // Create config with only bind_addr
        let fs = MockFileSystem::new()
            .with_file("config.yml", r#"
    server:
      bind_addr: '0.0.0.0'
    "#);

        let source = YamlConfigSource::new("config.yml", fs);
        let mut config = Config::default();
        let default_port = config.server.port;

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(config.server.port, default_port);
    }

    #[test]
    fn test_missing_file() {
        let fs = MockFileSystem::new();
        let source = YamlConfigSource::new("nonexistent.yml", fs);
        let mut config = Config::default();

        let result = source.apply(&mut config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_yaml() {
        let fs = MockFileSystem::new()
            .with_file("config.yml", "this is not valid yaml");

        let source = YamlConfigSource::new("config.yml", fs);
        let mut config = Config::default();

        let result = source.apply(&mut config);
        assert!(result.is_err());
    }
}