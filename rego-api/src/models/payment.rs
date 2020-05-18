use crate::{db::FromDoc, graphql::context::CustomContext, models::utils::*};
use serde::{Serialize, Deserialize};
use bson::{doc, oid::ObjectId, Document};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Payment {
    transactions: Vec<Transaction>,
}

impl Payment {
	pub async fn init(db : &CustomContext, booking_id : &ObjectId) {
		match db.bookings_handel().update_one(
			doc! {"_id" => booking_id.to_owned()},
			doc! {"$set" => {
				"payment" => {
					"transactions" => []
				},
			}},
			None,
		).await {
			Ok(_) => {},
			Err(e) => {
				eprintln!("{}", e);
				panic!("could not add payment info to booking");
			},
		};
	}

	pub async fn add_payment(db : &CustomContext, booking_id : &ObjectId, r#type: &str, doc : Option<Document>) {
        let mut new_transaction = doc! {
            "_id" => ObjectId::new().unwrap(),
            "type" => r#type,
        };

        if let Some(doc) = doc {
        new_transaction.extend(doc);
        }

        let update = doc! {
			"$push" => {
				"payment.transactions" => new_transaction,
			}
		};

		db.bookings_handel()
			.update_one(
				doc! { "_id" => booking_id },
				update,
				None,
			)
            .await
			.unwrap();
	}

	pub async fn add_stripe_payment(db : &CustomContext, booking_id : &ObjectId, pi : &str) {
        Self::add_payment(db, booking_id, "stripe", Some(doc! {
            "pi" => pi,
        })).await;
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    #[serde(rename = "_id")]
    id: ObjectId,
    r#type: String,
}
