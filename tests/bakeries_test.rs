mod common;
use common::*;

// use rocket::http::{ContentType, Status};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};

#[rocket::async_test]
async fn all() {
    let client = test_client().await;
    create_bakery(&client).await;
    create_bakery(&client).await;
    let response = client
        .get("/bakeries")
        .header(ContentType::JSON)
        .dispatch().await;
    println!("response: {:#?}", response);
}

async fn create_bakery(client: &Client) -> LocalResponse<'_> {
    let response = client
        .post("/bakeries")
        .header(ContentType::JSON)
        .body(
            r##"{
            "name": "Test Bakery",
            "profit_margin": 10.4
        }"##
        )
        .dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    // TODO: tear down the test_client database

    response
}
