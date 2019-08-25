#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::{Rocket, State};
use rocket_contrib::json::JsonValue;
use std::env;

struct MServer {
    server: String,
}

#[get("/.well-known/matrix/server")]
fn well_known(m_server: State<MServer>) -> JsonValue {
    json!({ "m.server": m_server.server })
}

fn rocket(server: String) -> Rocket {
    rocket::ignite()
        .manage(MServer { server })
        .mount("/", routes![well_known])
}

fn m_server_from_env() -> String {
    let server_name_env = env::var("SYNAPSE_SERVER_NAME");
    let port_env = env::var("SYNAPSE_SERVER_PORT");

    match server_name_env {
        Ok(server_name) => {
            let port = match port_env {
                Ok(val) => Some(val),
                Err(_) => None,
            };

            match port {
                Some(p) => format!("{}:{}", server_name, p),
                None => server_name,
            }
        }
        Err(e) => panic!(
            "SYNAPSE_SERVER_NAME environment variable is not set or invalid: {}!",
            e
        ),
    }
}

fn main() {
    let m_server = m_server_from_env();

    rocket(m_server).launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use crate::m_server_from_env;
    use rocket::http::Status;
    use rocket::local::Client;
    use std::env;

    #[test]
    fn well_known() {
        let exp_server = "server.example.com:5050";

        let client = Client::new(rocket(String::from(exp_server))).expect("valid rocket instance");
        let mut response = client.get("/.well-known/matrix/server").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some(format!("{{\"m.server\":\"{}\"}}", exp_server))
        );
    }

    fn remove_vars() {
        env::remove_var("SYNAPSE_SERVER_NAME");
        env::remove_var("SYNAPSE_SERVER_PORT");
    }

    #[test]
    fn without_port() {
        let exp_m_server = "server.example.com";

        remove_vars();
        env::set_var("SYNAPSE_SERVER_NAME", "server.example.com");

        let m_server = m_server_from_env();
        assert_eq!(m_server, exp_m_server)
    }

    #[test]
    fn with_port() {
        let exp_m_server = "server.example.com:5050";

        remove_vars();
        env::set_var("SYNAPSE_SERVER_NAME", "server.example.com");
        env::set_var("SYNAPSE_SERVER_PORT", "5050");

        let m_server = m_server_from_env();
        assert_eq!(m_server, exp_m_server)
    }

    #[test]
    #[should_panic(expected = "SYNAPSE_SERVER_NAME environment variable is not set or invalid:")]
    fn server_not_set() {
        remove_vars();
        m_server_from_env();
    }
}
