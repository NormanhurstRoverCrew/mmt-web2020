use crate::booking_agent::{self, BookingAgent, Ticket};
use crate::graphql::Gql;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::Properties;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/price.graphql",
    response_derives = "Debug"
)]
struct Price;

pub struct BookingSummary {
    link: ComponentLink<Self>,
    // booking_agent: Box<dyn Bridge<BookingAgent>>,
    // router: RouteAgentDispatcher<()>,
    render_details: bool,

    #[allow(unused)]
    booking_agent: Box<dyn Bridge<BookingAgent>>,
    tickets: Vec<Ticket>,

    price: Option<f64>,
    stripe_rate: Option<f64>,
    stripe: bool,
}

pub enum Msg {
    UpdateTickets(Vec<Ticket>),
    GetPrice,
    SetPrice(f64, f64),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub details: bool,

    #[prop_or(false)]
    pub stripe: bool,
}

impl Component for BookingSummary {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let booking_agent = BookingAgent::bridge(link.callback(|a| match a {
        //     booking_agent::Response::BookingUpdate(booking) => Msg::BookingUpdate(booking),
        // }));

        // let router = RouteAgentDispatcher::new();

        let booking_agent = BookingAgent::bridge(link.callback(|a| match a {
            booking_agent::Response::BookingUpdate(booking) => Msg::UpdateTickets(booking.tickets),
        }));

        link.send_message(Msg::GetPrice);
        Self {
            link,
            // router,
            // booking_agent,
            render_details: props.details,
            booking_agent,

            tickets: vec![],

            price: None,
            stripe_rate: None,
            stripe: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTickets(tickets) => {
                self.tickets = tickets;
                return true;
            }
            Msg::GetPrice => {
                let link = self.link.clone();
                spawn_local(async move {
                    let client = Gql::client();
                    let response = match client.call(Price, price::Variables).await {
                        Ok(r) => r,
                        Err(e) => {
                            ConsoleService::log(&format!("Network error: {:?}", e));
                            return;
                        }
                    };
                    if let Some(errors) = response.errors {
                        ConsoleService::log(&format!("Resp errors {:?}", errors));
                    } else if let Some(data) = response.data {
                        link.send_message(Msg::SetPrice(data.ticket_price, data.stripe_rate));
                    }
                });
                return false;
            }
            Msg::SetPrice(price, stripe_rate) => {
                self.price = Some(price);
                self.stripe_rate = Some(stripe_rate);
                return true;
            }
        };
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.stripe = props.stripe;
        true
    }

    fn view(&self) -> Html {
        let render_ticket = |(i, ticket): (usize, &Ticket)| {
            html! {
                <div class="ticket">
                    <div class="mui--text-subhead">
                        {format!("Ticket {}", i + 1)}
                    </div>
                    <div>{&ticket.user.name}</div>
                    <div>{&ticket.user.email}</div>
                    <div>{&ticket.user.mobile}</div>
                    <div>{&ticket.user.crew}</div>
                </div>
            }
        };

        let details = if self.render_details {
            html! {
                <div class="mui--divider-top">
                <div>
                    {self.tickets.iter().enumerate().map(render_ticket).collect::<Html>()}
                </div>
                </div>
            }
        } else {
            html! {}
        };

        let fee = if self.stripe {
            html! {
                <>
                    <div class="fee-name">{"Stripe Fee"}</div>
                    <div class="fee-price line">{self.stripe_fee()}</div>
                </>
            }
        } else {
            html! {}
        };

        html! {
            <div id="booking-summary" class="mui-panel">
                <p class="summary-title">{"Booking Summary"}</p>
                <div class="grid">
                        <div class="item-name">{"MMT 2021 Ticket"}</div>
                        <div class="item-qty-name right">{"Qty"}</div>
                        <div class="item-price line">{self.ticket_price()}</div>
                        <div class="item-quantity line right">{self.tickets.len()}</div>
                        {fee}
                        <div class="total-name">{"Total"}</div>
                        <div class="total">{self.total_price()}</div>
                </div>
                {details}
            </div>
        }
    }
}

impl BookingSummary {
    fn ticket_price(&self) -> String {
        value(self.price)
    }

    fn stripe_fee(&self) -> String {
        value(self._stripe_fee())
    }

    fn total_price(&self) -> String {
        value(match self.stripe {
            false => self._total_price(),
            true => self._stripe_total(),
        })
    }

    fn _stripe_fee(&self) -> Option<f64> {
        self._stripe_total()
            .zip(self._total_price())
            .map(|(s, t)| s - t)
    }

    fn _stripe_total(&self) -> Option<f64> {
        self._total_price()
            .zip(self.stripe_rate)
            .map(|(price, rate)| (price + 0.3) / (1.0 - rate))
    }

    fn _total_price(&self) -> Option<f64> {
        self.price.map(|price| price * self.tickets.len() as f64)
    }
}

fn value(v: Option<f64>) -> String {
    v.map(|price| format!("${:.02}", price))
        .unwrap_or(String::from("$--.--"))
}
