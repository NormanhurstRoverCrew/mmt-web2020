use serde::Serialize;
use stripe::Currency;

#[derive(Debug, Serialize)]
struct CreateCheckoutSession<'a> {
	success_url :          &'a str,
	cancel_url :           &'a str,
	payment_method_types : Vec<&'a str>,
	line_items :           Vec<LineItem<'a>>,
}

#[derive(Debug, Serialize)]
struct LineItem<'a> {
	amount :      i64,
	currency :    Currency,
	name :        &'a str,
	quantity :    i64,
	description : &'a str,
}

pub fn create_checkout_session(quantity : i64) {
	let ccs = CreateCheckoutSession {
		success_url :          "http://localhost:8080/payment?success",
		cancel_url :           "http://localhost:8080/payment?cancel",
		payment_method_types : vec!["card"],
		line_items :           vec![LineItem {
			amount : 30 * 100 as i64,
			currency : Currency::AUD,
			name : "MMT2020 Ticket",
			description : "MMT2020 Ticket including Satureday night dinner and Sunday breakfast",
			quantity,
		}],
	};

	dbg!(&ccs);

	let client = reqwest::blocking::Client::new();
	let req = client
		.post("https://api.stripe.com/v1/checkout/sessions")
		.basic_auth(
			"pk_test_3jpeKc8apKbPVGnnNoyY51GK00N6X69Wap",
			None as Option<&str>,
		)
		.form(&ccs);

	dbg!(&req);
}
