use reqwest::{self, Client, Url, Error};
use serde::Serialize;

pub trait Create {
	fn create(&self) -> Self;
}

pub trait Update {
	fn update(&self) -> Self;
}

pub fn client() -> reqwest::Result<Client> {
	reqwest::Client::builder()
	.build()
}

fn server_url() -> Result<Url, reqwest::UrlError> {
	Url::parse("http://backend:3000/api/")
}

pub fn create<T>(path: &'static str, obj: T) -> Result<T, Error>
where
	T:  Create+ Serialize,
{
	let object = obj.create();
	let client: Client = client()?;
	let full_url = server_url()
		.expect("Could not get base URL")
		.join(path)
		.expect("Could not join base and path");
	match client
		.post(full_url)
		.json(&obj)
		.send() {
		Ok(resp) => {
			println!("{:?}", resp);
		},
		Err(e) => println!("{:?}", e), 
	}

	println!("{:?}", serde_json::to_string(&object).unwrap());
	Ok(obj) //TODO make this return the new object
}
