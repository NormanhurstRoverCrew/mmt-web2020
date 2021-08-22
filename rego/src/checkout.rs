use crate::booking_agent::{self, Booking, BookingAgent};
use crate::booking_summary::BookingSummary;
use crate::graphql::Gql;
use crate::ObjectId;
use graphql_client::GraphQLQuery;
use js_sys::Reflect;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::Event;
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/create_stripe_payment_intent.graphql",
    response_derives = "Debug"
)]
struct PaymentIntent;

pub struct Checkout {
    link: ComponentLink<Self>,
    #[allow(unused)]
    booking_agent: Box<dyn Bridge<BookingAgent>>,
    // router: RouteAgentDispatcher<()>,
    method: Method,
    booking: Booking,
    stripe: Option<Arc<Stripe>>,
    stripe_card: Option<Arc<Element>>,
    stripe_payment_intent: Option<String>,
    stripe_disable_button: bool,
    stripe_loading: bool,
    stripe_error: Option<String>,

    payment_complete: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Stripe,
    EFT,
    Cash,
    None,
}

pub enum Msg {
    BookingUpdate(Booking),
    SetMethod(Method),
    InitStripe,
    StripeRequestPaymentIntent,
    StripePaymentIntent(String),
    StripeDisableButton(bool),
    StripeLoading(bool),
    StripeError(Option<String>),
    StripePayCard,
    CompletePayment,
}

#[wasm_bindgen]
extern "C" {
    fn stripe_pay_card(
        stripe: &Stripe,
        card: &Element,
        client_secret: &str,
        callback: &Closure<dyn FnMut(JsValue)>,
    );

    type Stripe;

    #[wasm_bindgen(js_name = "Stripe")]
    fn stripe(pub_key: &str) -> Stripe;

    #[wasm_bindgen(method, js_name = "redirectToCheckout")]
    fn redirect_to_checkout(this: &Stripe, optional: JsValue) -> JsValue;

    type Elements;
    type Element;

    #[wasm_bindgen(method)]
    fn elements(this: &Stripe) -> Elements;

    #[wasm_bindgen(method)]
    fn create(this: &Elements, method: &str, opts: JsValue) -> Element;

    #[wasm_bindgen(method)]
    fn mount(this: &Element, id: &str);

    #[wasm_bindgen(method)]
    fn on(this: &Element, event: &str, closure: &Closure<dyn FnMut(JsValue)>);

    #[wasm_bindgen(method, js_name = "confirmCardPayment")]
    async fn confirm_card_payment(this: &Stripe, client_secret: &str, data: &JsValue) -> JsValue;
}

#[derive(Debug, Clone, Deserialize)]
struct StripeCardEvent {
    empty: bool,
    complete: bool,
    error: Option<StripeCardEventError>,
}

#[derive(Debug, Clone, Deserialize)]
struct StripeCardEventError {
    code: String,
    r#type: String,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct StripePaymentResult {
    error: Option<String>,
    #[serde(rename = "paymentIntent")]
    payment_intent: StripePaymentIntent,
}

#[derive(Debug, Clone, Deserialize)]
struct StripePaymentIntent {
    id: String,
}

impl Component for Checkout {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let booking_agent = BookingAgent::bridge(link.callback(|a| match a {
            booking_agent::Response::BookingUpdate(booking) => Msg::BookingUpdate(booking),
        }));

        //     let router = RouteAgentDispatcher::new();

