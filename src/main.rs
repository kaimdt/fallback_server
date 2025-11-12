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
    // Fehler als einfache Textantwort, aber als JSON an den Client senden
    let error_response = ErrorResponse {
        error: "Service Unavailable".to_string(),
        message: "The requested service is temporarily unavailable. Please try again later.".to_string(),
        status: 503,
    };

    // Die JSON-Antwort zurückgeben
    (StatusCode::SERVICE_UNAVAILABLE, Json(error_response))
}

#[tokio::main]
async fn main() {
    // Optional: Tokio-Thread-Pool explizit konfigurieren (für hohe Last)
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)  // Anzahl der Worker anpassen je nach Bedarf
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(run_server());
}

async fn run_server() {
    // Router mit Fallback-Handler
    let app = Router::new().fallback(fallback_handler);

    // Port aus Umgebungsvariablen oder Default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Kein Logging: Keine println! Ausgaben oder ähnliche Logs

    // Server starten
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
