use crate::booking_agent::{self, Booking, BookingAgent, Ticket};
use yew::prelude::*;
use yew::Properties;

pub struct BookingSummary {
    // link: ComponentLink<Self>,
    // booking_agent: Box<dyn Bridge<BookingAgent>>,
    // router: RouteAgentDispatcher<()>,
    render_details: bool,

    booking_agent: Box<dyn Bridge<BookingAgent>>,
    tickets: Vec<Ticket>,
}

pub enum Msg {
    UpdateTickets(Vec<Ticket>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub details: bool,
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
        Self {
            // link,
            // router,
            // booking_agent,
            render_details: props.details,
            booking_agent,

            tickets: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTickets(tickets) => {
                self.tickets = tickets;
                return true;
            }
        };
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let render_ticket = |(i, ticket): (usize, &Ticket)| {
            html! {
                <div>
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
                <p class="mui--text-headline">{"Details"}</p>
                <div>
                    {self.tickets.iter().enumerate().map(render_ticket).collect::<Html>()}
                </div>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <div id="booking-summary" class="mui-panel">
                <p class="mui--text-headline">{"Booking Summary"}</p>
                <table>
                    <th>
                        <td>{"MMT 2021 Ticket"}</td>
                        <td>{"Amount"}</td>
                    </th>
                    <tr>
                        <td>{self.ticket_price()}</td>
                        <td>{self.tickets.len()}</td>
                    </tr>
                </table>
                <div class="mui--divider-top">
                    <p class="mui--text-title">{"Total"}</p>
                    <p class="mui--text-display1">{self.total_price()}</p>
                </div>
                {details}
            </div>
        }
    }
}

impl BookingSummary {
    fn ticket_price(&self) -> Html {
        html! {
            {"$40.00"}
        }
    }

    fn total_price(&self) -> Html {
        let maths: f32 = 40.0 * self.tickets.len() as f32;
        html! {
            {format!("${:.02}", maths)}
        }
    }
}
