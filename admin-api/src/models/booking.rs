use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::Database,
	models::{utils::*, Payment, Ticket, Transaction, User},
};
use juniper::ID;
use mongodb::{oid::ObjectId, Document};

#[derive(Clone, Debug)]
pub struct Booking {
	pub id :      String,
	pub no :      i32,
	pub tickets : Vec<Ticket>,
	pub payment : Payment,
}

impl FromDoc for Booking {
	fn from_doc(item : &Document) -> Self {
		Self {
			id :      doc_get_id(&item),
			no :      doc_get_i32(&item, "no", 9999999),
			tickets : doc_get_tickets(&item, "tickets"),
			payment : Payment::from_doc(item),
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
	pub fn get_tickets(&self, db : &Database) -> Vec<Ticket> {
		let tickets = db.tickets_handel();

		let tickets : Vec<Ticket> = DBHelper::search::<Ticket>(
			&tickets,
			doc! {
				"booking_id" => ObjectId::with_string(&self.id).unwrap(),
			},
		);

		tickets
	}

	pub fn get_user(&self, db : &Database) -> Option<User> {
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
	}
}

graphql_object!(Booking: Database |&self| {
	description: "The root order. This holds all details on an order
including contact, address and postage information"

	field id() -> ID { ID::from(self.id.to_owned()) }

	field idn() -> i32 { self.no }

	/// Contact details
	field user(&exec) -> Option<User> {
		self.get_user(exec.context())
	}

	field tickets(&exec) -> Vec<Ticket> {
		self.get_tickets(exec.context())
	}

	field payment(&exec) -> Payment {
		self.payment.to_owned()
	}
});
