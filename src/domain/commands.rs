use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReservationCommand {
    MakeReservation { hotel_id: String, room_type: String },
    CancelReservation,
}
