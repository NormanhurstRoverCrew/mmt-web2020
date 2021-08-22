use crate::vehicle_agent::{self, VehicleAgent, VehicleVehicle};
use crate::ObjectId;
use yew::prelude::*;

pub struct ManageVehicle {
    link: ComponentLink<Self>,
    vehicle_agent: Box<dyn Bridge<VehicleAgent>>,
    // router: RouteAgentDispatcher<()>,
    vehicle: Option<VehicleVehicle>,
}

pub enum Msg {
    UpdateVehicle(VehicleVehicle),
    Accept(ObjectId),
    Decline(ObjectId),
    Remove(ObjectId),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub vehicle_id: String,
}

impl Component for ManageVehicle {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let router = RouteAgentDispatcher::new();

        let mut vehicle_agent = VehicleAgent::bridge(link.callback(|a| match a {
            vehicle_agent::Response::Vehicle(v) => Msg::UpdateVehicle(v),
        }));

        vehicle_agent.send(vehicle_agent::Request::SetVehicleId(props.vehicle_id));

        Self {
            vehicle_agent,
            link,

            vehicle: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateVehicle(v) => {
                self.vehicle = Some(v);
                true
            }
            Msg::Accept(t) => {
                self.vehicle_agent.send(vehicle_agent::Request::Accept(t));
                false
            }
            Msg::Decline(t) => {
                self.vehicle_agent.send(vehicle_agent::Request::Decline(t));
                false
            }
            Msg::Remove(t) => {
                self.vehicle_agent.send(vehicle_agent::Request::Remove(t));
                false
            }
        }
        // return false;
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let link = self.link.clone();
        let vehicle = match &self.vehicle {
            Some(v) => {
                let team: Vec<Html> = v
                    .tickets
                    .iter()
                    .map(|ticket| match &ticket.user {
                        Some(u) => {
                            let ticket_id = ticket.id.clone();
                            html! {
                                <div class="ticket">
                                    <p class="name">{&u.name}</p>
                                    <p class="crew">{&u.crew}</p>
                                    <div class="actions">
                                        <button type="button" class="mui-btn mui-btn--raised" onclick=link.callback(move |_| Msg::Remove(ticket_id.clone()))>{"Remove from team"}</button>
                                    </div>
                                </div>
                            }
                        },
                        None => html! {
                            <div class="ticket">
                                {"No user for ticket"}
                            </div>
                        },
                    })
                    .collect();

                let requesting: Vec<Html> = v
                    .requests
                    .iter()
                    .map(|ticket| match &ticket.user {
                        Some(u) => {
                            let ticket_id1 = ticket.id.clone();
                            let ticket_id2 = ticket.id.clone();
                            html! {
                            <div class="ticket">
                                <p class="name">{&u.name}</p>
                                <p class="crew">{&u.crew}</p>
                                <div class="actions">
                                    <button type="button" class="mui-btn mui-btn--raised" onclick=link.callback(move |_| Msg::Accept(ticket_id1.clone()))>{"Accept"}</button>
                                    <button type="button" class="mui-btn mui-btn--raised" onclick=link.callback(move |_| Msg::Decline(ticket_id2.clone()))>{"Decline"}</button>
                                </div>
                            </div>
                        }},
                        None => html! {
                            <div class="ticket">
                                {"No user for ticket"}
                            </div>
                        },
                    })
                    .collect();

                let requesting = match requesting.is_empty() {
                    true => html! {
                        <p>{"No Requests at this time."}</p>
                    },
                    false => html! {
                        <div class="tickets requesting">
                            {requesting}
                        </div>
                    },
                };

                html! {
                    <div class="vehicle">
                        <h1>{format!("Team: {}", &v.name)}</h1>
                        <p>{format!("Rego: {}", &v.rego)}</p>
                        <p>{format!("Driver: {}", &v.driver.as_ref().unwrap_or(&" - ".to_string()))}</p>
                        <h2>{"Your team"}</h2>
                        <div class="tickets team">
                            {team}
                        </div>
                        <h2>{"Requests"}</h2>
                        {requesting}
                    </div>
                }
            }
            None => html! {
                <p>{"Loading..."}</p>
            },
        };
        html! {
            <div id="manage-vehicle">
            {vehicle}
            </div>
        }
    }
}
