mod common;
use common::*;

// use rocket::http::{ContentType, Status};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

#[test]
fn all() {
    let client = test_client();
    create_bakery(&client);
}

fn create_bakery(client: &Client) -> LocalResponse {
    let response = client
        .post("/bakeries")
        .header(ContentType::JSON)
        .body(
            r##"{
            "name": "Test Bakery",
            "profit_margin": 10.4
        }"##
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    response
}
