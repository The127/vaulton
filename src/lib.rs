pub mod config;
pub mod utils;

// Re-export main types for easier access
pub use config::{Config, ConfigSource};


#[cfg(test)]
mod test {
    #[test]
    fn foo(){
        assert!(true)
    }
}