mod booking;
mod payment;
mod ticket;
mod user;
pub mod utils;
mod vehicle;
pub use booking::Booking;
pub use payment::{Payment, Transaction};
pub use ticket::{Ticket, TicketUpdate};
pub use user::{BasicUser, User, UserUpdate};
pub use vehicle::{NewVehicle, Vehicle};

pub const TICKET_PRICE : f64 = 30.0;
