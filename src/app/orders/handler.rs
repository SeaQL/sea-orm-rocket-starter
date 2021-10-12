use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{Connection};
use sea_orm::{entity::*, query::*};

use super::order::Entity as Order;
use crate::pool;

#[get("/")]
pub async fn all(connection: Connection<pool::Db>) -> Result<Json<Vec<super::order::Model>>, Status> {
      Ok(Json(Order::find()
        .all(&connection)
        .await
        .expect("could not retrieve orders")
        .into_iter()
        .collect::<Vec<_>>()))
        // .map_err(|error| error_status(error))
}
