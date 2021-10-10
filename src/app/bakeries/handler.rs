use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{Connection};
use sea_orm::{entity::*, query::*};
use super::bakery::Entity as Bakery;

use super::pool;

#[get("/")]
pub async fn all(connection: Connection<pool::Db>) -> Result<Json<Vec<super::bakery::Model>>, Status> {
      Ok(Json(Bakery::find()
        .all(&connection)
        .await
        .expect("could not retrieve bakeries")
        .into_iter()
        .collect::<Vec<_>>()))
        // .map_err(|error| error_status(error))
}
