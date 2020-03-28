// Why do I need decl_macro when using 'format' in the route macro?
#![feature(proc_macro_hygiene, decl_macro)]

use std::net::SocketAddr;

use rocket::request::{self, FromRequest, Request};
use rocket::{get, routes, Outcome};
use rocket_contrib::json::Json;
use serde::Serialize;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
struct Ip {
    origin: SocketAddr,
}

#[get("/ip", format = "json")]
fn ip(origin: SocketAddr) -> Json<Ip> {
    Json(Ip { origin })
}

#[derive(Serialize)]
struct UserAgent {
    #[serde(rename = "user-agent")]
    agent: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAgent {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        const USER_AGENT: &str = "User-Agent";

        let agent = request.headers().get_one(USER_AGENT);
        match agent {
            Some(a) => Outcome::Success(UserAgent {
                agent: a.to_string(),
            }),
            None => Outcome::Forward(()),
        }
    }
}

#[get("/user-agent", format = "json")]
fn user_agent(agent: UserAgent) -> Json<UserAgent> {
    Json(agent)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![ip, user_agent])
        .launch();
}
