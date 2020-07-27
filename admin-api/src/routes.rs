use crate::{
        graphql::{
                context::CustomContext, mutation_root::MutationRoot, query_root::QueryRoot,
        },
};
use actix_web::{web, Error, HttpResponse};
use juniper::{
        http::{graphiql::graphiql_source, GraphQLRequest},
        EmptySubscription, RootNode,
};
use mongodb::Database;
use std::sync::Arc;
use stripe::{Client};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<CustomContext>>;

pub async fn graphiql() -> HttpResponse {
        let html = graphiql_source("http://localhost:8083/graphql", None);
        HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
}

pub async fn graphql(
        schema : web::Data<Arc<Schema>>,
        stripe : web::Data<Client>,
        db : web::Data<Database>,
        data : web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
        let context = CustomContext {
                db :     db.into_inner(),
                stripe : stripe.into_inner(),
        };

        let res = data.execute(&schema, &context).await;
        let res = serde_json::to_string(&res)?;
        Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(res))
}
