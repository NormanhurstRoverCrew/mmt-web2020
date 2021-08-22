use crate::{
	graphql::context::CustomContext,
	models::{Booking, Ticket},
};
use bson::{doc, oid::ObjectId};
use juniper::{GraphQLInputObject, ID};
use mmt::{Db, DB};
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct BasicUser {
	pub name :   String,
	pub email :  String,
	pub mobile : String,
	pub crew :   String,
}

#[derive(GraphQLInputObject, Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdate {
	pub name :           String,
	pub email :          String,
	pub mobile :         String,
	pub crew :           String,
	pub diet :           Option<String>,
	pub email_verified : bool,
}

#[DB(users)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "_id")]
	pub id :             ObjectId,
	pub name :           String,
	pub email :          String,
	pub mobile :         String,
	pub crew :           String,
	pub diet :           Option<String>,
	pub email_verified : bool,

	// Used to verify if the supplied email is valid
	code : Option<String>,
}

impl User {
	pub fn default() -> Self {
		Self {
			id :             ObjectId::new(),
			name :           "".to_string(),
			email :          "".to_string(),
			mobile :         "".to_string(),
			crew :           "".to_string(),
			diet :           None,
			email_verified : false,
			code :           None,
		}
	}

	pub fn is_code_valid(&self, code : &str) -> Result<(), String> {
		match &self.code {
			Some(real) => match code {
				"" => Err(String::from("Code is an empty string")),
				c if c == real => Ok(()),
				_ => Err(String::from("Supplied code is incorrect")),
			},
			_ => Err(String::from("Code was never generated...")),
		}
	}

	pub fn get_code(&self) -> Option<&str> { self.code.as_ref().map(|c| c.as_str()) }

	pub async fn get_booking(&self, context : &CustomContext) -> Option<Booking> {
		let booking = Booking::find_one(
			&context.db,
			doc! {
				"user_id" : &self.id,
			},
		);

		// let booking = match booking {
		// 	Some(booking) => Some(booking),
		// 	None => {
		// 		let booking_id = Booking::create(&db, self).expect("New Booking");

		// 		match DBHelper::get::<Booking>(&bookings, &booking_id) {
		// 			Some(b) => Some(b),
		// 			None => None,
		// 		}
		// 	},
		// };

		booking.await
	}

	pub async fn get_ticket(&self, _db : &CustomContext) -> Option<Ticket> {
		let _user_id = dbg!(&self.id);

		None
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl User {
	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	/// Contact name
	fn name(&self) -> &str { &self.name }

	/// Contact email
	fn email(&self) -> &str { &self.email }

	/// Contact mobile
	fn mobile(&self) -> &str { &self.mobile }

	/// Crew
	fn crew(&self) -> &str { &self.crew }

	/// Diet
	fn diet(&self) -> Option<&str> { self.diet.as_ref().map(|d| d.as_str()) }

	/// Has this users email been verified?
	fn email_verified(&self) -> bool { self.email_verified }

	async fn booking(&self, context : &CustomContext) -> Option<Booking> {
		self.get_booking(context).await
	}

	async fn ticket(&self, context : &CustomContext) -> Option<Ticket> {
		self.get_ticket(context).await
	}
}
