use graphql_client::web::Client;
use url::Url;
use web_sys::Location;
use yew::services::ConsoleService;

pub struct Gql;

impl Gql {
    pub fn client() -> Client {
        let loc: String = match web_sys::window() {
            Some(window) => {
                let location = window.location();
                let mut location = Url::parse(&location.href().unwrap()).unwrap();
                location.set_port(Some(8082)).unwrap();
                location.set_path("/graphql");
                ConsoleService::log(&format!("Path: {}", &location));
                location.into()
            }
            None => String::from("http://localhost:8082/graphql"),
        };

        Client::new(&loc)
    }
}
