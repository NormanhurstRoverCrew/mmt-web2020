use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stripe::{Client, PaymentIntent, PaymentIntentId};

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

	pub async fn value(&self, client : &Client) -> f64 {
		match self {
			Self::Cash {
				value, ..
			} => *value,
			Self::ElectronicFundsTransfer {
				value, ..
			} => *value,
			Self::Stripe {
				pi_id, ..
			} => {
				let pi_id = PaymentIntentId::from_str(pi_id).unwrap();
				PaymentIntent::retrieve(client, &pi_id, &[])
					.await
					.map(|pi| pi.amount_received)
					.ok()
					.flatten()
					.unwrap_or(0) as f64 / 100.0
			},
			_ => -999.0,
		}
	}
}
