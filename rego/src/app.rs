use serde::{Deserialize, Serialize};
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::checkin::CheckIn;
use crate::checkout::Checkout;
use crate::confirm_email::ConfirmEmail;
use crate::home::Home;
use crate::manage_vehicle::ManageVehicle;
use crate::new_booking_form::NewBookingForm;
use crate::purchase_tickets::PurchaseTickets;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="main" >
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html!{<Home />},
                            AppRoute::ConfirmEmail => html!{<ConfirmEmail />},
                            AppRoute::PurchaseTickets => html!{<PurchaseTickets />},
                            AppRoute::Register => html!{<NewBookingForm />},
                            AppRoute::Checkout => html!{<Checkout />},
                            AppRoute::CheckIn{user_id} => html!{<CheckIn user_id = user_id />},
                            AppRoute::ManageVehicle{vehicle_id} => html!{<ManageVehicle vehicle_id = vehicle_id />},
                        }
                    })
                />
            </div>
        }
    }
}
// <div>
//     <NewBookingForm />
// </div>

#[derive(Clone, Switch, Serialize, Deserialize, Debug, PartialEq)]
pub enum AppRoute {
    #[to = "/register"]
    Register,
    #[to = "/confirm_email"]
    ConfirmEmail,
    #[to = "/purchase"]
    PurchaseTickets,
    #[to = "/checkout"]
    Checkout,
    #[to = "/checkin/{user_id}"]
    CheckIn { user_id: String },
    #[to = "/manage_vehicle/{vehicle_id}"]
    ManageVehicle { vehicle_id: String },
    #[to = "/"]
    Home,
}

impl Default for AppRoute {
    fn default() -> Self {
        Self::Home
    }
}
