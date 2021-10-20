mod common;
use common::*;

// use rocket::http::{ContentType, Status};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};

use sea_orm_rocket_starter::domain::bakeries::*;

#[rocket::async_test]
async fn main() {
    all().await;
    get().await;
    update().await;
    delete().await;
}

async fn all() {
    let test_context = TestContext::init("bakeries_all").await;

    create_bakery(&test_context.client).await;
    create_bakery(&test_context.client).await;
    let response = test_context.client
        .get("/bakeries")
        .header(ContentType::JSON)
        .dispatch().await.into_json::<Vec<bakery::Model>>().await;

    let bakery_vec = response.expect("no bakeries returned!");
    assert_eq!(bakery_vec.len(), 2);

    TestContext::tear_down(&test_context).await;

}

async fn get() {
    let test_context = TestContext::init("bakeries_get").await;

    let r = create_bakery(&test_context.client).await;

    let response = test_context.client
        .get(format!("/bakeries/{}", r.id))
        .header(ContentType::JSON)
        .dispatch()
        .await;

    let r_bakery = response
        .into_json::<bakery::Model>()
        .await
        .unwrap();

    assert_eq!(r_bakery.name, "Test Bakery");

    TestContext::tear_down(&test_context).await;
}

async fn update() {
    let test_context = TestContext::init("bakeries_update").await;

    let r = create_bakery(&test_context.client).await;

    let response = test_context.client
        .put(format!("/bakeries/{}", r.id))
        .header(ContentType::JSON)
        .body(
            r##"{
            "name": "Updated Bakery",
            "profit_margin": 20.1
        }"##
        )
        .dispatch()
        .await;

    let r_bakery = response
        .into_json::<bakery::Model>()
        .await
        .unwrap();

    assert_eq!(r_bakery.name, "Updated Bakery");
    assert_eq!(r_bakery.profit_margin, 20.1);

    TestContext::tear_down(&test_context).await;
}

async fn delete() {
    let test_context = TestContext::init("bakery_delete").await;

    let r = create_bakery(&test_context.client).await;

    let response = test_context.client
        .get("/bakeries")
        .header(ContentType::JSON)
        .dispatch().await.into_json::<Vec<bakery::Model>>().await;

    let bakery_vec = response.expect("no bakeries returned!");
    assert_eq!(bakery_vec.len(), 1);

    let response = test_context.client
        .delete(format!("/bakeries/{}", r.id))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    let response = test_context.client
        .get("/bakeries")
        .header(ContentType::JSON)
        .dispatch().await.into_json::<Vec<bakery::Model>>().await;

    let bakery_vec = response.expect("no bakeries returned!");
    assert_eq!(bakery_vec.len(), 0);

    TestContext::tear_down(&test_context).await;
}

async fn create_bakery(client: &Client) -> bakery::Model {
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
    let r = response.into_json::<bakery::Model>().await.expect("bakery");

    r
}
