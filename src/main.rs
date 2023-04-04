mod handlers;
mod parse;
mod syllables;

use std::net::SocketAddr;

use crate::handlers::{names, username};
use crate::parse::get_syllables_from_json;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let syllables = get_syllables_from_json(include_str!("../syllables.json"));

    let app = Router::new()
        .route("/usernames", get(username))
        .route("/names", get(names))
        .with_state(syllables);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server Error");
}
