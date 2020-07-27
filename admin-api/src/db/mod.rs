use bson::Document;

pub mod helpers;

pub trait FromDoc {
	fn from_doc(item : &Document) -> Self;
}
