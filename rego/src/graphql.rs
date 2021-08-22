use graphql_client::web::Client;
use url::Url;
use yew::services::ConsoleService;

pub struct Gql;

impl Gql {
    pub fn client() -> Client {
        let loc: String = match web_sys::window() {
            Some(window) => {
                let location = window.location();
                let mut location = Url::parse(&location.href().unwrap()).unwrap();
                match location.host_str() {
                    Some("localhost") | Some("192.168.0.20") => {
                        location.set_port(Some(8082)).unwrap();
                    }
                    _ => {}
                };
                location.set_path("/graphql");
                ConsoleService::log(&format!("Path: {}", &location));
                location.into()
            }
            None => String::from("http://localhost:8082/graphql"),
        };

        Client::new(&loc)
    }
}
