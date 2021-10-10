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
        .mount("/cakes",
            routes![app::cakes::handler::all,
                // app::cakes::handler::get,
                // app::cakes::handler::post,
                // app::cakes::handler::put,
                // app::cakes::handler::delete
                ],
        )
        .mount("/bakeries",
            routes![app::bakeries::handler::all,
                // app::cakes::handler::get,
                // app::cakes::handler::post,
                // app::cakes::handler::put,
                // app::cakes::handler::delete
                ],
        )
        .mount("/bakers",
            routes![app::bakers::handler::all,
                // app::cakes::handler::get,
                // app::cakes::handler::post,
                // app::cakes::handler::put,
                // app::cakes::handler::delete
                ],
        )
        .register("/", catchers![not_found])
}
