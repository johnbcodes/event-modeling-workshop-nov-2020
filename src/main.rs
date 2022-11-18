#![forbid(unsafe_code)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]

use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use cqrs_es::persist::ViewRepository;
use sqlite_es::{default_sqlite_pool, SqliteCqrs, SqliteViewRepository};

use crate::config::cqrs_framework;
use crate::domain::aggregate::Reservation;
use crate::domain::commands::ReservationCommand;
use crate::metadata_extension::MetadataExtension;
use crate::queries::ReservationView;

mod config;
mod domain;
mod metadata_extension;
mod queries;

#[tokio::main]
async fn main() {
    // Configure the CQRS framework, backed by an SQLite database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let pool = default_sqlite_pool("sqlite://demo.db").await;
    sqlx::migrate!().run(&pool).await.unwrap();

    let (cqrs, reservation_query) = cqrs_framework(pool);

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
        .layer(Extension(cqrs))
        .layer(Extension(reservation_query));

    // Start the Axum server.
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested reservation.
async fn query_handler(
    Path(reservation_id): Path<String>,
    Extension(view_repo): Extension<Arc<SqliteViewRepository<ReservationView, Reservation>>>,
) -> Response {
    let view = match view_repo.load(&reservation_id).await {
        Ok(view) => view,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };
    match view {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(reservation_view) => (StatusCode::OK, Json(reservation_view)).into_response(),
    }
}

async fn make_reservation_handler(
    Path(reservation_id): Path<String>,
    Json(command): Json<ReservationCommand>,
    Extension(cqrs): Extension<Arc<SqliteCqrs<Reservation>>>,
    MetadataExtension(metadata): MetadataExtension,
) -> Response {
    match cqrs
        .execute_with_metadata(&reservation_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn cancel_reservation_handler(
    Path(reservation_id): Path<String>,
    Extension(cqrs): Extension<Arc<SqliteCqrs<Reservation>>>,
    MetadataExtension(metadata): MetadataExtension,
) -> Response {
    match cqrs
        .execute_with_metadata(
            &reservation_id,
            ReservationCommand::CancelReservation,
            metadata,
        )
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
