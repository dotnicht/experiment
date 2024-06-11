use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

//mod grpc;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    println!("Request received\nSending response.");
    Html("<h1>das experiment ebobo</h1>")
}
