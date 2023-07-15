use crate::domain::commands::ReservationCommand;
use crate::metadata_extractor::MetadataExtractor;
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use cqrs_es::persist::ViewRepository;

// Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested reservation.
pub(crate) async fn query_handler(
    State(state): State<ApplicationState>,
    Path(reservation_id): Path<String>,
) -> Response {
    let view = match state.view_repo.load(&reservation_id).await {
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

pub(crate) async fn make_reservation_handler(
    State(state): State<ApplicationState>,
    Path(reservation_id): Path<String>,
    MetadataExtractor(metadata): MetadataExtractor,
    Json(command): Json<ReservationCommand>,
) -> Response {
    match state
        .cqrs
        .execute_with_metadata(&reservation_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub(crate) async fn cancel_reservation_handler(
    State(state): State<ApplicationState>,
    Path(reservation_id): Path<String>,
    MetadataExtractor(metadata): MetadataExtractor,
) -> Response {
    match state
        .cqrs
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
