use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReservationEvent {
    ReservationMade { hotel_id: String, room_type: String },
    ReservationCancelled,
}

impl DomainEvent for ReservationEvent {
    fn event_type(&self) -> String {
        match self {
            ReservationEvent::ReservationMade { .. } => "ReservationMade".to_string(),
            ReservationEvent::ReservationCancelled => "ReservationCancelled".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub struct ReservationError(String);

impl From<&str> for ReservationError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for ReservationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ReservationError {}
