use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use sea_orm::{entity::*, query::*};

use super::order::Entity as Order;
use crate::db::pool;

pub fn routes() -> Vec<rocket::Route> {
    routes![all]
}

#[get("/")]
pub async fn all(
    connection: Connection<pool::Db>,
) -> Result<Json<Vec<super::order::Model>>, Status> {
    Ok(Json(
        Order::find()
            .all(&connection)
            .await
            .expect("could not retrieve orders")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}
