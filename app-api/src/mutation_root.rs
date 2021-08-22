use std::convert::TryInto;

use crate::custom_context::CustomContext;
use crate::models::booking::Booking;
use crate::models::event::Event;
use crate::models::event::EventInput;
use crate::models::event::EventType;
use crate::models::payment::Transaction;
use crate::models::vehicle::Vehicle;
use bson::oid::ObjectId;
use chrono::Utc;
use juniper::{graphql_value, FieldError, FieldResult};
use mmt::Db;
use mmt::Update;

pub struct MutationRoot;
#[juniper::graphql_object(
    Context = CustomContext
)]
impl MutationRoot {
    async fn update_tickets() -> FieldResult<bool> {
        Ok(true)
    }

    async fn add_transaction(
        context: &CustomContext,
        booking_id: ObjectId,
        value: f64,
    ) -> FieldResult<Booking> {
        let mut booking = Booking::get(&context.db, &booking_id)
            .await
            .ok_or(FieldError::new(
                "Booking does not exists",
                graphql_value!({"type":"BOOKING_NOT_FOUND"}),
            ))?;

        booking.payment.transactions.push(Transaction::Cash {
            id: ObjectId::new(),
            value,
        });

        booking.update(&context.db).await;

        dbg!(&booking);

        Ok(booking)
    }

    async fn add_events(
        context: &CustomContext,
        vehicle_id: ObjectId,
        events: Vec<EventInput>,
    ) -> FieldResult<Vehicle> {
        dbg!(&events);
        let events = events.into_iter().filter_map(|e| -> Option<Event> {
            match e.try_into() {
                Ok(e) => Some(e),
                Err(e) => {
                    dbg!(&e);
                    None
                }
            }
        });

        let mut vehicle = Vehicle::get(&context.db, &vehicle_id)
            .await
            .ok_or(FieldError::new(
                "Vehicle does not exists",
                graphql_value!({"type":"VEHICLE_NOT_FOUND"}),
            ))?;

        vehicle.events.extend(events);

        vehicle.update(&context.db).await;

        Ok(vehicle)
    }

    async fn add_event(
        context: &CustomContext,
        vehicle_id: ObjectId,
        event: EventInput,
    ) -> FieldResult<Event> {
        let event: Event = match event.try_into() {
            Ok(e) => e,
            Err(e) => {
                dbg!(&e);
                return Err(FieldError::new(
                    "Event not valid",
                    graphql_value!({"type": "EVENT_INVALID"}),
                ));
            }
        };

        let mut vehicle = Vehicle::get(&context.db, &vehicle_id)
            .await
            .ok_or(FieldError::new(
                "Vehicle does not exists",
                graphql_value!({"type":"VEHICLE_NOT_FOUND"}),
            ))?;

        vehicle.events.push(event.clone());

        vehicle.update(&context.db).await?;
        Ok(event)
    }

    async fn remove_events(
        context: &CustomContext,
        event_ids: Vec<ObjectId>,
    ) -> FieldResult<Vec<Event>> {
        let mut out: Vec<Event> = vec![];
        for event_id in event_ids {
            if let Some(mut vehicle) = Vehicle::get_by_event(&context, event_id.clone()).await {
                let event = vehicle.events.iter().find(|event| event.id == event_id);
                if let Some(event) = &event {
                    out.push((*event).clone());
                }
                vehicle.events.retain(|e| e.id != event_id);
                vehicle.update(&context.db).await;
            } else {
            }
        }

        Ok(out)
    }
}
