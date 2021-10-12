#[macro_use] extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};
use rocket_db_pools::{Database};

mod domain;
mod db;
use db::{pool, migrations};

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &pool::Db::fetch(&rocket).unwrap().conn;
    let _ = migrations::create_tables(conn).await;
    Ok(rocket)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/cakes", domain::cakes::handler::routes() )
        .mount("/bakeries", domain::bakeries::handler::routes() )
        .mount("/bakers", domain::bakers::handler::routes() )
        .mount("/customers", domain::customers::handler::routes() )
        .mount("/lineitems", domain::lineitems::handler::routes() )
        .mount("/orders", domain::orders::handler::routes() )
        .register("/", catchers![not_found])
}
