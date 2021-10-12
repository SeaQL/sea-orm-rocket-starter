use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{Connection};
use sea_orm::{entity::*, query::*};

use super::cake::Entity as Cake;
use crate::pool;

pub fn routes() -> Vec<rocket::Route>{
  routes![all]
}

#[get("/")]
pub async fn all(connection: Connection<pool::Db>) -> Result<Json<Vec<super::cake::Model>>, Status> {
      Ok(Json(Cake::find()
        .all(&connection)
        .await
        .expect("could not retrieve cakes")
        .into_iter()
        .collect::<Vec<_>>()))
        // .map_err(|error| error_status(error))
}
