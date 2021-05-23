use crate::booking_summary::BookingSummary;
use js_sys::Reflect;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::Event;
use yew::prelude::*;
use yew::services::ConsoleService;

use wasm_bindgen::prelude::*;

pub struct Checkout {
    link: ComponentLink<Self>,
    // booking_agent: Box<dyn Bridge<BookingAgent>>,
    // router: RouteAgentDispatcher<()>,
    method: Method,
    // booking: Booking,
    stripe: Option<Stripe>,
    stripe_card: Option<Element>,
    stripe_client_secret: Option<String>,
    stripe_disable_button: bool,
    stripe_error: Option<String>,
}

#[derive(Debug)]
pub enum Method {
    Stripe,
    EFT,
    Cash,
    None,
}

pub enum Msg {
    // BookingUpdate(Booking),
    SetMethod(Method),
    InitStripe,
    RedirectToCheckout(String),
    RequestSessionId,
    StripeDisableButton(bool),
    StripeError(Option<String>),
    StripePayCard,
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
    fn confirm_card_payment(this: &Stripe, client_secret: &str, data: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Deserialize)]
struct StripeCardEvent {
    empty: bool,
    error: Option<String>,
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
        //     let booking_agent = BookingAgent::bridge(link.callback(|a| match a {
        //         booking_agent::Response::BookingUpdate(booking) => Msg::BookingUpdate(booking),
        //     }));

        //     let router = RouteAgentDispatcher::new();

        Self {
            link,
            //         router,
            //         booking_agent,
            // booking: Booking::default(),
            method: Method::None,
            stripe: None,
            stripe_card: None,
            stripe_client_secret: None,
            stripe_disable_button: true,
            stripe_error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::BookingUpdate(booking) => {
            //     self.booking = booking;
            //     return true;
            // }
            Msg::SetMethod(method) => {
                match &method {
                    Method::Stripe => {
                        self.link
                            .send_message_batch(vec![Msg::InitStripe, Msg::RequestSessionId]);
                    }
                    _ => {}
                };

                self.method = method;
                return true;
            }
            Msg::InitStripe => {
                self.stripe = Some(stripe("pk_test_3jpeKc8apKbPVGnnNoyY51GK00N6X69Wap"));

                if let Some(stripe) = &self.stripe {
                    let elements = stripe.elements();
                    let card_element =
                        elements.create("card", JsValue::from_serde(&json!({})).unwrap());
                    card_element.mount("#card-element");

                    let link = self.link.clone();
                    let closure = Closure::wrap(Box::new(move |event: JsValue| {
                        let link = link.clone();
                        let event: StripeCardEvent = event.into_serde().unwrap();
                        link.send_message_batch(vec![
                            Msg::StripeDisableButton(event.empty),
                            Msg::StripeError(event.error),
                        ]);
                    }) as Box<dyn FnMut(JsValue)>);

                    card_element.on("change", &closure);
                    closure.forget();

                    self.stripe_card = Some(card_element);

                    let document = web_sys::window().unwrap().document().unwrap();
                    let form = document.get_element_by_id("payment-form").unwrap();

                    let link = self.link.clone();
                    let closure = Closure::wrap(Box::new(move |event: Event| {
                        let link = link.clone();
                        event.prevent_default();

                        link.send_message_batch(vec![Msg::StripePayCard]);
                    }) as Box<dyn FnMut(Event)>);
                    form.add_event_listener_with_callback(
                        "submit",
                        closure.as_ref().unchecked_ref(),
                    );
                    closure.forget();
                }
            }
            Msg::RedirectToCheckout(session_id) => {
                if let Some(stripe) = &self.stripe {
                    stripe.redirect_to_checkout(
                        JsValue::from_serde(&json!({ "sessionId": session_id })).unwrap(),
                    );
                }
            }
            Msg::RequestSessionId => {
                // self.stripe_client_secret = Some("TEST".to_string());
                return false;
            }
            Msg::StripeDisableButton(disable) => {
                self.stripe_disable_button = disable;
                return true;
            }
            Msg::StripeError(error) => {
                self.stripe_error = error;
                return true;
            }
            Msg::StripePayCard => {
                if let Self {
                    stripe: Some(stripe),
                    stripe_card: Some(card),
                    stripe_client_secret: Some(client_secret),
                    ..
                } = &self
                {
                    let opts = JsValue::from_serde(&json!({"payment_method": {}})).unwrap();
                    Reflect::set(
                        &Reflect::get(&opts, &JsValue::from_str("payment_method")).unwrap(),
                        &JsValue::from_str("card"),
                        card,
                    );

                    stripe.confirm_card_payment(&client_secret, opts);
                } else {
                    ConsoleService::log("Missing information to complete Stripe Card payment");
                    // Log an error or try to remedy this situation
                }
            }
        };
        return false;
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let instructions = match self.method {
            Method::Stripe => html! {
                <div>
                    {"Stripe"}
                    // <button id="checkout-button">{"Checkout"}</button>
                    <form id="payment-form">
                      <div id="card-element"></div>
                      <button id="submit" disabled={self.stripe_disable_button}>
                        <div class="spinner hidden" id="spinner"></div>
                        <span id="button-text">{"Pay now"}</span>
                      </button>
                      <p id="card-error" role="alert">{&self.stripe_error.as_ref().unwrap_or(&"".to_string())}</p>
                      <p class="result-message hidden"></p>
                    </form>
                </div>
            },
            Method::EFT => html! {
                <div>{"Electronic Funds Transfer (EFT)"}</div>
            },
            Method::Cash => html! {
                <div>{"Cash"}</div>
            },
            _ => html! {},
        };

        html! {
            <div id="checkout">
                <div>
                    <p>{"Select a payment option"}</p>
                    <div class="payment-type-field">
                        <div>
                            <input type="radio" name="checkout_method" id="stripe" onchange=self.link.callback(|_| Msg::SetMethod(Method::Stripe)) />
                            <label for="stripe">{"Debit/Credit Card via Stripe"}</label>
                        </div>
                        <div>
                            <input type="radio" name="checkout_method" id="eft" onchange=self.link.callback(|_| Msg::SetMethod(Method::EFT)) />
                            <label for="eft">{"Electronic Funds Transfer (EFT)"}</label>
                        </div>
                        <div>
                            <input type="radio" name="checkout_method" id="cash" onchange=self.link.callback(|_| Msg::SetMethod(Method::Cash)) />
                            <label for="cash">{"Cash (unadvisable)"}</label>
                        </div>
                    </div>
                    {instructions}
                </div>
                <BookingSummary details=true />
            </div>
        }
    }
}
