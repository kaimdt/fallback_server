use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::any,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    status: u16,
}

async fn fallback_handler() -> impl IntoResponse {
    let error_response = ErrorResponse {
        error: "Service Unavailable".to_string(),
        message: "The requested service is temporarily unavailable. Please try again later.".to_string(),
        status: 503,
    };
    
    (StatusCode::SERVICE_UNAVAILABLE, Json(error_response))
}

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new()
        .fallback(fallback_handler)
        .route("/*path", any(fallback_handler));

    // Get port from environment variable or use default 8080
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("Fallback server listening on {}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
