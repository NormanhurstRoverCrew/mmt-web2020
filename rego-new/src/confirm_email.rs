use crate::graphql::Gql;
use graphql_client::GraphQLQuery;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::{agent::Dispatched, prelude::*};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher, RouteService},
};

use crate::{
    app::AppRoute,
    booking_agent::{self, BookingAgent},
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "verify_user.graphql",
    response_derives = "Debug"
)]
struct VerifyUser;

pub struct ConfirmEmail {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,

    error: Option<String>,
    loading: bool,
}

pub enum Msg {
    SetLoading(bool),
    SetError(Option<String>),
    VerifyUser { uid: String, code: String },
    Verified,
}

impl Component for ConfirmEmail {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service: RouteService<()> = RouteService::new();
        let router = RouteAgentDispatcher::new();

        let mut query = route_service.get_query();
        if query.len() > 0 {
            query.remove(0);
        }
        let query: HashMap<&str, &str> = querystring::querify(&query).into_iter().collect();

        let code = query.get("code").map(|c| c.to_owned().to_owned());
        let uid = query.get("uid").map(|c| c.to_owned().to_owned());
        let error = if let (Some(uid), Some(code)) = (uid, code) {
            link.send_message(Msg::VerifyUser { uid, code });
            None
        } else {
            Some("Validation url missing part of query string".to_owned())
        };

        Self {
            link,
            router,
            loading: false,
            error,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetLoading(loading) => self.loading = loading,
            Msg::SetError(error) => self.error = error,
            Msg::VerifyUser { uid, code } => {
                let variables = verify_user::Variables {
                    uid: uid.clone(),
                    code,
                };

                let link = self.link.clone();
                spawn_local(async move {
                    link.send_message(Msg::SetLoading(true));

                    let client = Gql::client();
                    let response = client.call(VerifyUser, variables).await.unwrap();

                    if let Some(errors) = response.errors {
                        let err = errors.get(0).unwrap();
                        link.send_message_batch(vec![
                            Msg::SetError(Some(err.message.clone())),
                            Msg::SetLoading(false),
                        ]);
                    } else if let Some(verify_user::ResponseData {
                        verify_user:
                            verify_user::VerifyUserVerifyUser {
                                email_verified: true,
                            },
                    }) = response.data
                    {
                        link.send_message_batch(vec![
                            Msg::SetError(None),
                            Msg::SetLoading(false),
                            Msg::Verified,
                        ]);
                        let mut booking_agent = BookingAgent::dispatcher();
                        booking_agent.send(booking_agent::Request::SetUserUid(uid));
                    } else {
                        unreachable!();
                    }
                });
            }
            Msg::Verified => {
                let route = Route::from(AppRoute::PurchaseTickets);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
        };
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        if let Some(error) = &self.error {
            return html! {
                <p>{"Error: "}{error}</p>
            };
        }
        if self.loading {
            return html! {
                <p>{"Please wait while we verify that your are who you say you are"}</p>
            };
        }

        html! {"Verified! You should see another page now..."}
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
