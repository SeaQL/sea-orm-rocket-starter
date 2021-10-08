#[macro_use] extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::fs::relative;
use rocket::response::Redirect;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Request, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

mod lil_lib;
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
        .mount("/cakes",
            routes![lil_lib::cakes::handler::all,
                // lil_lib::cakes::handler::get,
                // lil_lib::cakes::handler::post,
                // lil_lib::cakes::handler::put,
                // lil_lib::cakes::handler::delete
                ],
        )
        .register("/", catchers![not_found])
}
