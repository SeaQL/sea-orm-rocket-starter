use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::entity::*;

use super::bakery;
use super::bakery::Entity as Bakery;
use crate::db::pool;

pub fn routes() -> Vec<rocket::Route> {
    routes![all, create, get, update, delete]
}

#[get("/")]
pub async fn all(conn: Connection<pool::Db>) -> Result<Json<Vec<super::bakery::Model>>, Status> {
    Ok(Json(
        Bakery::find()
            .all(&conn)
            .await
            .expect("could not retrieve bakeries")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}

#[get("/<id>")]
pub async fn get(
    conn: Connection<pool::Db>,
    id: i32,
) -> Result<Json<Option<super::bakery::Model>>, Status> {
    let bakery = Bakery::find_by_id(id).one(&conn).await.unwrap();

    Ok(Json(bakery))
}

#[post("/", data = "<input_data>")]
async fn create(
    conn: Connection<pool::Db>,
    input_data: Json<super::bakery::InputData>,
) -> Result<Json<super::bakery::Model>, Status> {
    let new_bakery = input_data.clone().into_inner();

    let bakery = bakery::ActiveModel {
        name: Set(new_bakery.name.to_owned()),
        profit_margin: Set(new_bakery.profit_margin),
        ..Default::default()
    };

    let res = Bakery::insert(bakery)
        .exec(&conn)
        .await
        .expect("could not insert bakery");

    Ok(Json(fetch_bakery(conn, res.last_insert_id).await))
}

#[put("/<id>", data = "<input_data>")]
async fn update(
    conn: Connection<pool::Db>,
    id: i32,
    input_data: Json<super::bakery::InputData>,
) -> Result<Json<super::bakery::Model>, Status> {
    let input_data = input_data.clone().into_inner();

    let r = bakery::ActiveModel {
        id: Set(id),
        name: Set(input_data.name.to_owned()),
        profit_margin: Set(input_data.profit_margin),
        ..Default::default()
    }
    .update(&conn)
    .await;

    Ok(Json(fetch_bakery(conn, id).await))
}

#[delete("/<id>")]
async fn delete(conn: Connection<pool::Db>, id: i32) -> Result<Json<super::bakery::Model>, Status> {
    let bakery = Bakery::find_by_id(id).one(&conn).await.unwrap().unwrap();

    let bakery_active_model: bakery::ActiveModel = bakery.clone().into();

    bakery_active_model.delete(&conn).await.unwrap();

    Ok(Json(bakery))
}

async fn fetch_bakery(conn: Connection<pool::Db>, id: i32) -> super::bakery::Model {
    Bakery::find_by_id(id)
        .one(&conn)
        .await
        .expect("could not find bakery")
        .unwrap()
}
