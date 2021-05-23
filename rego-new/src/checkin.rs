use crate::graphql::Gql;
use crate::ObjectId;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, services::ConsoleService};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "get_vehicle_driver.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct GetDriver;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "driver_add_vehicle.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct DriverAddVehicle;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "passenger_add_vehicle.graphql",
    response_derives = "Debug, Serialize, Clone, Default"
)]
struct PassengerAddVehicle;

pub struct CheckIn {
    link: ComponentLink<Self>,
    props: Props,
    // router: RouteAgentDispatcher<()>,
    ticket_type: Option<TicketType>,
    rego: String,
    rego_driver: Result<String, String>,
    team_name: String,
}

pub enum TicketType {
    Driver,
    Passenger,
}

pub enum Msg {
    SetRego(String),
    SetTeamName(String),
    SetDriver(Result<String, String>),
    SetTicketType(TicketType),
    Submit,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: String,
}

impl Component for CheckIn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let router = RouteAgentDispatcher::new();

        Self {
            props,
            link,
            // router,
            rego: String::new(),
            rego_driver: Err("".to_owned()),
            team_name: String::new(),
            ticket_type: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                    let Props { ticket_id } = &self.props;
                    let ticket_id = ticket_id.to_owned();
                    let rego = self.rego.to_owned();
                    let name = self.team_name.to_owned();

                    let variables = driver_add_vehicle::Variables {
                        ticket_id,
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
                            // link.send_message(Msg::SetDriver(Ok(data.driver_name_from_rego)));
                        }
                    });
                    true
                }
                Some(TicketType::Passenger) => {
                    let Props { ticket_id } = &self.props;
                    let ticket_id = ticket_id.to_owned();
                    let rego = self.rego.to_owned();

                    let variables = passenger_add_vehicle::Variables { ticket_id, rego };

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
                            // link.send_message(Msg::SetDriver(Ok(data.driver_name_from_rego)));
                        }
                    });
                    true
                }
                _ => return false,
            },
        }
        // return false;
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let driver = match self.rego_driver {
            Ok(ref driver) => html! {
                <div>{format!("Driver: {}", driver)}</div>
            },
            Err(ref err) => html! {
                <div style={"color:red"}>{format!("Error: {}", err)}</div>
            },
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
                    <>
                        <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::SetTicketType(TicketType::Passenger))>{"I am passenger"}</button>
                        <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::SetTicketType(TicketType::Driver))>{"I'm a driver"}</button>
                    </>
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

        html! {
            <div>
                <div>{"Checkin"}</div>
                <div>
                    {part_1}
                    {button}
                </div>
            </div>
        }
    }
}

// impl CheckIn {
//     fn input_field(
//         display: &str,
//         description: &str,
//         value: &str,
//         name: &str,
//         required: bool,
//         oninput: Callback<InputData>,
//     ) -> Html {
//         html! {
//               <div class="mui-textfield mui-textfield--float-label">
//                   <input type="text" name=name required=required value=value oninput=oninput />
//                   <label>{display}</label>
//                   <p class="mui--text-caption">{description}</p>
//               </div>
//         }
//     }
// }
