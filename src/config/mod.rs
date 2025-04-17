mod yaml_config_source;

use crate::utils::merge::Merge;
use serde::Deserialize;
use std::error::Error;

/// Main configuration structure for the Vaulton server
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Server-specific configuration settings
    #[serde(default)]
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
        }
    }
}

// Implement for Config
impl Merge for Config {
    fn merge(&mut self, other: Self) {
        self.server.merge(other.server);
    }
}


/// Configuration for the server's network settings
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    /// The IP address the server will bind to
    ///
    /// Examples:
    /// - "127.0.0.1" for localhost only
    /// - "0.0.0.0" for all interfaces
    pub bind_addr: Option<String>,

    /// The port number the server will listen on
    ///
    /// Valid values are from 1 to 65535, though ports below 1024
    /// typically require root/administrator privileges
    pub port: Option<u16>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: Some("127.0.0.1".to_string()),
            port: Some(3000),
        }
    }
}

// Implement for ServerConfig
impl Merge for ServerConfig {
    fn merge(&mut self, other: Self) {
        self.bind_addr.merge(other.bind_addr);
        self.port.merge(other.port);
    }
}

/// Trait for loading static configuration from different sources
pub trait ConfigSource {
    /// Apply configuration from this source to the given config
    fn apply(&self, config: &mut Config) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.bind_addr, Some("127.0.0.1".to_string()));
        assert_eq!(config.port, Some(3000));
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.server.bind_addr, Some("127.0.0.1".to_string()));
        assert_eq!(config.server.port, Some(3000));
    }

    #[test]
    fn test_config_merge() {
        let mut base = Config::default();
        let other = Config {
            server: ServerConfig {
                bind_addr: Some("0.0.0.0".to_string()),
                port: Some(8080),
            },
        };

        base.merge(other);

        assert_eq!(base.server.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(base.server.port, Some(8080));
    }

    #[test]
    fn test_server_config_merge() {
        let mut base = ServerConfig::default();
        let other = ServerConfig {
            bind_addr: Some("0.0.0.0".to_string()),
            port: None,
        };

        base.merge(other);

        assert_eq!(base.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(base.port, Some(3000));
    }
}
