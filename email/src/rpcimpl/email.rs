use crate::models::Ticket as TicketDb;
use askama::Template;
use bson::oid::ObjectId;
use futures::future::join;
use lettre_email::EmailBuilder;
use mmt::Db;
use mongodb::Database;
use tonic::{Request, Response, Status};

use crate::models::Booking as BookingDb;
use crate::models::User as UserDb;
use crate::models::Vehicle as VehicleDb;
use crate::transport::EmailTransport;
use mmt::email::{Booking, EmailResponse, TicketTeamUpdate, UpdateType, User, Vehicle};

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
        let user_id = ObjectId::parse_str(&user.id)
            .map_err(|e| Status::invalid_argument(format!("Could not parse ObjectId: {}", e)))?;
        if let Some(user) = UserDb::get(&self.db, &user_id).await {
            let code = if let Some(code) = user.code() {
                code.to_owned()
            } else {
                user.generate_code(&self.db).await
            };

            let verify_email_template = EmailVerifyTemplate {
                name: user.name(),
                verification_link: &format!(
                    "{}/confirm_email?uid={}&code={}",
                    std::env::var("REGO_URL").unwrap_or("http://localhost:8000".into()),
                    &user.id().to_string(),
                    &code,
                ),
            };

            let email = match Self::build(&user.email())
                .subject("Verify your Email! | MMT")
                .html(verify_email_template.render().unwrap())
                .build()
            {
                Ok(email) => email,
                Err(e) => {
                    dbg!(&e);
                    eprintln!(
                        "Failed to build verification email to {}: {}",
                        user.email(),
                        e
                    );
                    return Ok(Response::new(EmailResponse { success: false }));
                }
            };

            Ok(Response::new(match self.transport.send(email) {
                Ok(_) => {
                    eprintln!("Send verification email to {}", &user.email());
                    EmailResponse { success: true }
                }
                Err(e) => {
                    dbg!(e);
                    EmailResponse { success: false }
                }
            }))
        } else {
            Ok(Response::new(EmailResponse { success: false }))
        }
    }

    async fn onboard_booking(
        &self,
        request: Request<Booking>,
    ) -> Result<Response<EmailResponse>, Status> {
        let mut failed_to_send = 0;
        let booking = request.get_ref();
        let booking_id = ObjectId::parse_str(&booking.id)
            .map_err(|e| Status::invalid_argument(format!("Could not parse ObjectId: {}", e)))?;
        if let Some(booking) = BookingDb::get(&self.db, &booking_id).await {
            let user_ids: Vec<ObjectId> = booking
                .get_tickets(&self.db)
                .await
                .iter()
                .map(|t| t.user_id)
                .collect();

            let users = UserDb::find_ids(&self.db, &user_ids).await;

            for user in &users {
                // if user.verified() {
                //     continue;
                // }

                let code = if let Some(code) = user.code() {
                    code.to_owned()
                } else {
                    user.generate_code(&self.db).await
                };

                let verify_email_template = OnboardTemplate {
                    name: user.name(),
                    checkin_link: &format!(
                        "{}/checkin/{}&code={}",
                        std::env::var("REGO_URL").unwrap_or("http://localhost:8000".into()),
                        &user.id().to_string(),
                        &code,
                    ),
                };

                let email = Self::build(&user.email())
                    .subject("Join your team! | MMT")
                    .html(verify_email_template.render().unwrap())
                    .build()
                    .unwrap();

                match self.transport.send(email) {
                    Ok(_) => {
                        eprintln!("Send onboarding email to {}", &user.email());
                    }
                    Err(e) => {
                        dbg!(e);
                        failed_to_send += 1;
                    }
                }
            }

            Ok(Response::new(EmailResponse {
                success: failed_to_send == 0,
            }))
        } else {
            eprintln!("No booking found for {}", &booking_id);
            Ok(Response::new(EmailResponse { success: false }))
        }
    }

    async fn notify_driver_new_passenger(
        &self,
        request: Request<Vehicle>,
    ) -> Result<Response<EmailResponse>, Status> {
        let vehicle = request.get_ref();
        let vehicle_id = vehicle
            .id
            .parse()
            .map_err(|e| Status::invalid_argument(format!("Could not parse ObjectId: {}", e)))?;
        let vehicle = VehicleDb::get(&self.db, &vehicle_id)
            .await
            .ok_or(Status::not_found("Vehicle does not exist"))?;
        dbg!(&vehicle);

        let driver = vehicle.get_driver(&self.db).await;
        let driver = driver
            .user(&self.db)
            .await
            .ok_or(Status::not_found("Ticket does not exist"))?;

        let num_pax = vehicle.requested_tickets.len() as u32;

        let team_name = &vehicle.name;

        let new_pax = NotifyDriverNewPassenger {
            name: driver.name(),
            manage_link: &format!(
                "{}/manage_vehicle/{}",
                std::env::var("REGO_URL").unwrap_or("http://localhost:8000".into()),
                &vehicle.id.to_string(),
            ),
            num_pax,
            team_name,
        };

        let email = Self::build(&driver.email())
            .subject(&format!(
                "{} new passengers want to join your team | MMT",
                num_pax
            ))
            .html(new_pax.render().unwrap())
            .build()
            .map_err(|err| {
                Status::invalid_argument(format!("Failed to send email to driver: {:?}", err))
            })?;

        let success = match self.transport.send(email) {
            Ok(_) => {
                eprintln!("Send pax notification email to {}", &driver.email());
                true
            }
            Err(e) => {
                dbg!(e);
                false
            }
        };
        Ok(Response::new(EmailResponse { success }))
    }

    async fn ticket_team_update(
        &self,
        request: Request<TicketTeamUpdate>,
    ) -> Result<Response<EmailResponse>, Status> {
        let update = request.get_ref();

        let TicketTeamUpdate {
            ticket_id,
            vehicle_id,
            update_type,
        } = update;

        let ticket_id = ticket_id
            .parse()
            .map_err(|e| Status::invalid_argument(format!("Could not parse ObjectId: {}", e)))?;
        let vehicle_id = vehicle_id
            .parse()
            .map_err(|e| Status::invalid_argument(format!("Could not parse ObjectId: {}", e)))?;

        let (vehicle, ticket) = match join(
            VehicleDb::get(&self.db, &vehicle_id),
            TicketDb::get(&self.db, &ticket_id),
        )
        .await
        {
            (Some(v), Some(t)) => (v, t),
            _ => return Err(Status::not_found("Could not find vehicle or ticket")),
        };

        let user = match ticket.user(&self.db).await {
            Some(u) => u,
            _ => return Err(Status::not_found("Could not find user")),
        };

        let (title, msg_short, msg_long): (String, String, String) = match *update_type {
            x if x == UpdateType::Accept as i32 => (
                "Accepted".into(),
                format!("Your request to join {} has been accepted.", &vehicle.name),
                "From here there's not much else you need to do until MMT. Further information will come closer to the event".into(),
            ),
            x if x == UpdateType::Decline as i32 => ("Declined".into(), format!("Your request to join {} was declined.", &vehicle.name), "Speak to the driver of your vehicle to make sure you tried to join the right vehicle/team.".into()),
            x if x == UpdateType::Remove as i32 => ("Removed".into(), format!("You were previously accepted into the {} team, but you have now been removed.", &vehicle.name), "if this was a mistake, you need to request to join a team again. Speak to your driver to get this sorted.".into()),
            _ => {
                return Err(Status::invalid_argument(
                    "UpdateType is not a valid variant",
                ))
            }
        };

        let email = TeamUpdate {
            name: user.name(),
            msg_short: msg_short.as_str(),
            msg_long: msg_long.as_str(),
        };

        let email = Self::build(&user.email())
            .subject(&format!("Team Request {} | MMT", title))
            .html(email.render().unwrap())
            .build()
            .unwrap();

        let success = match self.transport.send(email) {
            Ok(_) => {
                eprintln!("Send team update email to {}", &user.email());
                true
            }
            Err(e) => {
                dbg!(e);
                false
            }
        };

        Ok(Response::new(EmailResponse { success }))
    }
}

#[derive(Template)]
#[template(path = "user/verify_email.html")]
struct EmailVerifyTemplate<'a> {
    name: &'a str,
    verification_link: &'a str,
}

#[derive(Template)]
#[template(path = "user/onboard.html")]
struct OnboardTemplate<'a> {
    name: &'a str,
    checkin_link: &'a str,
}

#[derive(Template)]
#[template(path = "user/notify_driver_new_passenger.html")]
struct NotifyDriverNewPassenger<'a> {
    name: &'a str,
    manage_link: &'a str,
    num_pax: u32,
    team_name: &'a str,
}

#[derive(Template)]
#[template(path = "user/team_update.html")]
struct TeamUpdate<'a> {
    name: &'a str,
    msg_short: &'a str,
    msg_long: &'a str,
}
