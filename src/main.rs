#![forbid(unsafe_code)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]

use crate::route_handler::{cancel_reservation_handler, make_reservation_handler, query_handler};
use crate::state::new_application_state;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;

mod config;
mod domain;
mod metadata_extractor;
mod queries;
mod route_handler;
mod state;

#[tokio::main]
async fn main() {
    let state = new_application_state().await;

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/reservation/make/:reservation_id",
            post(make_reservation_handler),
        )
        .route(
            "/reservation/cancel/:reservation_id",
            post(cancel_reservation_handler),
        )
        .route("/reservation/:reservation_id", get(query_handler))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
