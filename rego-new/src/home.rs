use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::prelude::RouterButton;

pub struct Home {
    // link: ComponentLink<Self>,
// router: RouteAgentDispatcher<AppRoute>,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            // router: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {}
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="home">
                <div >
                    <div id="first" >
                        <p class="presents">
                            {"Normo Rovers present our annual Magical Mystery Tour"}
                        </p>
                        <h1 class="title">
                            <div class="inline" id="m">{"M"}</div>
                            <div class="inline">
                                <div class="block" id="mt">{"MT"}</div>
                                <div class="block" id="drag-race">{"Drag Race"}</div>
                            </div>
                        </h1>
                        <p class="date">
                            {"11th - 12th of September 2021"}
                        </p>
                        <img id="dancer" src="https://i.imgur.com/8lqOKTF.png"/>
                        <RouterButton<AppRoute> route=AppRoute::Register>{"Register"}</RouterButton<AppRoute>>
                    </div>
                </div>
                <div id="second">
                    <div id="elements">
                        <div class="element-single bg-dark">
                            <h2>{"Meeting Point"}</h2>
                            <p>
                                {"Meet your team at Thornleigh Maccas overflow carpark at 8am on Saturday morning."}
                            </p>
                        </div>
                        <div class="element-single bg-dark">
                            <h2>{"Clues, music, trivia"}</h2>
                            <p>
                                {"We send you off with cryptic clues, a special MMT CD full of sweet tunes and banter, and some road trip trivia."}
                            </p>
                        </div>
                        <div class="element-single bg-dark">
                            <h2>{"Three activity bases"}</h2>
                            <p>
                                {"The clues will lead you on a drive to three activity bases. At each base you will have to complete team challenges."}
                            </p>
                        </div>
                        <div class="element-single bg-dark">
                            <h2>{"The end point"}</h2>
                            <p>
                                {"You will arrive at a mystery location approximately three hours out of Sydney where good times await you."}
                            </p>
                        </div>
                        <div class="element-single bg-dark">
                            <h2>{"Facebook Event"}</h2>
                            <p>
                                {"Look at the "}
                                <a href="https://www.facebook.com/events/807816656354607">{"Facebook Event"}</a>
                                {" for the most up to date information. We will be posting there regularly so make sure you click going!"}
                            </p>
                        </div>
                        <div class="element-single bg-dark">
                            <h2>{"Admin / Website issues"}</h2>
                            <p>
                                {"If you have any issues please contact Grant on "}
                                <a href="https://www.facebook.com/messages/t/100009414012698">{"Facebook Messenger"}</a>
                                {" or email the crew "}
                                <a href={"mailto:2ndnormorovers@gmail.com"}>{"2ndnormorovers@gmail.com"}</a>
                            </p>
                        </div>
                    </div>
                </div>
                <div id="third">
                    <div class="inner">
                        <p class="text">
                            {"Don't miss out on a great event."}
                        </p>
                        <div class="button">
                            <RouterButton<AppRoute> route=AppRoute::Register>{"Register Now"}</RouterButton<AppRoute>>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
