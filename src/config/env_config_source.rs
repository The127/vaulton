use std::error::Error;
use crate::config::{Config, ConfigSource, ServerConfig};
use crate::utils::env::Env;
use crate::utils::merge::Merge;

/// Configuration source that loads settings from environment variables
/// Environment variables are expected to be in the format:
/// - VAULTON_SERVER_BIND_ADDR
/// - VAULTON_SERVER_PORT
pub struct EnvConfigSource<E: Env> {
    /// Environment implementation to use
    env: E,
}

impl<E: Env> EnvConfigSource<E> {
    /// Creates a new environment configuration source with the provided environment implementation
    pub fn new(env: E) -> Self {
        Self { env }
    }
}

impl<E: Env> ConfigSource for EnvConfigSource<E> {
    /// Apply environment configuration to the given config instance
    ///
    /// # Arguments
    /// * `config` - Configuration instance to update
    ///
    /// # Returns
    /// * `Ok(())` if configuration was successfully loaded and applied
    /// * `Err` if environment variables couldn't be read
    fn apply(&self, config: &mut Config) -> Result<(), Box<dyn Error>> {
        let mut env_config = Config::default();

        if let Ok(addr) = self.env.get_var("VAULTON_SERVER_BIND_ADDR") {
            env_config.server.bind_addr = Some(addr);
        }

        if let Ok(port_str) = self.env.get_var("VAULTON_SERVER_PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                env_config.server.port = Some(port);
            }
        }

        config.merge(env_config);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::env::test_env::TestEnv;

    #[test]
    fn test_empty_env() {
        let env = TestEnv::new();
        let source = EnvConfigSource::new(env);
        let mut config = Config::default();
        let original_bind_addr = config.server.bind_addr.clone();
        let original_port = config.server.port;

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.bind_addr, original_bind_addr);
        assert_eq!(config.server.port, original_port);
    }

    #[test]
    fn test_bind_addr_env() {
        let env = TestEnv::with_vars([
            ("VAULTON_SERVER_BIND_ADDR".to_string(), "0.0.0.0".to_string()),
        ]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string()));
    }

    #[test]
    fn test_port_env() {
        let env = TestEnv::with_vars([
            ("VAULTON_SERVER_PORT".to_string(), "9000".to_string()),
        ]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.port, Some(9000));
    }

    #[test]
    fn test_multiple_env_vars() {
        let env = TestEnv::with_vars([
            ("VAULTON_SERVER_BIND_ADDR".to_string(), "0.0.0.0".to_string()),
            ("VAULTON_SERVER_PORT".to_string(), "9000".to_string()),
        ]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(config.server.port, Some(9000));
    }

    #[test]
    fn test_invalid_port() {
        let env = TestEnv::with_vars([
            ("VAULTON_SERVER_PORT".to_string(), "not_a_number".to_string()),
        ]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();
        let original_port = config.server.port;

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.port, original_port);
    }
}