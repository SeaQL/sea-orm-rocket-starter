use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm::entity::*;
use sea_orm_rocket::Connection;

use super::order::Entity as Order;
use crate::db::pool::Db;

pub fn routes() -> Vec<rocket::Route> {
    routes![all]
}

#[get("/")]
pub async fn all(conn: Connection<'_, Db>) -> Result<Json<Vec<super::order::Model>>, Status> {
    let db = conn.into_inner();
    Ok(Json(
        Order::find()
            .all(db)
            .await
            .expect("could not retrieve orders")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}
