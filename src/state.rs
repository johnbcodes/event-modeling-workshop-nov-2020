use crate::config::cqrs_framework;
use crate::domain::aggregate::Reservation;
use crate::queries::ReservationView;
use sqlite_es::{default_sqlite_pool, SqliteCqrs, SqliteViewRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<SqliteCqrs<Reservation>>,
    pub view_repo: Arc<SqliteViewRepository<ReservationView, Reservation>>,
}

pub async fn new_application_state() -> ApplicationState {
    // Configure the CQRS framework, backed by an SQLite database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let pool = default_sqlite_pool("sqlite://demo.db").await;
    sqlx::migrate!().run(&pool).await.unwrap();
    let (cqrs, view_repo) = cqrs_framework(pool);

    ApplicationState { cqrs, view_repo }
}
