use rocket_contrib::database;

#[database("primary_db")]

pub struct PrimaryDb(pub mongodb::db::Database);
