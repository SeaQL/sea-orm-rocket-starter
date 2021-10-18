mod common;
use common::*;

// use rocket::http::{ContentType, Status};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};

#[rocket::async_test]
async fn all() {
    let test_context = TestContext::init().await;
    create_bakery(&test_context.client).await;
    create_bakery(&test_context.client).await;
    let response = &test_context.client
        .get("/bakeries")
        .header(ContentType::JSON)
        .dispatch().await;

    TestContext::tear_down(&test_context).await;
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

    response
}
