// Why do I need decl_macro when using 'format' in the route macro?
#![feature(proc_macro_hygiene, decl_macro)]

use std::cmp;
use std::io::Cursor;
use std::net::SocketAddr;

use rand::distributions::Uniform;
use rand::Rng;
use rocket::config::{Config, Environment};
use rocket::http::{ContentType, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::response::content::Content;
use rocket::response::{NamedFile, Redirect, Stream};
use rocket::{get, routes, Outcome, Responder};
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

#[get("/stream-bytes/<n>")]
fn stream_bytes(n: usize) -> Content<Stream<Cursor<Vec<u8>>>> {
    const MAX_STREAM_SIZE: usize = 1024 * 1024;

    let n = cmp::min(n, MAX_STREAM_SIZE);
    let range = Uniform::from(0..u8::MAX);
    let values: Vec<u8> = rand::thread_rng().sample_iter(&range).take(n).collect();
    Content(ContentType::Binary, Stream::from(Cursor::new(values)))
}

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
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        const USER_AGENT: &str = "User-Agent";

        let agent = request.headers().get_one(USER_AGENT);
        match agent {
            Some(a) => Outcome::Success(UserAgent {
                agent: a.to_string(),
            }),
            None => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

#[get("/user-agent", format = "json")]
fn user_agent(agent: UserAgent) -> Json<UserAgent> {
    Json(agent)
}

#[derive(Debug, Responder)]
enum StatusOrRedirect {
    S(Status),
    R(Redirect),
}

#[get("/status/<s>")]
fn status(s: u16) -> StatusOrRedirect {
    match Status::from_code(s) {
        Some(Status::MovedPermanently) => StatusOrRedirect::R(Redirect::moved("/")),
        Some(Status::Found) => StatusOrRedirect::R(Redirect::found("/")),
        Some(Status::SeeOther) => StatusOrRedirect::R(Redirect::to("/")),
        Some(Status::TemporaryRedirect) => StatusOrRedirect::R(Redirect::temporary("/")),
        Some(Status::PermanentRedirect) => StatusOrRedirect::R(Redirect::permanent("/")),
        Some(s) => StatusOrRedirect::S(s),
        None => StatusOrRedirect::S(Status::BadRequest),
    }
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("routes.html").ok()
}

fn default_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize)]
struct EnvConfig {
    #[serde(default = "default_port")]
    port: u16,
}

fn main() {
    // We provide a default value so this is safe to do.
    let env = envy::from_env::<EnvConfig>().unwrap();
    let config = Config::build(Environment::Production)
        .port(env.port)
        .unwrap();

    rocket::custom(config)
        .attach(SpaceHelmet::default())
        .mount("/", routes![index, ip, status, stream_bytes, user_agent])
        .launch();
}
