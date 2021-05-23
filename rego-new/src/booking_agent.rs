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
pub struct Booking {
    pub id: ObjectId,
    pub user: User,
    pub tickets: Vec<Ticket>,
}

impl From<get_booking::GetBookingBookingFromUser> for Booking {
    fn from(b: get_booking::GetBookingBookingFromUser) -> Self {
        let id = b.id;
        let tickets: Vec<Ticket> = b.tickets.into_iter().map(|t| t.into()).collect();
        let user_id = b.user.id;
        let user = tickets
            .iter()
            .find(|t| &t.user.id == &user_id)
            .map(|t| t.user.clone())
            .unwrap();
        Self { id, user, tickets }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Option<ObjectId>,
    pub user: User,
}

impl From<get_booking::GetBookingBookingFromUserTickets> for Ticket {
    fn from(t: get_booking::GetBookingBookingFromUserTickets) -> Self {
        let id = Some(t.id);
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

impl From<get_booking::GetBookingBookingFromUserTicketsUser> for User {
    fn from(u: get_booking::GetBookingBookingFromUserTicketsUser) -> Self {
        let get_booking::GetBookingBookingFromUserTicketsUser {
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
    schema_path = "schema.json",
    query_path = "get_booking.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct GetBooking;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    SetUserUid(ObjectId),
    AddTicket,
    RemoveTicket(usize),
    UpdateTicket(usize, Ticket),
    UpdateBooking(Booking),
    SyncTickets,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    BookingUpdate(Booking),
}

pub enum Msg {
    Init,
    GetBookingIdFromUserId,
    SetUserId(ObjectId),
    UpdateBooking(Booking),
}

pub struct BookingAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,

    user_uid: ObjectId,
    booking: Option<Booking>,
}

impl Agent for BookingAgent {
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
            booking: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Init => {
                let storage = StorageService::new(Area::Local).expect("storage disabled by user");
                let uid = storage.restore::<Json<anyhow::Result<ObjectId>>>("user_uid");

                if let Json(Ok(uid)) = uid {
                    self.user_uid = uid;
                    self.link.send_message(Msg::GetBookingIdFromUserId);
                }
            }
            Msg::GetBookingIdFromUserId => {
                let variables = get_booking::Variables {
                    uid: self.user_uid.clone(),
                };

                let link = self.link.clone();
                spawn_local(async move {
                    let client = Gql::client();
                    let response = client.call(GetBooking, variables).await.unwrap();

                    if let Some(errors) = response.errors {
                        ConsoleService::log(&format!("{:?}", &errors));
                    } else if let Some(data) = response.data {
                        let booking = data.booking_from_user;
                        link.send_message(Msg::UpdateBooking(booking.into()));
                    }
                });
            }
            Msg::SetUserId(uid) => {
                self.user_uid = uid.clone();

                let mut storage =
                    StorageService::new(Area::Local).expect("storage disabled by user");
                storage.store("user_uid", Ok(serde_json::to_string(&uid).unwrap()));

                self.link.send_message(Msg::GetBookingIdFromUserId);
            }
            Msg::UpdateBooking(booking) => {
                self.booking = Some(booking);

                ConsoleService::log(&format!("UpdateBooking: {:?}", &self.booking));
                if let Some(booking) = &self.booking {
                    for who in &self.subscribers {
                        self.link
                            .respond(*who, Response::BookingUpdate(booking.clone()));
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
            Request::AddTicket => {
                if let Some(ref mut booking) = self.booking {
                    let ticket = Ticket::default();
                    booking.tickets.push(ticket);

                    for who in &self.subscribers {
                        self.link
                            .respond(*who, Response::BookingUpdate(booking.clone()));
                    }
                }
            }
            Request::RemoveTicket(idx) => {
                if let Some(ref mut booking) = self.booking {
                    if idx < booking.tickets.len() {
                        booking.tickets.remove(idx);

                        for who in &self.subscribers {
                            self.link
                                .respond(*who, Response::BookingUpdate(booking.clone()));
                        }
                    }
                }
            }
            Request::UpdateTicket(idx, ticket) => {
                if let Some(ref mut booking) = self.booking {
                    if let Some(t) = booking.tickets.get_mut(idx) {
                        *t = ticket;
                    }
                    for who in &self.subscribers {
                        self.link
                            .respond(*who, Response::BookingUpdate(booking.clone()));
                    }
                }
            }
            Request::UpdateBooking(booking) => {
                self.booking = Some(booking);

                if let Some(booking) = &self.booking {
                    for who in &self.subscribers {
                        self.link
                            .respond(*who, Response::BookingUpdate(booking.clone()));
                    }
                }
            }
            Request::SyncTickets => {
                if let Some(booking) = &self.booking {
                    for who in &self.subscribers {
                        self.link
                            .respond(*who, Response::BookingUpdate(booking.clone()));
                    }
                }
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
