use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm::{entity::*, DbConn};
use sea_orm_rocket::Connection;

use super::bakery;
use super::bakery::Entity as Bakery;
use crate::db::pool::Db;

pub fn routes() -> Vec<rocket::Route> {
    routes![all, create, get, update, delete]
}

#[get("/")]
pub async fn all(conn: Connection<'_, Db>) -> Result<Json<Vec<super::bakery::Model>>, Status> {
    let db = conn.into_inner();
    Ok(Json(
        Bakery::find()
            .all(db)
            .await
            .expect("could not retrieve bakeries")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}

#[get("/<id>")]
pub async fn get(
    conn: Connection<'_, Db>,
    id: i32,
) -> Result<Json<Option<super::bakery::Model>>, Status> {
    let db = conn.into_inner();
    let bakery = Bakery::find_by_id(id).one(db).await.unwrap();

    Ok(Json(bakery))
}

#[post("/", data = "<input_data>")]
async fn create(
    conn: Connection<'_, Db>,
    input_data: Json<super::bakery::InputData>,
) -> Result<Json<super::bakery::Model>, Status> {
    let db = conn.into_inner();
    let new_bakery = input_data.clone();

    let bakery = bakery::ActiveModel {
        name: Set(new_bakery.name.to_owned()),
        profit_margin: Set(new_bakery.profit_margin),
        ..Default::default()
    };

    let res = Bakery::insert(bakery)
        .exec(db)
        .await
        .expect("could not insert bakery");

    Ok(Json(fetch_bakery(db, res.last_insert_id).await))
}

#[put("/<id>", data = "<input_data>")]
async fn update(
    conn: Connection<'_, Db>,
    id: i32,
    input_data: Json<super::bakery::InputData>,
) -> Result<Json<super::bakery::Model>, Status> {
    let db = conn.into_inner();
    let input_data = input_data.clone();

    let r = bakery::ActiveModel {
        id: Set(id),
        name: Set(input_data.name.to_owned()),
        profit_margin: Set(input_data.profit_margin),
        ..Default::default()
    }
    .update(db)
    .await;

    Ok(Json(fetch_bakery(db, id).await))
}

#[delete("/<id>")]
async fn delete(conn: Connection<'_, Db>, id: i32) -> Result<Json<super::bakery::Model>, Status> {
    let db = conn.into_inner();
    let bakery = Bakery::find_by_id(id).one(db).await.unwrap().unwrap();

    let bakery_active_model: bakery::ActiveModel = bakery.clone().into();

    bakery_active_model.delete(db).await.unwrap();

    Ok(Json(bakery))
}

async fn fetch_bakery(db: &DbConn, id: i32) -> super::bakery::Model {
    Bakery::find_by_id(id)
        .one(db)
        .await
        .expect("could not find bakery")
        .unwrap()
}
