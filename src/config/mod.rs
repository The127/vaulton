pub mod yaml_config_source;
pub mod env_config_source;
pub mod builder;

use std::any::TypeId;
use crate::utils::merge::Merge;
use serde::Deserialize;
use std::error::Error;

use vaulton_derive::ConfigMetadata;

pub trait ConfigMetadata {
    /// Returns a list of all possible config paths and their types
    fn get_paths() -> Vec<ConfigPath<'static>>;
}

pub struct ConfigPath<'a> {
    /// Full path in dot notation (e.g. "server.bind_addr")
    path: String,
    /// Type information for proper parsing
    type_id: TypeId,
    /// Whether the field is optional
    is_optional: bool,
    /// Lifetime of the config path
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> ConfigPath<'a> {
    pub fn new(path: String, type_id: TypeId, is_optional: bool) -> Self {
        Self {
            path,
            type_id,
            is_optional,
            _lifetime: std::marker::PhantomData,
        }
    }
}

/// Main configuration structure for the Vaulton server
#[derive(Debug, Deserialize, ConfigMetadata)]
pub struct Config {
    /// Server-specific configuration settings
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub oidc: OIDCConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            oidc: OIDCConfig::default(),
        }
    }
}

// Implement for Config
impl Merge for Config {
    fn merge(&mut self, other: Self) {
        self.server.merge(other.server);
        self.oidc.merge(other.oidc);
    }
}

/// Configuration for the server's network settings
#[derive(Debug, Deserialize, ConfigMetadata)]
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

/// Configuration for OpenID Connect settings
#[derive(Clone, Debug, Deserialize, ConfigMetadata)]
pub struct OIDCConfig {
    /// The external URL where this server is accessible
    /// This is used for generating URLs in OIDC discovery document
    /// Example: "https://auth.example.com"
    pub external_url: Option<String>,
}

impl Default for OIDCConfig {
    fn default() -> Self {
        Self {
            external_url: Some("http://localhost:3000".to_string()),
        }
    }
}

impl Merge for OIDCConfig {
    fn merge(&mut self, other: Self) {
        self.external_url.merge(other.external_url);
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
    fn test_oidc_config_default() {
        let config = OIDCConfig::default();
        assert_eq!(config.external_url, None);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.server.bind_addr, Some("127.0.0.1".to_string()));
        assert_eq!(config.server.port, Some(3000));
        assert_eq!(config.oidc.external_url, None);
    }

    #[test]
    fn test_config_merge() {
        let mut base = Config::default();
        let other = Config {
            server: ServerConfig {
                bind_addr: Some("0.0.0.0".to_string()),
                port: Some(8080),
            },
            oidc: OIDCConfig {
                external_url: Some("https://example.com".to_string()),
            },
        };

        base.merge(other);

        assert_eq!(base.server.bind_addr, Some("0.0.0.0".to_string()));
        assert_eq!(base.server.port, Some(8080));
        assert_eq!(base.oidc.external_url, Some("https://example.com".to_string()));
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
