use crate::graphql::Gql;
use crate::ticket_agent::{self, Ticket, TicketAgent};
use crate::ObjectId;
use graphql_client::GraphQLQuery;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, services::ConsoleService};
use yew_router::prelude::RouteService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/get_vehicle_driver.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct GetDriver;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/driver_add_vehicle.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct DriverAddVehicle;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/passenger_add_vehicle.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct PassengerAddVehicle;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/verify_user.graphql",
    response_derives = "Debug"
)]
struct VerifyUser;

pub struct CheckIn {
    link: ComponentLink<Self>,
    #[allow(unused)]
    ticket_agent: Box<dyn Bridge<TicketAgent>>,
    ticket: Option<Ticket>,
    // router: RouteAgentDispatcher<()>,
    ticket_type: Option<TicketType>,
    rego: String,
    rego_driver: Result<String, String>,
    team_name: String,

    post: bool,
}

pub enum TicketType {
    Driver,
    Passenger,
}

pub enum Msg {
    VerifyUser { uid: String, code: String },
    SetRego(String),
    SetTeamName(String),
    SetDriver(Result<String, String>),
    SetTicketType(TicketType),
    TicketUpdate(Ticket),
    Submit,
    ShowPostMessage,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: String,
}

impl Component for CheckIn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service: RouteService<()> = RouteService::new();
        // let router = RouteAgentDispatcher::new();

        // Remove the '&' iff it exists
        let mut query = route_service.get_query();
        if query.len() > 0 {
            query.remove(0);
        }
        let query: HashMap<&str, &str> = querystring::querify(&query).into_iter().collect();

        if let Some(code) = query.get("code").map(|c| c.to_owned().to_owned()) {
            link.send_message(Msg::VerifyUser {
                uid: props.user_id.clone(),
                code,
            });
        }

        let mut ticket_agent = TicketAgent::bridge(link.callback(|a| match a {
            ticket_agent::Response::TicketUpdate(ticket) => Msg::TicketUpdate(ticket),
        }));

        ticket_agent.send(ticket_agent::Request::SetUserUid(props.user_id));

        Self {
            ticket_agent,
            link,
            ticket: None,
            // router,
            rego: String::new(),
            rego_driver: Err("".to_owned()),
            team_name: String::new(),
            ticket_type: None,
            post: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::VerifyUser { uid, code } => {
                let variables = verify_user::Variables {
                    uid: uid.clone(),
                    code,
                };

                // let link = self.link.clone();
                spawn_local(async move {
                    // link.send_message(Msg::SetLoading(true));

                    let client = Gql::client();
                    let response = client.call(VerifyUser, variables).await.unwrap();

                    if let Some(errors) = response.errors {
                        let err = errors.get(0).unwrap();
                        ConsoleService::log(&format!("Verify Errors: {}", err));
                        // link.send_message_batch(vec![
                        //     Msg::SetError(Some(err.message.clone())),
                        //     Msg::SetLoading(false),
                        // ]);
                    } else if let Some(verify_user::ResponseData {
                        verify_user:
                            verify_user::VerifyUserVerifyUser {
                                email_verified: true,
                            },
                    }) = response.data
                    {
                        ConsoleService::log("Verified");
                    } else {
                        unreachable!();
                    }
                });

                return false;
            }
            Msg::TicketUpdate(ticket) => {
                self.ticket = Some(ticket);
                return true;
            }
            Msg::SetRego(rego) => {
                self.rego = rego.clone();

                let variables = get_driver::Variables { rego };

                let link = self.link.clone();
                spawn_local(async move {
                    let client = Gql::client();
                    let response = match client.call(GetDriver, variables).await {
                        Ok(r) => r,
                        Err(e) => {
                            ConsoleService::log(&format!("Network error: {:?}", e));
                            return;
                        }
                    };

                    if let Some(errors) = response.errors {
                        ConsoleService::log(&format!("Resp errors {:?}", errors));
                        link.send_message(Msg::SetDriver(Err(errors
                            .first()
                            .unwrap()
                            .message
                            .clone())));
                    } else if let Some(data) = response.data {
                        ConsoleService::log(&format!("Resp data {:?}", data));
                        link.send_message(Msg::SetDriver(Ok(data.driver_name_from_rego)));
                    }
                });
                return true;
            }
            Msg::SetDriver(sd) => {
                self.rego_driver = sd;
                return true;
            }
            Msg::SetTeamName(team_name) => {
                self.team_name = team_name;
                return true;
            }
            Msg::SetTicketType(tt) => {
                self.ticket_type = Some(tt);
                return true;
            }
            Msg::Submit => match self.ticket_type {
                Some(TicketType::Driver) => {
                    if let Some(ticket) = &self.ticket {
                        let rego = self.rego.to_owned();
                        let name = self.team_name.to_owned();

                        let variables = driver_add_vehicle::Variables {
                            ticket_id: ticket.id.clone(),
                            rego,
                            name,
                        };

                        let link = self.link.clone();
                        spawn_local(async move {
                            let client = Gql::client();
                            let response = match client.call(DriverAddVehicle, variables).await {
                                Ok(r) => r,
                                Err(e) => {
                                    ConsoleService::log(&format!("Network error: {:?}", e));
                                    return;
                                }
                            };

                            if let Some(errors) = response.errors {
                                ConsoleService::log(&format!("Resp errors {:?}", errors));
                                link.send_message(Msg::SetDriver(Err(errors
                                    .first()
                                    .unwrap()
                                    .message
                                    .clone())));
                            } else if let Some(data) = response.data {
                                ConsoleService::log(&format!("Resp data {:?}", data));
                                link.send_message(Msg::ShowPostMessage);
                            }
                        });
                    }
                    true
                }
                Some(TicketType::Passenger) => {
                    if let Some(ticket) = &self.ticket {
                        let rego = self.rego.to_owned();

                        let variables = passenger_add_vehicle::Variables {
                            ticket_id: ticket.id.clone(),
                            rego,
                        };

                        let link = self.link.clone();
                        spawn_local(async move {
                            let client = Gql::client();
                            let response = match client.call(PassengerAddVehicle, variables).await {
                                Ok(r) => r,
                                Err(e) => {
                                    ConsoleService::log(&format!("Network error: {:?}", e));
                                    return;
                                }
                            };

                            if let Some(errors) = response.errors {
                                ConsoleService::log(&format!("Resp errors {:?}", errors));
                                link.send_message(Msg::SetDriver(Err(errors
                                    .first()
                                    .unwrap()
                                    .message
                                    .clone())));
                            } else if let Some(data) = response.data {
                                ConsoleService::log(&format!("Resp data {:?}", data));
                                link.send_message(Msg::ShowPostMessage);
                            }
                        });
                    }
                    true
                }
                _ => return false,
            },
            Msg::ShowPostMessage => {
                self.post = true;
                return true;
            }
        }
        // return false;
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let driver = match (self.rego_driver.as_ref(), self.ticket_type.as_ref()) {
            (Ok(_), Some(TicketType::Driver)) => html! {
                <div>{format!("This Registration is already taken. If you are the owner of this rego, please check that you havent already registered, or contact admin")}</div>
            },
            (Ok(driver), _) => html! {
                <div>{format!("Driver: {}", driver)}</div>
            },
            (Err(err), Some(TicketType::Driver)) if err.as_str() == "Vehicle does not exist" => {
                html! {
                    <></>
                }
            }
            (Err(err), _) if !err.is_empty() => html! {
                <div style={"color:red"}>{format!("Error: {}", err)}</div>
            },
            _ => html! {<></>},
        };
        let rego = html! {
            <>
                <div class="mui-textfield mui-textfield--float-label">
                    <input type="text" name="rego" value=self.rego.clone() required=true oninput=self.link.callback(|e: InputData| Msg::SetRego(e.value)) />
                    <label>{"Vehicle Registration"}</label>
                    // <p class="mui--text-caption">{description}</p>
                </div>
                {driver}
            </>
        };

        let part_1 = match self.ticket_type {
            Some(TicketType::Driver) => {
                html! {
                    <>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" name="team_name" required=true value=self.team_name.clone() oninput=self.link.callback(|e: InputData| Msg::SetTeamName(e.value)) />
                            <label>{"Team Name"}</label>
                            // <p class="mui--text-caption">{description}</p>
                        </div>
                        {rego}
                    </>
                }
            }
            Some(_) => {
                html! {
                <>
                    {rego}
                </>
                }
            }
            None => {
                html! {
                    <></>
                }
            }
        };

        let button = match self.ticket_type {
            Some(TicketType::Passenger) => {
                let disable = self.rego_driver.is_err() || self.rego.len() == 0;
                html! {
                    <button type="button" class="mui-btn mui-btn--raised" disabled=disable onclick=self.link.callback(|_| Msg::Submit)>{"Submit"}</button>
                }
            }
            Some(TicketType::Driver) => {
                let disable = self.rego_driver.is_ok() || self.rego.len() == 0;
                html! {
                    <>
                        <button type="button" class="mui-btn mui-btn--raised" disabled=disable onclick=self.link.callback(|_| Msg::Submit)>{"Submit"}</button>
                    </>
                }
            }
            None => html! {<></>},
        };

        let event = html! {<p>{"Please join the facebook event to receive information leading up to the event"}</p>};

        let post_message = match self.ticket_type {
            Some(TicketType::Driver) if self.post => html! {
                <>
                    <p>
                        {"You've created a team! Now you need to add people to it. Please tell the passengers in your car to sign up with the rego you provided above. When they do, you will get an email asking to accept or decline them for your team. The passenger will be told if they have been accepted or declined."}
                    </p>
                    {event}
                </>
            },
            Some(TicketType::Passenger) if self.post => html! {
                <>
                    <p>
                        {"You've requested to join a team. The driver will be asked to accept or decline your request soon."}
                    </p>
                    {event}
                </>
            },
            _ => html! {<></>},
        };

        html! {
            <div id="checkin">
                <h1>{"Check in"}</h1>
                <p>
                    {"Passengers - Please talk to the driver of your vehicle to find out what their Vehicle registration is. The driver must have already registered below before you can join their team"}
                </p>
                <p>
                    {"Drivers - Please create your team below. If you are unsure of your team name put anything down, You'll be able to change this later."}<br/>
                    {"If you are hiring a vehicle, please put down `HIREXXXX` and replace XX with your crew name eg `HIRENORMO`. You will be required to change your rego on the day."}
                </p>
                <div>
                    <div>
                        <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::SetTicketType(TicketType::Passenger))>{"I'm a passenger"}</button>
                        <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::SetTicketType(TicketType::Driver))>{"I'm a driver"}</button>
                    </div>
                    {part_1}
                    {button}
                    {post_message}
                </div>
            </div>
        }
    }
}
