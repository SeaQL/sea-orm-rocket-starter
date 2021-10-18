mod common;
use common::*;

// use rocket::http::{ContentType, Status};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};

use sea_orm_rocket_starter::domain::bakeries::*;

// #[rocket::async_test]
// async fn all() {

//     rocket::tokio::task::spawn(async move {
//         let test_context = TestContext::init().await;
//         // println!("@@ all() test_context: {:#?}", test_context.client);

//         create_bakery(&test_context.client).await;
//         create_bakery(&test_context.client).await;
//         let response = test_context.client
//             .get("/bakeries")
//             .header(ContentType::JSON)
//             .dispatch().await.into_json::<Vec<bakery::Model>>().await;

//         let bakery_vec = response.expect("no bakeries returned!");
//         assert_eq!(bakery_vec.len(), 2);

//         TestContext::tear_down(&test_context).await;
//     }).await.expect("Task panicked");

// }

#[rocket::async_test]
async fn get() {
    rocket::tokio::task::spawn(async move {
        let test_context = TestContext::init().await;
            // println!("@@ get() test_context: {:#?}", test_context.client);

        let r = create_bakery(&test_context.client).await;
        println!("r: {:#?}", r);

        let response = test_context.client
            .get(format!("/bakeries/{}", r.id))
            .header(ContentType::JSON)
            .dispatch()
            .await;
            println!("response: {:#?}", response);

        // let r_bakery = response
        //     .into_json::<bakery::Model>()
        //     .await
        //     .unwrap();
    println!("response: {:#?}", response);

        // assert_eq!(r_bakery.name, "Test Bakery");

        TestContext::tear_down(&test_context).await;
    }).await.expect("Task panicked");

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
    println!("create_bakery response: {:#?}", r);

    r

}
