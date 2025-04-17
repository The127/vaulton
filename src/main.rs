mod libr;
mod config;
mod fs;
mod utils;

use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn foo(){
        assert!(true)
    }
}