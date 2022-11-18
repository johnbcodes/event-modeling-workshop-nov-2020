use async_trait::async_trait;
use cqrs_es::persist::GenericQuery;
use cqrs_es::{EventEnvelope, Query, View};
use serde::{Deserialize, Serialize};
use sqlite_es::SqliteViewRepository;

use crate::domain::aggregate::Reservation;
use crate::domain::events::ReservationEvent;

pub struct SimpleLoggingQuery;

// Our simplest query, this is great for debugging but absolutely useless in production.
// This query just pretty prints the events as they are processed.
#[async_trait]
impl Query<Reservation> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Reservation>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

// Our second query, this one will be handled with an SQLite `GenericQuery`
// which will serialize and persist our view after it is updated. It also
// provides a `load` method to deserialize the view on request.
pub type ReservationQuery =
    GenericQuery<SqliteViewRepository<ReservationView, Reservation>, ReservationView, Reservation>;

// The view for a BankAccount query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReservationView {
    hotel_id: String,
    room_type: String,
    reserved: bool,
    reason: String,
}

// This updates the view with events as they are committed.
// The logic should be minimal here, e.g., don't calculate.
// design the events to carry the balance information instead.
impl View<Reservation> for ReservationView {
    fn update(&mut self, event: &EventEnvelope<Reservation>) {
        match &event.payload {
            ReservationEvent::ReservationMade {
                hotel_id,
                room_type,
            } => {
                self.hotel_id = hotel_id.clone();
                self.room_type = room_type.clone();
                self.reserved = true;
            }
            ReservationEvent::ReservationCancelled { .. } => {
                self.reserved = false;
            }
        }
    }
}
