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
		id :     ObjectId,
		amount : f64,
	},
	ElectronicFundsTransfer {
		#[serde(rename = "_id")]
		id :     ObjectId,
		amount : f64,
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
	pub fn cash(amount : f64) -> Self {
		Self::Cash {
			id : ObjectId::new(),
			amount,
		}
	}

	pub fn eft(amount : f64) -> Self {
		Self::ElectronicFundsTransfer {
			id : ObjectId::new(),
			amount,
		}
	}

	pub fn stripe(pi_id : String) -> Self {
		Self::Stripe {
			id : ObjectId::new(),
			pi_id,
		}
	}
}
