use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::commands::ReservationCommand;
use crate::domain::events::{ReservationError, ReservationEvent};

#[derive(Default, Serialize, Deserialize)]
pub struct Reservation {
    hotel_id: String,
    room_type: String,
    reserved: bool,
    reason: String,
}

#[async_trait]
impl Aggregate for Reservation {
    type Command = ReservationCommand;
    type Event = ReservationEvent;
    type Error = ReservationError;
    type Services = ();

    // This identifier should be unique to the system.
    fn aggregate_type() -> String {
        "reservation".to_string()
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            ReservationCommand::MakeReservation {
                hotel_id,
                room_type,
            } => Ok(vec![ReservationEvent::ReservationMade {
                hotel_id,
                room_type,
            }]),
            ReservationCommand::CancelReservation => {
                Ok(vec![ReservationEvent::ReservationCancelled])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ReservationEvent::ReservationMade {
                hotel_id,
                room_type,
            } => {
                self.hotel_id = hotel_id;
                self.room_type = room_type;
                self.reserved = true;
            }
            ReservationEvent::ReservationCancelled { .. } => {
                self.reserved = false;
            }
        }
    }
}

// The aggregate tests are the most important part of a CQRS system.
// The simplicity and flexibility of these tests are a good part of what
// makes an event sourced system so friendly to changing business requirements.
#[cfg(test)]
mod aggregate_tests {

    use cqrs_es::test::TestFramework;

    use crate::domain::aggregate::Reservation;
    use crate::domain::commands::ReservationCommand;
    use crate::domain::events::ReservationEvent;

    // A test framework that will apply our events and command
    // and verify that the logic works as expected.
    type AccountTestFramework = TestFramework<Reservation>;

    #[test]
    fn test_make_reservation() {
        let expected = ReservationEvent::ReservationMade {
            hotel_id: "id".to_string(),
            room_type: "Queen".to_string(),
        };
        let command = ReservationCommand::MakeReservation {
            hotel_id: "id".to_string(),
            room_type: "Queen".to_string(),
        };
        // Obtain a new test framework
        AccountTestFramework::with(())
            // In a test case with no previous events
            .given_no_previous_events()
            // When we fire this command
            .when(command)
            // then we expect these results
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_cancel_reservation() {
        let previous = ReservationEvent::ReservationMade {
            hotel_id: "id".to_string(),
            room_type: "Queen".to_string(),
        };
        let expected = ReservationEvent::ReservationCancelled;
        let command = ReservationCommand::CancelReservation;

        AccountTestFramework::with(())
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![expected]);
    }
}
