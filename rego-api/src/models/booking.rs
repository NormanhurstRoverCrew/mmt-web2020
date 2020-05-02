use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::CustomContext,
	models::{utils::*, Payment, Ticket, User},
};
use juniper::ID;
use mongodb::{oid::ObjectId, Document};

#[derive(Clone, Debug)]
pub struct Booking {
	pub id :      String,
	pub no :      i32,
	pub tickets : Vec<Ticket>,
}

impl FromDoc for Booking {
	fn from_doc(item : &Document) -> Self {
		Self {
			id :      doc_get_id(&item),
			no :      doc_get_i32(&item, "no", 9999999),
			tickets : doc_get_tickets(item, "tickets"),
		}
	}
}

#[derive(Clone, Debug)]
struct BookingUserOnly {
	pub user_id : String,
}

impl FromDoc for BookingUserOnly {
	fn from_doc(item : &Document) -> Self {
		Self {
			user_id : doc_get_id_key(&item, "user_id"),
		}
	}
}

impl Booking {
	// Create a new booking with a booking user and 1 ticket containing the booking
	// user.
	pub fn create(db : &CustomContext, user : &User) -> Option<ObjectId> {
		let bookings = db.bookings_handel();
		let seq = db.index("bookings");
		dbg!(seq);

		let booking_id = match bookings.insert_one(
			doc! {
				"user_id" => ObjectId::with_string(&user.id).expect("User id could not be converted to ObjectID"),
				"no" => seq,
			},
			None,
		) {
			Ok(b) => b.inserted_id.map_or(None, |id| {
				Some(id.as_object_id().expect("ObjectId").to_owned())
			}),
			Err(e) => {
				dbg!(e);
				None
			},
		};

		if let Some(b) = &booking_id {
			let tickets = db.tickets_handel();
			tickets
				.insert_one(
					doc! {
						"booking_id" => b.to_owned(),
						"user_id" => ObjectId::with_string(&user.id).expect("User id could not be converted to ObjectID"),
					},
					None,
				)
				.ok();

			Payment::init(&db, &b);
		}

		booking_id
	}

	pub fn get_tickets(&self, db : &CustomContext) -> Vec<Ticket> {
		let tickets = db.tickets_handel();

		let tickets : Vec<Ticket> = DBHelper::search::<Ticket>(
			&tickets,
			doc! {
				"booking_id" => ObjectId::with_string(&self.id).unwrap(),
			},
		);

		tickets
	}

	pub fn get_user(&self, db : &CustomContext) -> User {
		let bookings = db.bookings_handel();
		let booking = match DBHelper::get::<BookingUserOnly>(
			&bookings,
			&ObjectId::with_string(&self.id).unwrap(),
		) {
			Some(booking) => booking,
			None => panic!(),
		};

		DBHelper::get::<User>(
			&db.users_handel(),
			&ObjectId::with_string(&booking.user_id).unwrap(),
		)
		.unwrap()
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Booking {
	/// The root order. This holds all details on an order
	/// including contact, address and postage information

	fn id(&self) -> ID { ID::from(self.id.to_owned()) }

	fn no(&self) -> i32 { self.no }

	/// Contact details
	fn user(&self, context : &CustomContext) -> User { self.get_user(context) }

	fn tickets(&self, context : &CustomContext) -> Vec<Ticket> { self.get_tickets(context) }
}
