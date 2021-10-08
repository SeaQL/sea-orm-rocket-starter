use chrono::offset::Utc;
// use crate::lil_lib::bakery_chain::*;
use rocket::tokio::runtime;
use rocket_db_pools::rocket::figment::{
    providers::{Format, Toml},
    Figment,
};
use rust_decimal_macros::dec;
use sea_orm::entity::prelude::*;
use sea_orm::entity::*;
use uuid::Uuid;

// extern crate lil_lib;

fn main() {
    let url = Figment::from(Toml::file("Rocket.toml"))
        .extract_inner::<String>("default.databases.rocket_starter.url")
        .unwrap();

    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(seed_database(&url));
}

async fn seed_database(url: &str) {
    let db = sea_orm::Database::connect(url).await.unwrap();
    lil_lib::setup::create_tables(&db).await;
    // Bakery
    println!("Seeding Bakery...");

    let seaside_bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let bakery_insert_res = Bakery::insert(seaside_bakery)
        .exec(&db)
        .await
        .expect("could not insert bakery");

    // Baker
    println!("Seeding Bakery...");

    let baker_bob = baker::ActiveModel {
        name: Set("Baker Bob".to_owned()),
        contact_details: Set(serde_json::json!({
            "mobile": "+61424000000",
            "home": "0395555555",
            "address": "12 Test St, Testville, Vic, Australia"
        })),
        bakery_id: Set(Some(bakery_insert_res.last_insert_id as i32)),
        ..Default::default()
    };
    let baker_insert_res = Baker::insert(baker_bob)
        .exec(&db)
        .await
        .expect("could not insert baker");

    // Cake
    println!("Seeding Cake...");

    let mud_cake = cake::ActiveModel {
        name: Set("Mud Cake".to_owned()),
        price: Set(dec!(10.25)),
        gluten_free: Set(false),
        serial: Set(Uuid::new_v4()),
        bakery_id: Set(Some(bakery_insert_res.last_insert_id as i32)),
        ..Default::default()
    };

    let cake_insert_res = Cake::insert(mud_cake)
        .exec(&db)
        .await
        .expect("could not insert cake");

    // Cake_Baker
    println!("Seeding Cake_Baker...");

    let cake_baker = cakes_bakers::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id as i32),
        baker_id: Set(baker_insert_res.last_insert_id as i32),
    };
    let cake_baker_res = CakesBakers::insert(cake_baker.clone())
        .exec(&db)
        .await
        .expect("could not insert cake_baker");
    assert_eq!(
        cake_baker_res.last_insert_id,
        if cfg!(feature = "sqlx-postgres") {
            (cake_baker.cake_id.unwrap(), cake_baker.baker_id.unwrap())
        } else {
            Default::default()
        }
    );

    // Customer
    println!("Seeding Cake_Baker...");

    let customer_kate = customer::ActiveModel {
        name: Set("Kate".to_owned()),
        notes: Set(Some("Loves cheese cake".to_owned())),
        ..Default::default()
    };
    let customer_insert_res = Customer::insert(customer_kate)
        .exec(&db)
        .await
        .expect("could not insert customer");

    // Order
    println!("Seeding Order...");

    let order_1 = order::ActiveModel {
        bakery_id: Set(bakery_insert_res.last_insert_id as i32),
        customer_id: Set(customer_insert_res.last_insert_id as i32),
        total: Set(dec!(15.10)),
        placed_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let order_insert_res = lil_lib::bakery_chain::Order::insert(order_1)
        .exec(&db)
        .await
        .expect("could not insert order");

    // Lineitem
    println!("Seeding Lineitem...");

    let lineitem_1 = lineitem::ActiveModel {
        cake_id: Set(cake_insert_res.last_insert_id as i32),
        order_id: Set(order_insert_res.last_insert_id as i32),
        price: Set(dec!(7.55)),
        quantity: Set(2),
        ..Default::default()
    };
    let _lineitem_insert_res = Lineitem::insert(lineitem_1)
        .exec(&db)
        .await
        .expect("could not insert lineitem");
}
