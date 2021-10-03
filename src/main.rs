#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::fs::{relative};
use rocket::response::{Redirect};
use rocket::{Build, Request, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use sea_orm::{entity::*, query::*};

mod pool;
use pool::RocketDbPool;
pub mod bakery_chain;

mod setup;

#[derive(Database, Debug)]
#[database("rocket_starter")]
struct Db(RocketDbPool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// mod post;
// pub use post::Entity as Post;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = setup::create_tables(conn).await;
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
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            routes![],
        )
        .register("/", catchers![not_found])
}