        Self {
            link,
            //         router,
            booking_agent,
            booking: Booking::default(),
            method: Method::None,
            stripe: None,
            stripe_card: None,
            stripe_payment_intent: None,
            stripe_disable_button: true,
            stripe_loading: true,
            stripe_error: None,

            payment_complete: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BookingUpdate(booking) => {
                self.booking = booking;
                return false;
            }
            Msg::SetMethod(method) => {
                match &method {
                    Method::Stripe if self.method != method => {
                        self.link.send_message_batch(vec![
                            Msg::InitStripe,
                            Msg::StripeRequestPaymentIntent,
                        ]);
                    }
                    _ => {}
                };

                self.method = method;
                return true;
            }
            Msg::InitStripe => {
                self.init_stripe();
                return true;
            }
            Msg::StripeRequestPaymentIntent => {
                self.stripe_get_payment_intent();
            }
            Msg::StripePaymentIntent(pi) => {
                self.stripe_payment_intent = Some(pi);
                return true;
            }
            Msg::StripeDisableButton(disable) => {
                self.stripe_disable_button = disable;
                return true;
            }
            Msg::StripeLoading(loading) => {
                self.stripe_loading = loading;
                return true;
            }
            Msg::StripeError(error) => {
                self.stripe_error = error;
                return true;
            }
            Msg::StripePayCard => {
                self.pay_stripe();
            }
            Msg::CompletePayment => {
                self.payment_complete = true;
                return true;
            }
        };
        return false;
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let instructions = match self.method {
            Method::Stripe => {
                let spinner_class = format!(
                    "spinner {}",
                    if self.stripe_loading() { "" } else { "hidden" }
                );
                let form = html! {
                    <form id="payment-form">
                      <div id="card-element"></div>
                      <button id="submit" disabled={self.stripe_disable_button}>
                        <div class={spinner_class} id="spinner"></div>
                        <span id="button-text">{"Pay now"}</span>
                      </button>
                      <p id="card-error" role="alert">{&self.stripe_error.as_ref().map(|e| format!("Error: {}", e)).unwrap_or("".to_string())}</p>
                      <p class="result-message hidden"></p>
                    </form>
                };

                html! {
                    <div>
                        <h2>{"Stripe"}</h2>
                        {form}
                    </div>
                }
            }
            Method::EFT => html! {
                <div id="eft">
                    <h2>{"Electronic Funds Transfer (EFT)"}</h2>
                    <div>
                        <p>{"Please make sure you use use the exact description we provide here, this is to make sure your payment goes through and ensures your place at MMT."}</p>
                        <div class="eft-details">
                            <div class="name-title">{"Name"}</div>
                            <div class="name">{"Normanhurst Rover Crew"}</div>
                            <div class="bsb-title">{"BSB"}</div>
                            <div class="bsb">{"032186"}</div>
                            <div class="acc-title">{"Acc"}</div>
                            <div class="acc">{"811413"}</div>
                            <div class="desc-title">{"Description"}</div>
                            <div class="desc">{format!("MMT21-{}", self.booking.index)}</div>
                        </div>
                        <button type="button" class="mui-btn mui-btn--raised" onclick=self.link.callback(|_| Msg::CompletePayment)>{"I have paid via EFT!"}</button>
                    </div>
                </div>
            },
            Method::Cash => html! {
                <h2>{"Cash"}</h2>
            },
            _ => html! {},
        };

        let content = if !self.payment_complete {
            html! {
                <>
                        <p>{"Please select a payment option"}</p>
                        <div class="payment-type-field">
                            <div>
                                <input type="radio" checked={self.method == Method::Stripe} name="checkout_method" id="stripe" onchange=self.link.callback(|_| Msg::SetMethod(Method::Stripe)) />
                                <label for="stripe">{"Debit/Credit Card via Stripe (Fee to cover the card payment added on)"}</label>
                            </div>
                            <div>
                                <input type="radio" checked={self.method == Method::EFT} name="checkout_method" id="eft" onchange=self.link.callback(|_| Msg::SetMethod(Method::EFT)) />
                                <label for="eft">{"Electronic Funds Transfer (EFT)"}</label>
                            </div>
                            <div>
                                <input type="radio" checked={self.method == Method::Cash} name="checkout_method" id="cash" disabled=true onchange=self.link.callback(|_| Msg::SetMethod(Method::Cash)) />
                                <label for="cash">{"Cash on the day (unavailable due to COVID)"}</label>
                            </div>
                        </div>
                        {instructions}
                </>
            }
        } else {
            html! {
                <>
                    <h1>{"Thankyou!"}</h1>
                    <p>{"We look forward to seeing you at MMT 2021. Please keep an eye on you emails and the Facebook event (click going if you haven't already)."}</p>
                    <p>{"If you paid via EFT not much will happen until we process your payment. You will receive an email receipt once this occurs. Please contact us if this doesn't happen within 3-4 business days."}</p>
                    <p>{"If you've bought multiple tickets, for other participants, they will be receiving an email - once we process your payment - with instructions on how to continue. Please check with them to make sure they receive it."}</p>
                    <p>
                        {"If there are any issues with this process, please reach out to us using the contact details on the "}
                        <a href="/">{"Home Page"}</a>
                        {" or on Facebook."}
                    </p>
                    <p>{"To ensure COVID safety, teams must be created online, before the event. If you have any cold or flu symptoms, please isolate at home, and follow the advice given by NSW Health."}</p>
                    <p>{"You will receive an email with instructions on how to create a team, or join one."}</p>
                </>
            }
        };

        html! {
            <div id="checkout">
                <h1>{"Payment"}</h1>
                <div class="layout">
                    <div class="content">
                        {content}
                    </div>
                    <BookingSummary details=true stripe={self.method == Method::Stripe}/>
                </div>
            </div>
        }
    }
}

