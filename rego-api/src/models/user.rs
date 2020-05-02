use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::{context::CustomContext, util::string_to_id},
	models::{utils::*, Booking, Ticket},
};
use juniper::{GraphQLInputObject, ID};
use mongodb::Document;

#[derive(GraphQLInputObject, Clone, Debug)]
pub struct BasicUser {
	pub name :   String,
	pub email :  String,
	pub mobile : String,
	pub crew :   String,
}

#[derive(Clone, Debug)]
pub struct User {
	pub id :             String,
	pub name :           String,
	pub email :          String,
	pub mobile :         String,
	pub crew :           String,
	pub email_verified : bool,

	// Used to verify if the supplied email is valid
	code : String,
}

impl User {
	pub fn default() -> Self {
		Self {
			id :             "".to_string(),
			name :           "".to_string(),
			email :          "".to_string(),
			mobile :         "".to_string(),
			crew :           "".to_string(),
			email_verified : false,
			code :           "".to_string(),
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

	pub fn get_booking(&self, db : &CustomContext) -> Option<Booking> {
		let user_id = &self.id;

		let bookings = db.bookings_handel();
		let booking = DBHelper::find::<Booking>(
			&bookings,
			doc! {
				"user_id" => string_to_id(user_id).expect("UID").to_owned(),
			},
		);

		let booking = booking.or_else(|| {
			dbg!("No booking attached to User");
			let booking_id = Booking::create(&db, self).expect("New Booking");

			match DBHelper::get::<Booking>(&bookings, &booking_id) {
				Some(b) => Some(b),
				None => {
					dbg!("Failed to attach new booking to User");
					None
				},
			}
		});

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

		booking
	}

	pub fn get_ticket(&self, _db : &CustomContext) -> Option<Ticket> {
		let _user_id = dbg!(&self.id);

		None
	}
}

impl FromDoc for User {
	fn from_doc(item : &Document) -> Self {
		Self {
			id :             doc_get_id(&item),
			name :           doc_get_string(&item, "name", ""),
			email :          doc_get_string(&item, "email", ""),
			mobile :         doc_get_string(&item, "mobile", ""),
			crew :           doc_get_string(&item, "crew", ""),
			email_verified : doc_get_bool(&item, "email_verified", false),
			code :           doc_get_string(&item, "code", ""), //TODO This might be a loophole???
		}
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl User {
	// description: "Contact Details of the person making the purchase"

	fn id(&self) -> ID { ID::from(self.id.to_owned()) }

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

	fn booking(&self, context : &CustomContext) -> Option<Booking> { self.get_booking(context) }

	fn ticket(&self, context : &CustomContext) -> Option<Ticket> { self.get_ticket(context) }
}
