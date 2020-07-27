use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::{context::CustomContext, util::string_to_id},
	models::{utils::*, Booking, Ticket},
};
use bson::{doc, oid::ObjectId, Document};
use juniper::{GraphQLInputObject, ID};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct BasicUser {
	pub name :   String,
	pub email :  String,
	pub mobile : String,
	pub crew :   String,
}

impl From<BasicUser> for User {
	fn from(bu : BasicUser) -> Self {
		Self {
			name : bu.name,
			email : bu.email,
			mobile : bu.mobile,
			crew : bu.crew,
			..Self::default()
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "_id")]
	pub id :             ObjectId,
	pub name :           String,
	pub email :          String,
	pub mobile :         String,
	pub crew :           String,
	pub diet :           String,
	pub email_verified : bool,

	// Used to verify if the supplied email is valid
	code : String,
}

impl User {
	pub fn default() -> Self {
		Self {
			id :             ObjectId::new().expect("OID"),
			name :           "".to_string(),
			email :          "".to_string(),
			mobile :         "".to_string(),
			crew :           "".to_string(),
			diet :           "".to_string(),
			email_verified : false,
			code :           rand::thread_rng()
				.sample_iter(&Alphanumeric)
				.take(16)
				.collect::<String>(),
		}
	}

	pub fn is_code_valid(&self, code : &str) -> Result<(), String> {
		match code {
			"" => Err(String::from("Code is an empty string")),
			c if c == self.code => Ok(()),
			_ => Err(String::from("Supplied code is incorrect")),
		}
	}

	pub fn get_code(&self) -> &str { &self.code }

	pub async fn set_email_verified(&mut self, context : &CustomContext, verified : bool) {
		self.email_verified = verified;

		if let Ok(bson::Bson::Document(doc)) = bson::to_bson(&self) {
			let _ = context
				.users_handel()
				.update_one(
					doc! {
						"_id" => &self.id,
					},
					doc! {
						"$set" => doc,
					},
					None,
				)
				.await;
		};
	}

	pub async fn get_booking(&self, db : &CustomContext) -> Option<Booking> {
		let bookings = db.bookings_handel();
		let booking = DBHelper::find::<Booking>(
			&bookings,
			doc! {
				"user_id" => &self.id,
			},
		);

		match booking.await {
			Some(b) => Some(b),
			None => {
				let booking_id = Booking::create(&db, self).await.expect("New Booking");

				DBHelper::get::<Booking>(&bookings, &booking_id).await
			},
		}
	}

	pub fn get_ticket(&self, _db : &CustomContext) -> Option<Ticket> {
		let _user_id = dbg!(&self.id);

		None
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl User {
	// description: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_hex()) }

	/// Contact name
	fn name(&self) -> &str { &self.name }

	/// Contact email
	fn email(&self) -> &str { &self.email }

	/// Contact mobile
	fn mobile(&self) -> &str { &self.mobile }

	/// Crew
	fn crew(&self) -> &str { &self.crew }

	/// Has this users email been verified?
	fn email_verified(&self) -> bool { self.email_verified }

	async fn booking(&self, context : &CustomContext) -> Option<Booking> {
		self.get_booking(context).await
	}

	fn ticket(&self, context : &CustomContext) -> Option<Ticket> { self.get_ticket(context) }
}
