use crate::graphql::Gql;
use crate::ObjectId;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;
use yew::{
    services::ConsoleService,
    worker::{Agent, AgentLink, Context, HandlerId},
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/vehicle.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone, Default"
)]
pub struct Vehicle;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/vehicle_accept_ticket.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone, Default"
)]
pub struct VehicleAcceptTicket;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/vehicle_decline_ticket.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone, Default"
)]
pub struct VehicleDeclineTicket;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/vehicle_remove_ticket.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone, Default"
)]
pub struct VehicleRemoveTicket;

pub use vehicle::VehicleVehicle;
pub use vehicle::VehicleVehicleTickets;
pub use vehicle::VehicleVehicleTicketsUser;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    SetVehicleId(ObjectId),
    Accept(ObjectId),
    Decline(ObjectId),
    Remove(ObjectId),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Vehicle(VehicleVehicle),
}

pub enum Msg {
    GetVehicle,
    UpdateVehicle(VehicleVehicle),
}

pub struct VehicleAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,

    id: Option<ObjectId>,
    vehicle: Option<VehicleVehicle>,
}

impl Agent for VehicleAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),

            id: None,
            vehicle: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::GetVehicle => {
                if let Some(vehicle_id) = &self.id {
                    let vehicle_id = vehicle_id.clone();
                    let variables = vehicle::Variables { vehicle_id };

                    let link = self.link.clone();
                    spawn_local(async move {
                        let client = Gql::client();
                        let response = client.call(Vehicle, variables).await.unwrap();

                        if let Some(errors) = response.errors {
                            ConsoleService::log(&format!("{:?}", &errors));
                        } else if let Some(data) = response.data {
                            let vehicle = data.vehicle;
                            link.send_message(Msg::UpdateVehicle(vehicle));
                        }
                    });
                }
            }
            Msg::UpdateVehicle(vehicle) => {
                self.vehicle = Some(vehicle);

                if let Some(v) = &self.vehicle {
                    for who in &self.subscribers {
                        self.link.respond(*who, Response::Vehicle(v.clone()));
                    }
                }
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) {
        match msg {
            Request::SetVehicleId(id) => {
                self.id = Some(id);
                self.link.send_message(Msg::GetVehicle);
            }
            Request::Accept(ticket_id) => {
                if let Some(vehicle_id) = &self.id {
                    let vehicle_id = vehicle_id.clone();
                    let variables = vehicle_accept_ticket::Variables {
                        vehicle_id,
                        ticket_id,
                    };

                    let link = self.link.clone();
                    spawn_local(async move {
                        let client = Gql::client();
                        let response = client.call(VehicleAcceptTicket, variables).await.unwrap();

                        if let Some(errors) = response.errors {
                            ConsoleService::log(&format!("{:?}", &errors));
                        } else if let Some(_) = response.data {
                            link.send_message(Msg::GetVehicle);
                        }
                    });
                }
            }
            Request::Decline(ticket_id) => {
                if let Some(vehicle_id) = &self.id {
                    let vehicle_id = vehicle_id.clone();
                    let variables = vehicle_decline_ticket::Variables {
                        vehicle_id,
                        ticket_id,
                    };

                    let link = self.link.clone();
                    spawn_local(async move {
                        let client = Gql::client();
                        let response = client.call(VehicleDeclineTicket, variables).await.unwrap();

                        if let Some(errors) = response.errors {
                            ConsoleService::log(&format!("{:?}", &errors));
                        } else if let Some(_) = response.data {
                            link.send_message(Msg::GetVehicle);
                        }
                    });
                }
            }
            Request::Remove(ticket_id) => {
                if let Some(vehicle_id) = &self.id {
                    let vehicle_id = vehicle_id.clone();
                    let variables = vehicle_remove_ticket::Variables {
                        vehicle_id,
                        ticket_id,
                    };

                    let link = self.link.clone();
                    spawn_local(async move {
                        let client = Gql::client();
                        let response = client.call(VehicleRemoveTicket, variables).await.unwrap();

                        if let Some(errors) = response.errors {
                            ConsoleService::log(&format!("{:?}", &errors));
                        } else if let Some(_) = response.data {
                            link.send_message(Msg::GetVehicle);
                        }
                    });
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
