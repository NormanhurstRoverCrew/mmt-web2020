use crate::models::booking::Booking;
use crate::models::ticket::TICKET_PRICE;
use crate::CustomContext;
use bson::{doc, oid::ObjectId};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use stripe::Client;
use stripe::PaymentIntent;
use stripe::PaymentIntentId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(skip)]
    pub num_tickets: Option<usize>,
    pub transactions: Vec<Transaction>,
}

impl Payment {
    pub fn new(num_tickets: usize) -> Self {
        let num_tickets = Some(num_tickets);
        Self {
            num_tickets,
            transactions: vec![],
        }
    }

    pub fn set_num_tickets(&mut self, num_tickets: usize) {
        self.num_tickets = Some(num_tickets);
    }
}

impl Default for Payment {
    fn default() -> Self {
        Self {
            transactions: vec![],
            num_tickets: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Transaction {
    None,
    Cash {
        #[serde(rename = "_id")]
        id: ObjectId,
        value: f64,
    },
    ElectronicFundsTransfer {
        #[serde(rename = "_id")]
        id: ObjectId,
        value: f64,
    },
    Stripe {
        #[serde(rename = "_id")]
        id: ObjectId,
        pi_id: String,
    },
}

impl Default for Transaction {
    fn default() -> Self {
        Self::None
    }
}

impl Transaction {
    pub fn cash(value: f64) -> Self {
        Self::Cash {
            id: ObjectId::new(),
            value,
        }
    }

    pub fn eft(value: f64) -> Self {
        Self::ElectronicFundsTransfer {
            id: ObjectId::new(),
            value,
        }
    }

    pub fn stripe(pi_id: String) -> Self {
        Self::Stripe {
            id: ObjectId::new(),
            pi_id,
        }
    }

    pub async fn value(&self, context: &CustomContext) -> f64 {
        match self {
            Self::Cash { value, .. } => *value,
            Self::ElectronicFundsTransfer { value, .. } => *value,
            Self::Stripe { pi_id, .. } => {
                let pi_id = PaymentIntentId::from_str(pi_id).unwrap();
                PaymentIntent::retrieve(&context.stripe, &pi_id, &[])
                    .await
                    .map(|pi| pi.amount_received)
                    .ok()
                    .flatten()
                    .unwrap_or(0) as f64
                    / 100.0
            }
            _ => -999.0,
        }
    }

    fn method(&self) -> &str {
        match self {
            Self::Cash { .. } => "CASH",
            Self::ElectronicFundsTransfer { .. } => "EFT",
            Self::Stripe { .. } => "STRIPE",
            Self::None => "UNKNOWN",
        }
    }

    fn id(&self) -> Option<String> {
        match self {
            Self::Stripe { pi_id, .. } => Some(pi_id.to_owned()),
            Self::Cash { .. } => None,
            Self::ElectronicFundsTransfer { id, .. } => Some(id.to_hex()),
            Self::None => None,
        }
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Transaction {
    /// Contact details
    fn method(&self) -> &str {
        self.method()
    }

    async fn value(&self, context: &CustomContext) -> f64 {
        self.value(&context).await
    }

    fn id(&self) -> Option<String> {
        self.id()
    }
}

impl From<TransactionInput> for Transaction {
    fn from(input: TransactionInput) -> Self {
        let id = ObjectId::new();

        match input.method.as_str() {
            "Cash" => Transaction::Cash {
                value: input.value,
                id,
            },
            "ElectronicFundsTransfer" => Transaction::ElectronicFundsTransfer {
                value: input.value,
                id,
            },
            _ => Transaction::None,
        }
    }
}

#[juniper::graphql_object(Context = CustomContext)]
impl Payment {
    /// Contact details
    fn transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    fn ticket_price(&self) -> f64 {
        TICKET_PRICE
    }

    fn price(&self) -> Option<f64> {
        self.num_tickets
            .map(|tickets| tickets as f64 * TICKET_PRICE)
    }
}

#[derive(GraphQLInputObject, Deserialize, Debug, Clone)]
pub struct TransactionInput {
    pub value: f64,
    pub method: String,
}
