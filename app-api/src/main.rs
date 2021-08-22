#[macro_use]
extern crate lazy_static;

use actix_cors::Cors;
use actix_web::{
    dev::ServiceRequest,
    http::header,
    middleware::{self, Logger},
    web, App, Error, HttpResponse, HttpServer,
};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use custom_context::CustomContext;
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptySubscription, RootNode,
};
use models::admin_user::AdminUser;
use mongodb::{Client as Mongo, Database};
use mutation_root::MutationRoot;
use query_root::QueryRoot;
use std::sync::Arc;
use stripe::Client;

mod auth;
mod custom_context;
mod errors;
mod models;
mod mutation_root;
mod query_root;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default)
        .scope("openid profile");
    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let client = Mongo::with_uri_str("mongodb://localhost:27017/")
        .await
        .unwrap();
    let db = client.database("mmt_development");

    let stripe = std::env::var("STRIPE_API_KEY").expect("Stripe Api Key");
    let stripe = Client::new(stripe);

    // Create Juniper schema
    let schema = std::sync::Arc::new(Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::<CustomContext>::new(),
    ));

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:8081")
            // .allowed_origin("http://localhost:8083")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(auth)
            .wrap(cors)
            .data(schema.clone())
            .data(db.clone())
            .data(stripe.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("0.0.0.0:8069")?
    .run()
    .await
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<CustomContext>>;

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8083/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    schema: web::Data<Arc<Schema>>,
    stripe: web::Data<Client>,
    db: web::Data<Database>,
    // email: web::Data<Endpoint>,
    data: web::Json<GraphQLRequest>,
    admin_user: AdminUser,
) -> Result<HttpResponse, Error> {
    let context = CustomContext {
        db: db.into_inner(),
        admin_user,
        stripe: stripe.into_inner(),
    };

    let res = data.execute(&schema, &context).await;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(res))
}
