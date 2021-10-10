use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{Connection};
use sea_orm::{entity::*, query::*};

use super::lineitem::Entity as Lineitem;
use crate::pool;

#[get("/")]
pub async fn all(connection: Connection<pool::Db>) -> Result<Json<Vec<super::lineitem::Model>>, Status> {
      Ok(Json(Lineitem::find()
        .all(&connection)
        .await
        .expect("could not retrieve lineitems")
        .into_iter()
        .collect::<Vec<_>>()))
        // .map_err(|error| error_status(error))
}
