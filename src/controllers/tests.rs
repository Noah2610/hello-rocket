use rocket::http::{ Status, ContentType };
use rocket::local::Client;
use crate::rocket;

fn client() -> Client {
    let rocket = rocket();
    Client::new(rocket).expect("valid rocket client")
}

#[test]
fn test_hello_page() {
    let client = client();
    let mut response = client.get("/hello").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Hello Rocket!".into()));
}
