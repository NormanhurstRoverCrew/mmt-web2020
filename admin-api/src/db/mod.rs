use mongodb::Document;
use rocket_contrib::database;

pub mod helpers;

#[database("primary_db")]
pub struct PrimaryDb(pub mongodb::db::Database);

pub trait FromDoc {
	fn from_doc(item : &Document) -> Self;
}
