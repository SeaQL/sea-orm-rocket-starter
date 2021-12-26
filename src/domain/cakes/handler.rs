use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm::entity::*;
use sea_orm_rocket::Connection;

use super::cake::Entity as Cake;
use crate::db::pool::Db;

pub fn routes() -> Vec<rocket::Route> {
    routes![all]
}

#[get("/")]
pub async fn all(conn: Connection<'_, Db>) -> Result<Json<Vec<super::cake::Model>>, Status> {
    let db = conn.into_inner();
    Ok(Json(
        Cake::find()
            .all(db)
            .await
            .expect("could not retrieve cakes")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}
