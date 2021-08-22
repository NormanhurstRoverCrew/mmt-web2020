use crate::graphql::Gql;
use crate::ObjectId;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use yew::{
    format::Json,
    services::{storage::Area, ConsoleService, StorageService},
    worker::{Agent, AgentLink, Context, HandlerId},
};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: ObjectId,
    pub user: User,
}

impl From<get_ticket::GetTicketUserTicket> for Ticket {
    fn from(t: get_ticket::GetTicketUserTicket) -> Self {
        let id = t.id;
        let user = t.user.into();
        Self { id, user }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: ObjectId,

    pub name: String,
    pub email: String,
    pub mobile: String,
    pub crew: String,
}

impl From<get_ticket::GetTicketUserTicketUser> for User {
    fn from(u: get_ticket::GetTicketUserTicketUser) -> Self {
        let get_ticket::GetTicketUserTicketUser {
            name,
            email,
            mobile,
            crew,
            id,
        } = u;
        Self {
            id,
            name,
            email,
            mobile,
            crew,
        }
    }
}

// pub type Booking = get_booking::GetBookingBookingFromUser;
// pub type Ticket = get_booking::GetBookingBookingFromUserTickets;
// pub type User = get_booking::GetBookingBookingFromUserTicketsUser;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/get_ticket.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct GetTicket;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    SetUserUid(ObjectId),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    TicketUpdate(Ticket),
}

pub enum Msg {
    Init,
    GetTicketFromUserId,
    SetUserId(ObjectId),
    UpdateTicket(Ticket),
}

pub struct TicketAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,

    user_uid: ObjectId,
    ticket: Option<Ticket>,
}

impl Agent for TicketAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        link.send_message(Msg::Init);
        Self {
            link,
            subscribers: HashSet::new(),

            user_uid: "".to_string(),
            ticket: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Init => {
                let storage = StorageService::new(Area::Local).expect("storage disabled by user");
                let uid = storage.restore::<Json<anyhow::Result<ObjectId>>>("user_uid");

                if let Json(Ok(uid)) = uid {
                    self.user_uid = uid;
                    self.link.send_message(Msg::GetTicketFromUserId);
                }
            }
            Msg::GetTicketFromUserId => {
                let variables = get_ticket::Variables {
                    uid: self.user_uid.clone(),
                };

                let link = self.link.clone();
                spawn_local(async move {
                    let client = Gql::client();
                    let response = client.call(GetTicket, variables).await.unwrap();

                    if let Some(errors) = response.errors {
                        ConsoleService::log(&format!("{:?}", &errors));
                    } else if let Some(data) = response.data {
                        if let Some(ticket) = data.user.ticket {
                            link.send_message(Msg::UpdateTicket(ticket.into()));
                        }
                    }
                });
            }
            Msg::SetUserId(uid) => {
                self.user_uid = uid.clone();

                let mut storage =
                    StorageService::new(Area::Local).expect("storage disabled by user");
                storage.store("user_uid", Ok(serde_json::to_string(&uid).unwrap()));

                self.link.send_message(Msg::GetTicketFromUserId);
            }
            Msg::UpdateTicket(ticket) => {
                self.ticket = Some(ticket);

                ConsoleService::log(&format!("UpdateTicket: {:?}", &self.ticket));
                if let Some(ticket) = &self.ticket {
                    for who in &self.subscribers {
                        self.link
                            .respond(*who, Response::TicketUpdate(ticket.clone()));
                    }
                }
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) {
        match msg {
            Request::SetUserUid(user_uid) => {
                self.link.send_message(Msg::SetUserId(user_uid));
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
