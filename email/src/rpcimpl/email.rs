use askama::Template;
use bson::oid::ObjectId;
use lettre_email::EmailBuilder;
use mmt::Db;
use mongodb::Database;
use tonic::{Request, Response, Status};

use crate::models::User as UserDb;
use crate::transport::EmailTransport;
use mmt::email::{EmailResponse, User};

#[derive(Debug)]
pub struct Email {
    db: Database,
    transport: EmailTransport,
}

impl Email {
    pub fn new(db: Database, transport: EmailTransport) -> Self {
        Self { db, transport }
    }

    fn build(email: &str) -> EmailBuilder {
        EmailBuilder::new()
            .from((
                format!(
                    "bookings@{}",
                    std::env::var("MAILGUN_HELLO").expect("MAILGUN_HELLO")
                ),
                "MMT Admin",
            ))
            .to(email.to_owned())
            .reply_to(std::env::var("EMAIL_RETURN").expect("EMAIL_RETURN"))
    }
}

#[tonic::async_trait]
impl mmt::email::email_server::Email for Email {
    async fn verify(&self, request: Request<User>) -> Result<Response<EmailResponse>, Status> {
        let user = request.get_ref();
        if let Some(user) = UserDb::get(&self.db, &ObjectId::with_string(&user.id).unwrap()).await {
            let code = if let Some(code) = user.code() {
                code.to_owned()
            } else {
                user.generate_code(&self.db).await
            };

            let verify_email_template = EmailVerifyTemplate {
                name: user.name(),
                verification_link: &format!(
                    "http://localhost:8080/confirm_email?uid={}&code={}",
                    &user.id().to_string(),
                    &code,
                ),
            };

            let email = Self::build(&user.email())
                .subject("Verify your Email! Magical Mystery Tour 2020")
                .html(verify_email_template.render().unwrap())
                .build()
                .unwrap();

            Ok(Response::new(match self.transport.send(email) {
                Ok(_) => EmailResponse { success: true },
                Err(e) => {
                    dbg!(e);
                    EmailResponse { success: false }
                }
            }))
        } else {
            Ok(Response::new(EmailResponse { success: false }))
        }
    }
}

#[derive(Template)]
#[template(path = "user/verify_email.html")]
struct EmailVerifyTemplate<'a> {
    name: &'a str,
    verification_link: &'a str,
}
