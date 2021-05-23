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
mod new_booking_form;
mod purchase_tickets;

mod booking_agent;

pub type ObjectId = String;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
