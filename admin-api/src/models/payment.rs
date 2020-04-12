use crate::{
	db::{helpers as DBHelper, FromDoc},
	graphql::context::Database,
	models::{utils::*, Booking},
};
use juniper::ID;
use mongodb::{oid::ObjectId, Document};

#[derive(Clone, Debug)]
pub struct Payment {
	pub ticket_price :    f64,
	pub transactions :    Vec<Transaction>,
	pub proposed_method : TransactionMethod,
}

impl FromDoc for Payment {
	fn from_doc(item : &Document) -> Self {
		let p = item.get_document("payment").unwrap();
		Self {
			transactions :    Transaction::get_vec_from_doc(&p),
			ticket_price :    doc_get_f64(&p, "ticket_price", 40.0),
			proposed_method : TransactionMethod::from_doc(&p),
		}
	}
}

impl Payment {
	pub fn init(&self, db : &Database, booking_id : &ObjectId) {
		match db.bookings_handel().update_one(
			doc! {"_id" => booking_id.to_owned()},
			doc! {"$set" => {
				"payments" => [],
			}},
			None,
		) {
			Ok(_) => {},
			Err(e) => {
				eprintln!("{}", e);
				panic!("could not add payment info to booking");
			},
		};
	}
}

graphql_object!(Payment: Database |&self| {
	description: "The root order. This holds all details on an order
including contact, address and postage information"

	field transactions() -> Vec<Transaction> {self.transactions.to_owned()}
	field proposed_method() -> String {self.proposed_method.to_string()}
	field ticket_price() -> f64 {self.ticket_price}

});

#[derive(juniper::GraphQLEnum, Copy, Clone, Debug)]
pub enum TransactionMethod {
	Cash,
	EFT,
	Stripe,
	Paypal,
}

impl FromDoc for TransactionMethod {
	fn from_doc(item : &Document) -> Self {
		match item.get_str("transaction_method") {
			Ok("CASH") => TransactionMethod::Cash,
			Ok("STRIPE") => TransactionMethod::Stripe,
			Ok("PAYPAL") => TransactionMethod::Paypal,
			Ok("EFT") | _ => TransactionMethod::EFT,
		}
	}
}

impl ToString for TransactionMethod {
	fn to_string(&self) -> String {
		String::from(match self {
			TransactionMethod::Cash => "CASH",
			TransactionMethod::Stripe => "STRIPE",
			TransactionMethod::Paypal => "PAYPAL",
			TransactionMethod::EFT => "EFT",
		})
	}
}

#[derive(GraphQLInputObject, Clone, Debug, Copy)]
pub struct TransactionInput {
	pub value :  f64,
	pub method : TransactionMethod,
}

#[derive(Clone, Debug)]
pub struct Transaction {
	id :     String,
	value :  f64,
	method : TransactionMethod,
}

impl Transaction {
	pub fn get_vec_from_doc(doc : &Document) -> Vec<Transaction> {
		doc.get_array("transactions").map_or_else(
			|_e| vec![],
			|t| {
				t.iter()
					.map(|t| Transaction::from_doc(t.as_document().unwrap()))
					.collect::<Vec<Transaction>>()
			},
		)
	}
}

impl FromDoc for Transaction {
	fn from_doc(item : &Document) -> Self {
		Self {
			id :     doc_get_id(&item),
			value :  doc_get_f64(&item, "value", 0.0),
			method : TransactionMethod::from_doc(&item),
		}
	}
}

graphql_object!(Transaction: Database |&self| {
	field value() -> f64 {self.value}
	field method() -> TransactionMethod {self.method}

});
