#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

mod db;
pub mod domain;
use db::{migrations, pool};

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

pub fn rocket() -> Rocket<Build> {
    use figment::{
        providers::{Env, Format, Toml},
        Figment,
    };

    let figment = Figment::new()
        .merge(rocket::Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("ROCKET_APP_").split("_"));

    rocket::custom(figment)
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/cakes", domain::cakes::handler::routes())
        .mount("/bakeries", domain::bakeries::handlers::routes())
        .mount("/bakers", domain::bakers::handlers::routes())
        .mount("/customers", domain::customers::handler::routes())
        .mount("/lineitems", domain::lineitems::handler::routes())
        .mount("/orders", domain::orders::handler::routes())
        .register("/", catchers![not_found])
}
