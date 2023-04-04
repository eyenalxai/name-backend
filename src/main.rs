mod handlers;
mod parse;
mod syllables;

use std::net::SocketAddr;

use crate::handlers::{fullnames, username};
use crate::parse::get_syllables_from_json;
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let syllables = get_syllables_from_json(include_str!("../syllables.json"));

    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a valid port number");

    let app = Router::new()
        .route("/usernames", get(username))
        .route("/fullnames", get(fullnames))
        .layer(CorsLayer::permissive())
        .with_state(syllables);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server Error");
}
