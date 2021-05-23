use crate::graphql::Gql;
use graphql_client::GraphQLQuery;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "new_user.graphql",
    response_derives = "Debug"
)]
struct NewUser;

pub struct NewBookingForm {
    link: ComponentLink<Self>,
    name: String,
    email: String,
    mobile: String,
    crew: String,
    loading: bool,
    check_email: bool,
}

pub enum Msg {
    SetName(String),
    SetEmail(String),
    SetMobile(String),
    SetCrew(String),
    SubmitBooking,
    SetLoading(bool),
    CheckEmail,
}

impl Component for NewBookingForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::new(),
            email: String::new(),
            mobile: String::new(),
            crew: String::new(),
            loading: false,
            check_email: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetName(name) => self.name = name,
            Msg::SetEmail(email) => self.email = email,
            Msg::SetMobile(mobile) => self.mobile = mobile,
            Msg::SetCrew(crew) => self.crew = crew,
            Msg::SetLoading(loading) => self.loading = loading,
            Msg::CheckEmail => self.check_email = true,
            Msg::SubmitBooking => {
                let Self {
                    name,
                    email,
                    mobile,
                    crew,
                    ..
                } = self;
                ConsoleService::log("Hello");
                let user = new_user::BasicUser {
                    name: name.to_owned(),
                    email: email.to_owned(),
                    mobile: mobile.to_owned(),
                    crew: crew.to_owned(),
                };
                let variables = new_user::Variables { user };

                let link = self.link.clone();
                spawn_local(async move {
                    link.send_message(Msg::SetLoading(true));
                    let client = Gql::client();
                    let response = match client.call(NewUser, variables).await {
                        Ok(r) => r,
                        Err(e) => {
                            ConsoleService::log(&format!("Network error: {:?}", e));
                            return;
                        }
                    };
                    link.send_message(Msg::SetLoading(false));
                    // link.send_message(NMsg::SetText(format!(
                    //     "{}",
                    //     response.data.unwrap().ticket_price,
                    // )));

                    if let Some(errors) = response.errors {
                        ConsoleService::log(&format!("Resp errors {:?}", errors));
                    } else if let Some(data) = response.data {
                        ConsoleService::log(&format!("Resp data {:?}", data));
                        link.send_message(Msg::CheckEmail);
                    }
                });

                return false;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        if !self.check_email {
            html! {
                <div class="form" id="new-booking-form">
                    <h1 class="title">{"Register"}</h1>
                    <form class="mui-form">
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" name="full_name" required=true value=self.name.clone() oninput=self.link.callback(|e: InputData| Msg::SetName(e.value)) />
                            <label>{"Name"}</label>
                            <p class="mui--text-caption">{"Please type your full name"}</p>
                        </div>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" name="email" required=true value=self.email.clone() oninput=self.link.callback(|e: InputData| Msg::SetEmail(e.value)) />
                            <label>{"Email"}</label>
                        </div>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" name="mobile" required=true value=self.mobile.clone() oninput=self.link.callback(|e: InputData| Msg::SetMobile(e.value)) />
                            <label>{"Mobile"}</label>
                        </div>
                        <div class="mui-textfield mui-textfield--float-label">
                            <input type="text" name="crew" required=true value=self.crew.clone() oninput=self.link.callback(|e: InputData| Msg::SetCrew(e.value)) />
                            <label>{"Crew"}</label>
                        </div>
                        <div class="button">
                            <button type="button" onclick=self.link.callback(|_| Msg::SubmitBooking)>{if self.loading {"Please wait..."} else {"Submit Booking"} }</button>
                        </div>
                    </form>
                </div>
            }
        } else {
            html! {
                <div id="new-booking-form">
                    <h1 class="title">{"Please Confirm"}</h1>
                    <h2>{format!("Hello {}, Please confirm your email", &self.name)}</h2>
                    <p>{"Thanks for registering. We’ve sent you an email from "}<b>{"bookings@normorovers.com"}</b>{" to "}{&self.email}{". Please click on the link in your email to finalise your tickets."}</p>
                </div>
            }
        }
    }
}
