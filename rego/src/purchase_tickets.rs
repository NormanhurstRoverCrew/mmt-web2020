use crate::booking_summary::BookingSummary;
use crate::graphql::Gql;
use crate::ObjectId;
use crate::{
    app::AppRoute,
    booking_agent::{self, Booking, BookingAgent, Ticket, User},
};
use graphql_client::GraphQLQuery;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, services::ConsoleService};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/update_booking_tickets.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct UpdateBookingTickets;

impl From<update_booking_tickets::UpdateBookingTicketsUpdateBookingTickets> for Booking {
    fn from(b: update_booking_tickets::UpdateBookingTicketsUpdateBookingTickets) -> Self {
        let id = b.id;
        let index = b.no;
        let tickets: Vec<Ticket> = b.tickets.into_iter().map(|t| t.into()).collect();
        let user_id = b.user.id;
        let user = tickets
            .iter()
            .find(|t| &t.user.id == &user_id)
            .map(|t| t.user.clone())
            .unwrap();
        Self {
            id,
            user,
            tickets,
            index,
        }
    }
}

impl From<update_booking_tickets::UpdateBookingTicketsUpdateBookingTicketsTickets> for Ticket {
    fn from(t: update_booking_tickets::UpdateBookingTicketsUpdateBookingTicketsTickets) -> Self {
        let id = Some(t.id);
        let user = t.user.into();
        Self { id, user }
    }
}

