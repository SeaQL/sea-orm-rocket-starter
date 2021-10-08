use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{sqlx, Connection, Database};
use sea_orm::{entity::*, query::*};
use crate::lil_lib::pool::Db;
// use crate::lil_lib::bakery_chain::*;
use crate::lil_lib::bakery_chain;
use crate::lil_lib::bakery_chain::cake::Entity as Cake;

#[get("/")]
pub async fn all(connection: Connection<Db>) -> Result<Json<Vec<bakery_chain::cake::Model>>, Status> {
      Ok(Json(Cake::find()
        .all(&connection)
        .await
        .expect("could not retrieve posts")
        .into_iter()
        .collect::<Vec<_>>()))
        // .map_err(|error| error_status(error))
}
