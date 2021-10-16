use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use sea_orm::{entity::*, query::*};

use super::bakery::Entity as Bakery;
use super::bakery;
use crate::db::pool;

pub fn routes() -> Vec<rocket::Route> {
    routes![all, create]
}

#[get("/")]
pub async fn all(
    connection: Connection<pool::Db>,
) -> Result<Json<Vec<super::bakery::Model>>, Status> {
    Ok(Json(
        Bakery::find()
            .all(&connection)
            .await
            .expect("could not retrieve bakeries")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
    // .map_err(|error| error_status(error))
}

#[post("/", data = "<new_bakery>")]
async fn create(conn: Connection<pool::Db>, new_bakery: Json<bakery::Model>) -> Result<Json<super::bakery::Model>, Status> {
    let mut new_bakery = new_bakery.clone().into_inner();

    let bakery = bakery::ActiveModel {
        name: Set(new_bakery.name.to_owned()),
        profit_margin: Set(new_bakery.profit_margin),
        ..Default::default()
    }
    .save(&conn)
    .await
    .expect("error");
    new_bakery.id = bakery.id.unwrap();

    Ok(Json(new_bakery))
}
