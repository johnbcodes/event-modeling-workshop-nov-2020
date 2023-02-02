use std::sync::Arc;

use cqrs_es::Query;
use sqlite_es::{SqliteCqrs, SqliteViewRepository};
use sqlx::{Pool, Sqlite};

use crate::domain::aggregate::Reservation;
use crate::queries::{ReservationQuery, ReservationView, SimpleLoggingQuery};

pub fn cqrs_framework(
    pool: Pool<Sqlite>,
) -> (
    Arc<SqliteCqrs<Reservation>>,
    Arc<SqliteViewRepository<ReservationView, Reservation>>,
) {
    // A very simple query that writes each event to stdout.
    let simple_query = SimpleLoggingQuery {};

    // A query that stores the current state of an individual reservation.
    let reservation_view_repo =
        Arc::new(SqliteViewRepository::new("reservation_query", pool.clone()));
    let mut reservation_query = ReservationQuery::new(reservation_view_repo.clone());

    // Without a query error handler there will be no indication if an
    // error occurs (e.g., database connection failure, missing columns or table).
    // Consider logging an error or panicking in your own application.
    reservation_query.use_error_handler(Box::new(|e| println!("{e}")));

    // Create and return an event-sourced `CqrsFramework`.
    let queries: Vec<Box<dyn Query<Reservation>>> =
        vec![Box::new(simple_query), Box::new(reservation_query)];
    (
        Arc::new(sqlite_es::sqlite_cqrs(pool, queries, ())),
        reservation_view_repo,
    )
}
