mod common;
use common::*;

// use rocket::http::{ContentType, Status};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;

use sea_orm_rocket_starter::domain::bakeries::*;
use sea_orm_rocket_starter::domain::bakers::*;

#[rocket::async_test]
async fn main() {
    all().await;
    get().await;
    update().await;
    delete().await;
}

async fn all() {
    let test_context = TestContext::init("bakers_all").await;

    create_baker(&test_context.client).await;
    create_baker(&test_context.client).await;

    let response = test_context
        .client
        .get("/bakers")
        .header(ContentType::JSON)
        .dispatch()
        .await
        .into_json::<Vec<baker::Model>>()
        .await;

    let baker_vec = response.expect("no bakers returned!");
    assert_eq!(baker_vec.len(), 2);

    TestContext::tear_down(&test_context).await;
}

async fn get() {
    let test_context = TestContext::init("bakers_get").await;

    let r = create_baker(&test_context.client).await;

    let response = test_context
        .client
        .get(format!("/bakers/{}", r.id))
        .header(ContentType::JSON)
        .dispatch()
        .await;

    let r_baker = response.into_json::<baker::Model>().await.unwrap();

    assert_eq!(r_baker.name, "Baker Bob");

    TestContext::tear_down(&test_context).await;
}

async fn update() {
    let test_context = TestContext::init("bakers_update").await;

    let r = create_baker(&test_context.client).await;

    let response = test_context
        .client
        .put(format!("/bakers/{}", r.id))
        .header(ContentType::JSON)
        .body(
            r##"{
            "name": "Baker Tom",
            "contact_details": {
                "mobile": "+61424555555",
                "home": "039999999",
                "address": "Updated address"
            }
        }"##,
        )
        .dispatch()
        .await;

    let r_baker = response.into_json::<baker::Model>().await.unwrap();

    assert_eq!(r_baker.name, "Baker Tom");
    assert_eq!(r_baker.contact_details["mobile"], json!("+61424555555"));

    TestContext::tear_down(&test_context).await;
}

async fn delete() {
    let test_context = TestContext::init("bakers_delete").await;

    let r = create_baker(&test_context.client).await;

    let response = test_context
        .client
        .get("/bakers")
        .header(ContentType::JSON)
        .dispatch()
        .await
        .into_json::<Vec<baker::Model>>()
        .await;

    let baker_vec = response.expect("no bakers returned!");
    assert_eq!(baker_vec.len(), 1);

    let response = test_context
        .client
        .delete(format!("/bakers/{}", r.id))
        .header(ContentType::JSON)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let response = test_context
        .client
        .get("/bakers")
        .header(ContentType::JSON)
        .dispatch()
        .await
        .into_json::<Vec<baker::Model>>()
        .await;

    let baker_vec = response.expect("no bakers returned!");
    assert_eq!(baker_vec.len(), 0);

    TestContext::tear_down(&test_context).await;
}

async fn create_baker(client: &Client) -> baker::Model {
    let bakery = client
        .post("/bakeries")
        .header(ContentType::JSON)
        .body(
            r##"{
        "name": "Test Bakery",
        "profit_margin": 10.4
    }"##,
        )
        .dispatch()
        .await
        .into_json::<bakery::Model>()
        .await
        .expect("bakery");

    let response = client
        .post("/bakers")
        .header(ContentType::JSON)
        .body(format!(
            r##"{{
            "name": "Baker Bob",
            "contact_details": {{
                "mobile": "+61424000000",
                "home": "0395555555",
                "address": "12 Test St, Testville, Vic, Australia"
            }},
            "bakery_id": {}
        }}"##,
            bakery.id
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let r = response.into_json::<baker::Model>().await.expect("baker");

    r
}
