use crate::config::{Config, ConfigSource};
use crate::config::yaml_config_source::YamlConfigSource;
use crate::config::env_config_source::EnvConfigSource;
use crate::utils::fs::{FileSystem, LocalFileSystem};
use crate::utils::env::{Env, SystemEnv};

/// A builder pattern implementation for creating and configuring `Config` instances.
///
/// # Type Parameters
/// * `FS`: A type that implements the `FileSystem` trait for file operations
/// * `E`: A type that implements the `Env` trait for environment variable operations
pub struct ConfigBuilder<FS: FileSystem, E: Env> {
    config: Config,
    yaml_path: Option<String>,
    fs: FS,
    env: E,
}

/// Provides a default implementation for `ConfigBuilder` using `LocalFileSystem` and `SystemEnv`.
/// This is the most common way to create a new `ConfigBuilder` instance.
impl Default for ConfigBuilder<LocalFileSystem, SystemEnv> {
    fn default() -> Self {
        Self::new(LocalFileSystem::default(), SystemEnv::default())
    }
}

impl<FS: FileSystem, E: Env> ConfigBuilder<FS, E> {
    /// Creates a new `ConfigBuilder` instance with the specified filesystem and environment implementations.
    ///
    /// # Arguments
    /// * `fs` - An implementation of the `FileSystem` trait
    /// * `env` - An implementation of the `Env` trait
    pub fn new(fs: FS, env: E) -> Self {
        Self {
            config: Config::default(),
            yaml_path: None,
            fs,
            env,
        }
    }

    /// Specifies a YAML configuration file to be used for configuration.
    ///
    /// # Arguments
    /// * `path` - The path to the YAML configuration file
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn with_yaml_file<S: Into<String>>(mut self, path: S) -> Self {
        self.yaml_path = Some(path.into());
        self
    }

    /// Builds the final configuration by applying configurations from YAML file (if specified)
    /// and environment variables.
    ///
    /// # Returns
    /// * `Ok(Config)` - The fully configured `Config` instance
    /// * `Err(Box<dyn std::error::Error>)` - If there was an error loading or applying the configuration
    pub fn build(self) -> Result<Config, Box<dyn std::error::Error>> {
        let mut config = self.config;

        // Apply YAML config if path is set
        if let Some(path) = self.yaml_path {
            let yaml_source = YamlConfigSource::new(path, self.fs);
            yaml_source.apply(&mut config)?;
        }

        // Apply environment variables
        let env_source = EnvConfigSource::new(self.env);
        env_source.apply(&mut config)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs::mock_file_system::MockFileSystem;
    use crate::utils::env::test_env::TestEnv;

    #[test]
    fn test_default_config() {
        let builder = ConfigBuilder::new(MockFileSystem::new(), TestEnv::new());
        let config = builder.build().unwrap();
        // Default config should have default values
        assert_eq!(config.server.bind_addr, Some("127.0.0.1".to_string()));
        assert_eq!(config.server.port, Some(3000));
    }

    #[test]
    fn test_yaml_config() {
        let fs = MockFileSystem::new()
            .with_file("config.yaml", "server:\n  bind_addr: '0.0.0.0'\n  port: 8080");

        let builder = ConfigBuilder::new(fs, TestEnv::new())
            .with_yaml_file("config.yaml");

        let config = builder.build().unwrap();
        assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(config.server.port, Some(8080));
    }

    #[test]
    fn test_env_overrides_yaml() {
        let fs = MockFileSystem::new()
            .with_file("config.yaml", "server:\n  bind_addr: '0.0.0.0'\n  port: 8080");

        let env = TestEnv::with_vars([
            ("VAULTON__SERVER__BIND_ADDR".to_string(), "127.0.0.1".to_string()),
            ("VAULTON__SERVER__PORT".to_string(), "9000".to_string()),
        ]);

        let builder = ConfigBuilder::new(fs, env)
            .with_yaml_file("config.yaml");

        let config = builder.build().unwrap();
        assert_eq!(config.server.bind_addr, Some("127.0.0.1".to_string()));
        assert_eq!(config.server.port, Some(9000));
    }

    #[test]
    fn test_missing_yaml_file() {
        let builder = ConfigBuilder::new(MockFileSystem::new(), TestEnv::new())
            .with_yaml_file("nonexistent.yaml");

        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_yaml_content() {
        let fs = MockFileSystem::new()
            .with_file("config.yaml", "invalid: yaml: content: : :");

        let builder = ConfigBuilder::new(fs, TestEnv::new())
            .with_yaml_file("config.yaml");

        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_env_only_config() {
        let env = TestEnv::with_vars([
            ("VAULTON__SERVER__BIND_ADDR".to_string(), "127.0.0.1".to_string()),
            ("VAULTON__SERVER__PORT".to_string(), "9000".to_string()),
        ]);

        let builder = ConfigBuilder::new(MockFileSystem::new(), env);
        let config = builder.build().unwrap();

        assert_eq!(config.server.bind_addr, Some("127.0.0.1".to_string()));
        assert_eq!(config.server.port, Some(9000));
    }
}