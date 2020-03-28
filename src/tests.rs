use rocket::http::{Header, Status};
use rocket::{self, local::Client, routes};

fn setup() -> Client {
    let rocket = rocket::ignite().mount(
        "/",
        routes![
            super::index,
            super::ip,
            super::status,
            super::stream_bytes,
            super::user_agent
        ],
    );
    Client::new(rocket).unwrap()
}

#[test]
fn ip() {
    let client = setup();
    let mut response = client
        .get("/ip")
        .remote("127.0.0.1:8000".parse().unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string().unwrap(),
        r#"{"origin":"127.0.0.1:8000"}"#
    );
}

#[test]
fn user_agent_header_present() {
    let client = setup();
    let header_names = vec!["User-Agent", "user-agent", "user-Agent", "User-agent"];

    for agent in header_names {
        let mut response = client
            .get("/user-agent")
            .header(Header::new(agent, "test"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string().unwrap(), r#"{"user-agent":"test"}"#);
    }
}

#[test]
fn user_agent_header_missing() {
    let client = setup();
    let response = client.get("/user-agent").dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn stream_bytes() {
    let client = setup();
    let mut response = client.get("/stream-bytes/7").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_bytes().unwrap().len(), 7);
}

#[test]
fn status() {
    let client = setup();
    let statuses = vec![(204, Status::NoContent), (301, Status::MovedPermanently)];
    for s in statuses {
        let response = client.get(format!("/status/{}", s.0)).dispatch();
        assert_eq!(response.status(), s.1);
    }
}
