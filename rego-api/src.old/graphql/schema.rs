use juniper::{Context as JuniperContext, ID};
use mongodb::{
	coll::{options::WriteModel, Collection},
	db::ThreadedDatabase,
	oid::ObjectId,
	Bson,
};

use crate::{
	db::PrimaryDb,
	models::{Booking, Todo, User},
};

pub struct Context {
	pub connection : PrimaryDb,
}

impl Context {
	pub fn todos_handle(&self) -> Collection { self.connection.collection("todos") }
	pub fn bookings_handel(&self) -> Collection { self.connection.collection("bookings") }
}

impl JuniperContext for Context {}

#[juniper::object(description = "A todo item that can be marked as completed")]
impl Todo {
	/// The unique id of the todo item
	fn id(&self) -> &ID { &self.id }

	/// The user-editable title
	fn title(&self) -> &str { &self.title }

	/// Determines whether the user has completed the item or not
	fn completed(&self) -> bool { self.completed }
}

#[juniper::object(description = "A Booking holds the payment information for multiple tickets")]
impl User {
	/// The unique id of the todo item
	fn id(&self) -> &ID { &self.id }

	/// The user-editable title
	fn name(&self) -> &str { &self.name }

	/// Determines whether the user has completed the item or not
	fn email(&self) -> Option<String> { self.email.clone() }

	/// Eh
	fn mobile(&self) -> Option<String> { self.mobile.clone() }

	/// Eh
	fn crew(&self) -> Option<String> { self.crew.clone() }
}

#[juniper::object(description = "A Booking holds the payment information for multiple tickets")]
impl Booking {
	/// The unique id of the todo item
	fn id(&self) -> &ID { &self.id }

	/// The user-editable title
	fn users(&self) -> Vec<User> { self.users.clone() }
}

pub struct QueryRoot;

#[juniper::object(
    Context = Context,
)]
impl QueryRoot {
	/// An array of every booking
	fn bookings(context : &Context) -> Vec<Booking> {
		let bookings = context.bookings_handel();
		bookings
			.find(None, None)
			.unwrap()
			.into_iter()
			.filter_map(|item| match item {
				Ok(item) => Some(Booking::from_doc(item)),
				Err(_) => None,
			})
			.collect()
	}

	/// An array of every user that has a ticket
	fn users(context : &Context) -> Vec<User> {
		let bookings = context.bookings_handel();
		bookings
			.find(None, None)
			.unwrap()
			.into_iter()
			.filter_map(|item| match item {
				Ok(item) => Some(Booking::from_doc(item)),
				Err(_) => None,
			})
			.flat_map(|item| {
				item.users
					.iter()
					.map(|user| user.to_owned())
					.collect::<Vec<User>>()
			})
			.collect()
	}

	/// Returns an array of all the crews that have a participant
	fn crews(context : &Context) -> Vec<String> {
		let bookings = context.bookings_handel();
		bookings
			.find(None, None)
			.unwrap()
			.into_iter()
			.filter_map(|item| match item {
				Ok(item) => Some(Booking::from_doc(item)),
				Err(_) => None,
			})
			.flat_map(|item| {
				item.users
					.iter()
					.filter_map(|user| user.crew.clone())
					.collect::<Vec<String>>()
			})
			.collect()
	}
}

pub struct MutationRoot;

#[juniper::object(
    Context = Context
)]
impl MutationRoot {
	fn newBooking(context : &Context, name : String) -> Option<Booking> {
		let bookings = context.bookings_handel();
		let result = bookings
			.insert_one(
				doc! {
					"users" => [{
						"name" => name
					}]
				},
				None,
			)
			.unwrap();

		let id = result
			.inserted_id
			.unwrap_or(Bson::ObjectId(ObjectId::new().unwrap()));

		match bookings.find_one(Some(doc! {"_id" => id}), None) {
			Ok(Some(booking)) => Some(Booking::from_doc(booking)),
			_ => None,
		}
	}

	/// Update an existing todo item.
	/// Will only updated the provided fields - if either `completed` or `title`
	/// are omitted or null, they will be ignored.
	/// The mutation will return null if no todo item with the specified ID
	/// could be found.

	fn updateTodo(
		context : &Context,
		id : String,
		title : Option<String>,
		completed : Option<bool>,
	) -> Option<Todo> {
		let todos = context.todos_handle();

		let oid = ObjectId::with_string(&id).unwrap();

		// Make sure the document exisits and return none if it doesnt
		match todos.find_one(Some(doc! {"_id" => oid.to_owned()}), None) {
			Ok(Some(_)) => (),
			_ => return None,
		};

		let mut writes : Vec<WriteModel> = vec![];

		if let Some(t) = title {
			writes.push(WriteModel::UpdateOne {
				filter : doc! { "_id" => oid.to_owned() },
				update : doc! { "$set" => { "title" => t} },
				upsert : None,
			});
		};

		if let Some(c) = completed {
			writes.push(WriteModel::UpdateOne {
				filter : doc! { "_id" => oid.to_owned() },
				update : doc! { "$set" => { "completed" => c } },
				upsert : None,
			});
		};

		todos.bulk_write(writes, false /* ordered */);

		match todos.find_one(Some(doc! {"_id" => oid.to_owned()}), None) {
			Ok(Some(item)) => Some(Todo::from_doc(item)),
			_ => None,
		}
	}
}
