use std::error::Error;
use std::fmt;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, Number};

#[derive(Debug)]
pub enum ParserError {
    InvalidValue(String),
    DeserializeError(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            ParserError::DeserializeError(msg) => write!(f, "Deserialization error: {}", msg),
        }
    }
}

impl Error for ParserError {}

fn try_convert_value(value: &str) -> Value {
    // Try to convert to number first
    if let Ok(num) = value.parse::<i64>() {
        return Value::Number(Number::from(num));
    }
    if let Ok(num) = value.parse::<u64>() {
        return Value::Number(Number::from(num));
    }
    if let Ok(num) = value.parse::<f64>() {
        if let Some(num) = Number::from_f64(num) {
            return Value::Number(num);
        }
    }
    
    // Try boolean
    match value.to_lowercase().as_str() {
        "true" => return Value::Bool(true),
        "false" => return Value::Bool(false),
        _ => {}
    }
    
    // Default to string if no other type matches
    Value::String(value.to_string())
}

/// Converts an iterator of key-value pairs into a nested structure.
///
/// # Arguments
///
/// * `iter` - An iterator yielding (String, String) pairs representing environment variables
///
/// # Type Parameters
///
/// * `T` - The target type to deserialize into
/// * `I` - The iterator type
///
/// # Returns
///
/// Returns `Result<T, ParserError>` where T is the deserialized structure
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     database: DatabaseConfig,
/// }
///
/// #[derive(Deserialize)]
/// struct DatabaseConfig {
///     host: String,
///     port: String,
/// }
///
/// let env_vars = vec![
///     ("DATABASE__HOST".to_string(), "localhost".to_string()),
///     ("DATABASE__PORT".to_string(), "5432".to_string()),
/// ];
///
/// let config: Config = from_iter(env_vars).unwrap();
/// ```
pub fn from_iter<T, I>(iter: I) -> Result<T, ParserError>
where
    T: DeserializeOwned,
    I: IntoIterator<Item = (String, String)>
{
    let mut json_map = Map::new();

    for (key, value) in iter {
        let parts: Vec<&str> = key.split("__").collect();
        
        let mut current = &mut json_map;
        
        for part in parts.iter().take(parts.len() - 1) {
            let lower_part = part.to_ascii_lowercase();
            
            if let Some(existing) = current.get(&lower_part) {
                if !existing.is_object() {
                    return Err(ParserError::InvalidValue(
                        format!("Invalid nesting: {} is not an object", part)
                    ));
                }
            }
            
            current = current
                .entry(&lower_part)
                .or_insert(Value::Object(Map::new()))
                .as_object_mut()
                .ok_or_else(|| ParserError::InvalidValue(
                    format!("Failed to create nested structure at {}", part)
                ))?;
        }
        
        if let Some(last_part) = parts.last() {
            let lower_last = last_part.to_ascii_lowercase();
            
            if let Some(existing) = current.get(&lower_last) {
                if existing.is_object() {
                    return Err(ParserError::InvalidValue(
                        format!("Attempted to set value on object at {}", last_part)
                    ));
                }
            }
            
            current.insert(lower_last, try_convert_value(&value));
        }
    }

    serde_json::from_value(Value::Object(json_map))
        .map_err(|e| ParserError::DeserializeError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        database: DatabaseConfig,
        server: ServerConfig,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct DatabaseConfig {
        host: String,
        port: u16,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct ServerConfig {
        host: String,
    }

    #[test]
    fn test_basic_parsing() {
        let env_vars = vec![
            ("DATABASE__HOST".to_string(), "localhost".to_string()),
            ("DATABASE__PORT".to_string(), "5432".to_string()),
            ("SERVER__HOST".to_string(), "127.0.0.1".to_string()),
        ];

        let config: TestConfig = from_iter(env_vars).unwrap();
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.server.host, "127.0.0.1");
    }

    #[test]
    fn test_case_insensitive_keys() {
        let env_vars = vec![
            ("DATABASE__HOST".to_string(), "localhost".to_string()),
            ("database__PORT".to_string(), "5432".to_string()),
            ("SERVER__HOST".to_string(), "127.0.0.1".to_string()),
        ];

        let config: TestConfig = from_iter(env_vars).unwrap();
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
    }

    #[test]
    fn test_deserialize_error() {
        #[derive(Debug, Deserialize)]
        struct StrictConfig {
            port: u16,
        }

        let env_vars = vec![("PORT".to_string(), "invalid".to_string())];

        let result: Result<StrictConfig, _> = from_iter(env_vars);
        assert!(matches!(result, Err(ParserError::DeserializeError(_))));
    }



    #[test]
    fn test_boolean_conversion() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct BoolConfig {
            enabled: bool,
        }

        let vars = vec![
            ("ENABLED".to_string(), "true".to_string()),
        ];

        let config: BoolConfig = from_iter(vars).unwrap();
        assert!(config.enabled);
    }

    #[test]
    fn test_number_conversion() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct NumConfig {
            integer: i32,
            unsigned: u32,
            float: f64,
        }

        let vars = vec![
            ("INTEGER".to_string(), "-42".to_string()),
            ("UNSIGNED".to_string(), "42".to_string()),
            ("FLOAT".to_string(), "42.5".to_string()),
        ];

        let config: NumConfig = from_iter(vars).unwrap();
        assert_eq!(config.integer, -42);
        assert_eq!(config.unsigned, 42);
        assert_eq!(config.float, 42.5);
    }

}