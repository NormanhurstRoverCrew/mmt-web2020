use lettre::{
    smtp::{
        authentication::{Credentials, Mechanism},
        error::SmtpResult,
        extension::ClientId,
        ConnectionReuseParameters,
    },
    SendableEmail, SmtpClient, SmtpTransport, Transport,
};

#[derive(Default, Debug)]
pub struct EmailTransport {}

impl EmailTransport {
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

    pub fn send<E: Into<SendableEmail>>(&self, email: E) -> SmtpResult {
        Self::transport().send(email.into())
    }
}