impl Checkout {
    fn stripe_loading(&self) -> bool {
        self.stripe_loading
            || self.stripe.is_none()
            || self.stripe_card.is_none()
            || self.stripe_payment_intent.is_none()
    }

    fn init_stripe(&mut self) {
        self.link.send_message(Msg::StripeLoading(true));
        self.stripe = Some(Arc::new(stripe(
            std::option_env!("STRIPE_CLIENT_KEY")
                .unwrap_or("pk_test_3jpeKc8apKbPVGnnNoyY51GK00N6X69Wap"),
        )));

        if let Some(stripe) = &self.stripe {
            let elements = stripe.elements();
            let card_element = elements.create(
                "card",
                JsValue::from_serde(&json!({
                    "style": {
                        "base": {
                          "color": "#32325d",
                          "fontFamily": "Arial, sans-serif",
                          "fontSmoothing": "antialiased",
                          "fontSize": "16px",
                          "::placeholder": {
                            "color": "#32325d"
                          }
                        },
                        "invalid": {
                          "fontFamily": "Arial, sans-serif",
                          "color": "#fa755a",
                          "iconColor": "#fa755a"
                        }
                    }
                }))
                .unwrap(),
            );
            card_element.mount("#card-element");

            let link = self.link.clone();
            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                let link = link.clone();
                let event: StripeCardEvent = event.into_serde().unwrap();
                link.send_message_batch(vec![
                    Msg::StripeDisableButton(!event.complete),
                    Msg::StripeError(event.error.map(|e| e.message)),
                ]);
            }) as Box<dyn FnMut(JsValue)>);

            card_element.on("change", &closure);
            closure.forget();

            self.stripe_card = Some(Arc::new(card_element));

            let document = web_sys::window().unwrap().document().unwrap();
            let form = document.get_element_by_id("payment-form").unwrap();

            let link = self.link.clone();
            let closure = Closure::wrap(Box::new(move |event: Event| {
                let link = link.clone();
                event.prevent_default();

                link.send_message_batch(vec![Msg::StripePayCard]);
            }) as Box<dyn FnMut(Event)>);
            if let Err(e) =
                form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())
            {
                ConsoleService::log(&format!("Could not add event listener to submit: {:?}", e));
                return;
            }
            closure.forget();
            self.link.send_message(Msg::StripeLoading(true));
        }
    }

    fn pay_stripe(&mut self) {
        if let Self {
            stripe: Some(stripe),
            stripe_card: Some(card),
            stripe_payment_intent: Some(client_secret),
            ..
        } = &self
        {
            self.link.send_message(Msg::StripeLoading(true));
            let opts = Arc::new(JsValue::from_serde(&json!({"payment_method": {}})).unwrap());
            if let Err(e) = Reflect::set(
                &Reflect::get(&opts, &JsValue::from_str("payment_method")).unwrap(),
                &JsValue::from_str("card"),
                &*card,
            ) {
                ConsoleService::log(&format!("unable to reflect structure. {:?}", e));
                return;
            }

            let client_secret = client_secret.clone();
            let stripe = stripe.clone();
            let link = self.link.clone();
            spawn_local(async move {
                let result = stripe
                    // .confirm_card_payment(&client_secret, JsValue::from(card))
                    .confirm_card_payment(&client_secret, &*opts)
                    .await;
                ConsoleService::log(&format!("res: {:?}", &result));
                link.send_message_batch(vec![Msg::StripeLoading(false), Msg::CompletePayment]);
            });
        } else {
            ConsoleService::log("Missing information to complete Stripe Card payment");
            // Log an error or try to remedy this situation
            self.link.send_message(Msg::StripeLoading(false));
        }
    }

    fn stripe_get_payment_intent(&mut self) {
        let variables = payment_intent::Variables {
            booking_id: self.booking.id.clone(),
        };

        let link = self.link.clone();
        spawn_local(async move {
            link.send_message(Msg::StripeLoading(true));
            let client = Gql::client();
            let response = match client.call(PaymentIntent, variables).await {
                Ok(r) => r,
                Err(e) => {
                    ConsoleService::log(&format!("Network error: {:?}", e));
                    return;
                }
            };

            if let Some(errors) = response.errors {
                link.send_message_batch(vec![
                    Msg::StripeError(Some(errors.first().unwrap().message.to_owned())),
                    Msg::StripeLoading(false),
                    Msg::StripeDisableButton(true),
                ]);
            } else if let Some(data) = response.data {
                link.send_message_batch(vec![
                    Msg::StripePaymentIntent(data.create_stripe_payment_intent_for_booking),
                    Msg::StripeLoading(false),
                ]);
            }
        });
    }
}
