use crate::models::User;
use askama::Template;
use lettre::{
	builder::EmailBuilder,
	smtp::{
		authentication::{Credentials, Mechanism},
		error::{Error, SmtpResult},
		extension::ClientId,
		response::Response,
		ConnectionReuseParameters,
	},
	Email, SmtpClient, SmtpTransport, Transport,
};
use std::{error, fmt};

#[derive(Debug)]
pub enum MyEmailError {
	SmtpError(Error),
	NoUser,
}

impl fmt::Display for MyEmailError {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result { write!(f, "Some Email Error... TODO") }
}

// This is important for other errors to wrap this one.
impl error::Error for MyEmailError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			MyEmailError::SmtpError(e) => Some(e),
			_ => None,
		}
	}
}

pub type MyEmailResult<T> = Result<T, MyEmailError>;

#[derive(Template)]
#[template(path = "user/verify_email.html")]
struct VerifyEmailTemplate<'a> {
	name :              &'a str,
	verification_link : &'a str,
}

pub struct MyEmail<'a> {
	user : Option<&'a User>,
}

impl<'a> MyEmail<'a> {
	pub fn from_user(user : &'a User) -> Self {
		Self {
			user : Some(&user)
		}
	}

	fn transport() -> SmtpTransport {
		// Connect to a remote server on a custom port
		SmtpClient::new_simple(&std::env::var("MAILGUN_SERVER").expect("MAILGUN_SERVER"))
			.unwrap()
			// Set the name sent during EHLO/HELO, default is `localhost`
			.hello_name(ClientId::Domain(
				std::env::var("MAILGUN_HELLO").expect("MAILGUN_HELLO"),
			))
			// Add credentials for authentication
			.credentials(Credentials::new(
				std::env::var("MAILGUN_USER").expect("MAILGUN_USER"),
				std::env::var("MAILGUN_PASS").expect("MAILGUN_PASS"),
			))
			// Enable SMTPUTF8 if the server supports it
			.smtp_utf8(true)
			// Configure expected authentication mechanism
			.authentication_mechanism(Mechanism::Plain)
			// Enable connection reuse
			.connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
			.transport()
	}

	fn send(&self, email : Email) -> SmtpResult { Self::transport().send(email) }

	pub fn verify_email(&'a self) -> MyEmailResult<Response> {
		if let Some(user) = &self.user {
			let verify_email_template = VerifyEmailTemplate {
				name :              &user.name,
				verification_link : &dbg!(format!(
					"http://localhost:8080/confirm_email?uid={}&code={}",
					&user.id.to_string(),
					&user.get_code(),
				)),
			};
			let email = EmailBuilder::new()
				.from(format!(
					"bookings@{}",
					std::env::var("MAILGUN_HELLO").expect("MAILGUN_HELLO")
				))
				.to(user.email.to_owned())
				.reply_to(std::env::var("EMAIL_RETURN").expect("EMAIL_RETURN"))
				.subject("Verify your Email! Magical Mystery Tour 2020")
				.html(verify_email_template.render().unwrap())
				.build()
				.unwrap();

			match self.send(email) {
				Ok(resp) => Ok(resp),
				Err(e) => Err(MyEmailError::SmtpError(e)),
			}
		} else {
			Err(MyEmailError::NoUser)
		}
	}
}
