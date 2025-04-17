use crate::config::{Config, ConfigMetadata, ConfigSource};
use crate::utils::env::Env;
use crate::utils::merge::Merge;
use std::collections::HashMap;
use std::error::Error;

pub struct EnvConfigSource<E: Env> {
    env: E,
    prefix: String,
}

impl<E: Env> EnvConfigSource<E> {
    pub fn new(env: E) -> Self {
        Self {
            env,
            prefix: "VAULTON_".to_string(),
        }
    }

    fn collect_env_vars(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut vars = HashMap::new();

        // Get all possible config paths
        for path in Config::get_paths() {
            // Convert dot notation to env var format
            let env_key = format!(
                "{}{}",
                self.prefix,
                path.path.replace('.', "_").to_uppercase()
            );

            if let Ok(value) = self.env.get_var(&env_key) {
                vars.insert(path.path, value);
            }
        }

        Ok(vars)
    }
}



impl<E: Env> ConfigSource for EnvConfigSource<E> {
    fn apply(&self, config: &mut Config) -> Result<(), Box<dyn Error>> {
        let vars = self.collect_env_vars()?;
        if vars.is_empty() {
            return Ok(());
        }

        let env_config: Config = envy::from_iter(vars.into_iter())?;
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
        let env = TestEnv::with_vars([(
            "VAULTON_SERVER_BIND_ADDR".to_string(),
            "0.0.0.0".to_string(),
        )]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string()));
    }

    #[test]
    fn test_port_env() {
        let env = TestEnv::with_vars([("VAULTON_SERVER_PORT".to_string(), "9000".to_string())]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.port, Some(9000));
    }

    #[test]
    fn test_multiple_env_vars() {
        let env = TestEnv::with_vars([
            (
                "VAULTON_SERVER_BIND_ADDR".to_string(),
                "0.0.0.0".to_string(),
            ),
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
        let env = TestEnv::with_vars([(
            "VAULTON_SERVER_PORT".to_string(),
            "not_a_number".to_string(),
        )]);

        let source = EnvConfigSource::new(env);
        let mut config = Config::default();
        let original_port = config.server.port;

        source.apply(&mut config).unwrap();

        assert_eq!(config.server.port, original_port);
    }
}
