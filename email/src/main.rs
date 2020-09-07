use mongodb::Client as Mongo;
use tonic::transport::Server;

use mmt::email::email_server::EmailServer;

mod models;
mod rpcimpl;
mod transport;

use rpcimpl::Email;
use transport::EmailTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Mongo::with_uri_str("mongodb://db:27017/").await.unwrap();
    let db = client.database("mmt_development");
    let transport = EmailTransport::default();

    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(EmailServer::new(Email::new(db, transport)))
        .serve(addr)
        .await?;

    Ok(())
}