impl From<update_booking_tickets::UpdateBookingTicketsUpdateBookingTicketsTicketsUser> for User {
    fn from(
        u: update_booking_tickets::UpdateBookingTicketsUpdateBookingTicketsTicketsUser,
    ) -> Self {
        let update_booking_tickets::UpdateBookingTicketsUpdateBookingTicketsTicketsUser {
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

pub struct PurchaseTickets {
    link: ComponentLink<Self>,
    booking_agent: Box<dyn Bridge<BookingAgent>>,
    router: RouteAgentDispatcher<()>,

    booking: Booking,

    errors: Vec<String>,
}

pub enum Msg {
    BookingUpdate(Booking),
    UpdateTicket(usize, Ticket),
    AddTicket,
    RemoveTicket(usize),
    Continue,
    UpdateBookingAgentBooking(Booking),
    SetErrors(Vec<String>),
    GotoCheckout,
}

impl Component for PurchaseTickets {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let booking_agent = BookingAgent::bridge(link.callback(|a| match a {
            booking_agent::Response::BookingUpdate(booking) => Msg::BookingUpdate(booking),
        }));

        let router = RouteAgentDispatcher::new();

        Self {
            link,
            router,
            booking_agent,
            booking: Booking::default(),
            errors: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BookingUpdate(booking) => {
                self.booking = booking;
                return true;
            }
            Msg::UpdateTicket(idx, ticket) => {
                self.booking_agent
                    .send(booking_agent::Request::UpdateTicket(idx, ticket));
            }
            Msg::AddTicket => self.booking_agent.send(booking_agent::Request::AddTicket),
            Msg::RemoveTicket(idx) => self
                .booking_agent
                .send(booking_agent::Request::RemoveTicket(idx)),
            Msg::Continue => {
                let booking = &self.booking;
                ConsoleService::log(&format!("booking: {:?}", &booking));

                let variables = update_booking_tickets::Variables {
                    booking_uid: booking.id.clone(),
                    tickets: booking
                        .tickets
                        .iter()
                        .map(|t| update_booking_tickets::BookingTicketUpdate {
                            id: t.id.clone(),
                            user: update_booking_tickets::BasicUser {
                                name: t.user.name.clone(),
                                email: t.user.email.clone(),
                                mobile: t.user.mobile.clone(),
                                crew: t.user.crew.clone(),
                            },
                        })
                        .collect(),
                };

                let link = self.link.clone();
                spawn_local(async move {
                    let client = Gql::client();
                    let response = client.call(UpdateBookingTickets, variables).await.unwrap();

                    if let Some(errors) = response.errors {
                        if let Some(error) = errors.first() {
                            if let Some(ext) = &error.extensions {
                                if let Some(serde_json::Value::String(s)) = ext.get("type") {
                                    if s == "FIELD_VALIDATION" {
                                        ConsoleService::log(&format!("{:?}", ext["advice"]));
                                        if let Some(advice) = ext["advice"].as_array() {
                                            link.send_message(Msg::SetErrors(
                                                advice
                                                    .iter()
                                                    .map(|adv| {
                                                        format!(
                                                            "Ticket {}: {} {}",
                                                            adv["idx"].as_u64().unwrap_or(42) + 1,
                                                            adv["field"],
                                                            adv["advice"]
                                                        )
                                                    })
                                                    .collect(),
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                    } else if let Some(data) = response.data {
                        let booking = data.update_booking_tickets;
                        link.send_message_batch(vec![
                            Msg::UpdateBookingAgentBooking(booking.into()),
                            Msg::GotoCheckout,
                        ]);
                    }
                });
            }
            Msg::UpdateBookingAgentBooking(booking) => self
                .booking_agent
                .send(booking_agent::Request::UpdateBooking(booking)),
            Msg::GotoCheckout => {
                let route = Route::from(AppRoute::Checkout);
                self.router.send(RouteRequest::ChangeRoute(route));
            }
            Msg::SetErrors(errors) => {
                self.errors = errors;
                return true;
            }
        };
        return false;
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let render_ticket = |idx: usize, ticket: &Ticket, mutable: bool| {
            let booking_agent::User {
                name,
                email,
                mobile,
                crew,
                ..
            } = &ticket.user;
            if mutable {
                let ticket = Rc::new(ticket.clone());

                let input = |ticket: Rc<Ticket>, f: Box<dyn Fn(&mut User, InputData)>| {
                    self.link.callback(move |input: InputData| {
                        let mut ticket = ticket.clone();
                        let ticket = Rc::make_mut(&mut ticket);
                        f(&mut ticket.user, input);
                        Msg::UpdateTicket(idx, ticket.clone())
                    })
                };

                let select = |ticket: Rc<Ticket>, f: Box<dyn Fn(&mut User, ChangeData)>| {
                    self.link.callback(move |input: ChangeData| {
                        let mut ticket = ticket.clone();
                        let ticket = Rc::make_mut(&mut ticket);
                        f(&mut ticket.user, input);
                        Msg::UpdateTicket(idx, ticket.clone())
                    })
                };

                Some(html! {
                    <div class="form mui-container-fluid editable">
                        <div class="ticket-header">
                            <div class="mui--text-title">{"Ticket "}{idx+1}</div>
                            <button type="button" class="mui-btn mui-btn--raised mui-btn--danger" onclick=self.link.callback(move |_| Msg::RemoveTicket(idx))>{"Remove"}</button>
                        </div>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" autocomplete="name" required=true value=name.clone() oninput=input(ticket.clone(), Box::new(|mut user, input| user.name = input.value)) />
                            <label>{"Name"}</label>
                            <p class="mui--text-caption">{"eg. Ru Paul"}</p>
                        </div>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="email" autocomplete="email" required=true value=email.clone() oninput=input(ticket.clone(), Box::new(|mut user, input| user.email = input.value)) />
                            <label>{"Email"}</label>
                            <p class="mui--text-caption">{"eg. example@gmail.com"}</p>
                        </div>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" autocomplete="phone" required=true value=mobile.clone() oninput=input(ticket.clone(), Box::new(|mut user, input| user.mobile = input.value)) />
                            <label>{"Mobile"}</label>
                            <p class="mui--text-caption">{"eg. 0400 000 000"}</p>
                        </div>

                        <div class="mui-select mui-select--float-label">
                            // <input type="text" name="crew" required=true value=self.crew.clone() oninput=self.link.callback(|e: InputData| Msg::SetCrew(e.value)) />
                            <select value=crew.clone() onchange=select(ticket.clone(), Box::new(|mut user, input| if let ChangeData::Select(sel) = input {
                                user.crew = sel.value()
                                })) >
                                {crate::crews::crews_option()}
                            </select>
                            <label>{"Crew"}</label>
                        </div>

                    </div>
                })
            } else {
                Some(html! {
                    <div class="mui-container-fluid">
                        <div class="mui--text-title">{"Ticket "}{idx+1}</div>
                        <div>
                            <div class="mui--text-caption">{"Name"}</div>
                            <div class="mui--text-body1">{name}</div>
                        </div>
                        <div>
                            <div class="mui--text-caption">{"Email"}</div>
                            <div class="mui--text-body1">{email}</div>
                        </div>
                        <div>
                            <div class="mui--text-caption">{"Mobile"}</div>
                            <div class="mui--text-body1">{mobile}</div>
                        </div>
                        <div>
                            <div class="mui--text-caption">{"Crew"}</div>
                            <div class="mui--text-body1">{crew}</div>
                        </div>
                    </div>
                })
            }
        };

        let errors: Vec<Html> = self.errors.iter().map(|e| html! { <p>{e}</p> }).collect();
        let error_msg = if !errors.is_empty() {
            "Please fix all errors before trying \"GO TO CHECKOUT\" again. If have more trouble please contact admin"
        } else {
            ""
        };

        html! {
            <div id="purchase-tickets">
                <div>
                    {self.booking.tickets.iter().enumerate().filter_map(|(i,t)| render_ticket(i, t, i != 0)).collect::<Html>()}

                    <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::AddTicket)>{"Add Another Ticket"}</button>
                </div>
                <div>
                    <BookingSummary />
                    <div class="errors">
                        {errors}
                        <div id="msg">{error_msg}</div>
                    </div>
                    <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::Continue)>{"Go to Checkout"}</button>
                </div>
            </div>
        }
    }
}
