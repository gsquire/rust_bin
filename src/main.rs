// Why do I need decl_macro when using 'format' in the route macro?
#![feature(proc_macro_hygiene, decl_macro)]

use std::cmp;
use std::io::Cursor;
use std::net::SocketAddr;

use rand::distributions::Uniform;
use rand::Rng;
use rocket::http::{ContentType, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::response::content::Content;
use rocket::response::Stream;
use rocket::{get, routes, Outcome};
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::json::Json;
use serde::Serialize;

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

fn main() {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![ip, stream_bytes, user_agent])
        .launch();
}
