use axum::{Router, routing::get, Json}; // Axum handles routing & web requests
use std::net::SocketAddr; // For defining server address
use tokio; // Asynchronous runtime for handling async tasks
use tokio::net::TcpListener; // For creating a TCP listener

#[tokio::main] // Marks this function as async so Rust can run it properly
async fn main() {
    // Create a Router (like a map for directing web requests)
    let app = Router::new()
        .route("/", get(root)) // Route for the home page "/"
        .route("/hello", get(hello)); // Route for "/hello" which returns JSON

    // Define server address (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr); // Print server URL to console

    // ✅ Alternative way to start the server using Axum
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Function that handles requests to "/"
async fn root() -> &'static str {
    "Welcome to my Rust Web App!" // Returns a simple text response
}

// Function that handles requests to "/hello"
async fn hello() -> Json<serde_json::Value> {
    // Create a JSON response
    let data = serde_json::json!({ "message": "Hello, world!" });
    Json(data) // Return JSON response
}