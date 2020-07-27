use juniper::GraphQLInputObject;
use serde::Deserialize;

#[derive(GraphQLInputObject, Deserialize, Debug, Clone)]
pub struct TransactionInput {
	pub value :  f64,
	pub method : String,
}
