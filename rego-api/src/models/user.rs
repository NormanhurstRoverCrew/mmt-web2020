use crate::{
	graphql::context::CustomContext,
	models::{Booking, Ticket},
};
use bson::{doc, oid::ObjectId};
use juniper::{GraphQLInputObject, ID};
use mmt::{Db, Update, DB};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
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
	code : String,
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

		if let Ok(_) = self.update(&context.db).await {};
	}

	pub async fn get_booking(&self, context : &CustomContext) -> Option<Booking> {
		let booking = Booking::find_one(
			&context.db,
			doc! {
				"user_id" : &self.id,
			},
		);

		match booking.await {
			Some(b) => Some(b),
			None => {
				let booking_id = Booking::create(&context, self).await.expect("New Booking");

				Booking::get(&context.db, &booking_id).await
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
