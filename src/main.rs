#[macro_use] extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};
use rocket_db_pools::{Database};

mod app;
use app::db::{pool, migrations};

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
        .mount("/cakes", app::cakes::handler::routes() )
        .mount("/bakeries", app::bakeries::handler::routes() )
        .mount("/bakers", app::bakers::handler::routes() )
        .mount("/customers", app::customers::handler::routes() )
        .mount("/lineitems", app::lineitems::handler::routes() )
        .mount("/orders", app::orders::handler::routes() )
        .register("/", catchers![not_found])
}
