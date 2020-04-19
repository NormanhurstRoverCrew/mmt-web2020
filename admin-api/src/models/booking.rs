use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::SharedContext,
	models::{utils::*, Payment, Ticket, User},
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
	pub fn get_tickets(&self, db : &SharedContext) -> Vec<Ticket> {
		let tickets = db.tickets_handel();

		let tickets : Vec<Ticket> = DBHelper::search::<Ticket>(
			&tickets,
			doc! {
				"booking_id" => ObjectId::with_string(&self.id).unwrap(),
			},
		);

		tickets
	}

	pub fn get_user(&self, db : &SharedContext) -> Option<User> {
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

	pub fn delete(&self, db : &SharedContext) -> bool {
		for ticket in &self.tickets {
			ticket.delete(&db);
		}

		db.bookings_handel()
            .delete_one(doc! { "_id" => ObjectId::with_string(&self.id).expect("Booking id could no be converted to ObjectID") }, None).is_ok()
	}
}

#[juniper::graphql_object(
    Context = SharedContext,
)]
impl Booking {
	fn id(&self) -> ID { ID::from(self.id.to_owned()) }

	fn idn(&self) -> i32 { self.no }

	/// Contact details
	fn user(&self, context : &SharedContext) -> Option<User> { self.get_user(context) }

	fn tickets(&self, context : &SharedContext) -> Vec<Ticket> { self.get_tickets(context) }

	fn payment(&self) -> Payment { self.payment.to_owned() }
}
