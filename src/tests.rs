use rocket::http::Status;
use rocket::{self, local::Client, routes};

#[test]
fn ip() {
    let rocket = rocket::ignite().mount("/", routes![super::ip]);
    let client = Client::new(rocket).unwrap();
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
