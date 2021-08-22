#![recursion_limit = "512"]
mod app;
mod graphql;
use wasm_bindgen::prelude::*;

mod components;

mod booking_summary;
mod checkin;
mod checkout;
mod confirm_email;
mod home;
mod manage_vehicle;
mod new_booking_form;
mod purchase_tickets;
mod vehicle_agent;

mod booking_agent;
mod ticket_agent;

mod crews;
pub use crews::CREWS;

pub type ObjectId = String;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
