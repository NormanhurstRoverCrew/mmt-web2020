use crate::{graphql::context::CustomContext, models::TICKET_PRICE, wire::TransactionInput};
use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payment {
	pub transactions : Vec<Transaction>,
}

impl Default for Payment {
	fn default() -> Self {
		Self {
			transactions : vec![],
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Transaction {
	None,
	Cash {
		#[serde(rename = "_id")]
		id :    ObjectId,
		value : f64,
	},
	ElectronicFundsTransfer {
		#[serde(rename = "_id")]
		id :    ObjectId,
		value : f64,
	},
	Stripe {
		#[serde(rename = "_id")]
		id :    ObjectId,
		pi_id : String,
	},
}

impl Default for Transaction {
	fn default() -> Self { Self::None }
}

impl Transaction {
	pub fn cash(value : f64) -> Self {
		Self::Cash {
			id : ObjectId::new(),
			value,
		}
	}

	pub fn eft(value : f64) -> Self {
		Self::ElectronicFundsTransfer {
			id : ObjectId::new(),
			value,
		}
	}

	pub fn stripe(pi_id : String) -> Self {
		Self::Stripe {
			id : ObjectId::new(),
			pi_id,
		}
	}

	fn value(&self) -> f64 {
		match self {
			Self::Cash {
				value, ..
			} => *value,
			Self::ElectronicFundsTransfer {
				value, ..
			} => *value,
			_ => -999.9,
		}
	}

	fn method(&self) -> &str {
		match self {
			Self::Cash {
				..
			} => "CASH",
			Self::ElectronicFundsTransfer {
				..
			} => "EFT",
			Self::Stripe {
				..
			} => "STRIPE",
			Self::None => "UNKNOWN",
		}
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Transaction {
	/// Contact details
	fn method(&self) -> &str { self.method() }

	fn value(&self) -> f64 { self.value() }
}

impl From<TransactionInput> for Transaction {
	fn from(input : TransactionInput) -> Self {
		let id = ObjectId::new();

		match input.method.as_str() {
			"Cash" => Transaction::Cash {
				value : input.value,
				id,
			},
			"ElectronicFundsTransfer" => Transaction::ElectronicFundsTransfer {
				value : input.value,
				id,
			},
			_ => Transaction::None,
		}
	}
}

#[juniper::graphql_object(Context = CustomContext)]
impl Payment {
	/// Contact details
	fn transactions(&self, context : &CustomContext) -> Vec<Transaction> {
		self.transactions.clone()
	}

	fn ticket_price(&self) -> f64 { TICKET_PRICE }
}
