pub use mmt_db as db;
pub use mmt_derive as derive;

pub use db::{Create, Db, Delete, Update};
pub use derive::DB;

pub mod email {
    tonic::include_proto!("email");
}
