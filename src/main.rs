use vaulton;

use tokio;

#[tokio::main]
async fn main() {
    let app = vaulton::server::create_server().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}