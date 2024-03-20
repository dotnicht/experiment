use axum::{handler::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new().route("/", get(handler));

    // Define the handler function
    async fn handler() -> &'static str {
        "Hello, world!"
    }

    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}