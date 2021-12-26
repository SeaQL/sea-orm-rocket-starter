use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm::{entity::*, DbConn};
use sea_orm_rocket::Connection;

use super::baker;
use super::baker::Entity as Baker;
use crate::db::pool::Db;

pub fn routes() -> Vec<rocket::Route> {
    routes![all, create, get, update, delete]
}

#[get("/")]
pub async fn all(conn: Connection<'_, Db>) -> Result<Json<Vec<super::baker::Model>>, Status> {
    let db = conn.into_inner();
    Ok(Json(
        Baker::find()
            .all(db)
            .await
            .expect("could not retrieve bakers")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}

#[get("/<id>")]
pub async fn get(
    conn: Connection<'_, Db>,
    id: i32,
) -> Result<Json<Option<super::baker::Model>>, Status> {
    let db = conn.into_inner();
    let baker = Baker::find_by_id(id).one(db).await.unwrap();

    Ok(Json(baker))
}

#[post("/", data = "<input_data>")]
async fn create(
    conn: Connection<'_, Db>,
    input_data: Json<super::baker::InputData>,
) -> Result<Json<super::baker::Model>, Status> {
    let db = conn.into_inner();
    let new_baker = input_data.clone();

    let baker = baker::ActiveModel {
        name: Set(new_baker.name.to_owned()),
        contact_details: Set(serde_json::json!(new_baker.contact_details)),
        ..Default::default()
    };

    let res = Baker::insert(baker)
        .exec(db)
        .await
        .expect("could not insert baker");

    Ok(Json(fetch_baker(db, res.last_insert_id).await))
}

#[put("/<id>", data = "<input_data>")]
async fn update(
    conn: Connection<'_, Db>,
    id: i32,
    input_data: Json<super::baker::InputData>,
) -> Result<Json<super::baker::Model>, Status> {
    let db = conn.into_inner();
    let input_data = input_data.clone();

    let r = baker::ActiveModel {
        id: Set(id),
        name: Set(input_data.name.to_owned()),
        contact_details: Set(serde_json::json!(input_data.contact_details)),
        ..Default::default()
    }
    .update(db)
    .await;

    Ok(Json(fetch_baker(db, id).await))
}

#[delete("/<id>")]
async fn delete(conn: Connection<'_, Db>, id: i32) -> Result<Json<super::baker::Model>, Status> {
    let db = conn.into_inner();
    let baker = Baker::find_by_id(id).one(db).await.unwrap().unwrap();

    let baker_active_model: baker::ActiveModel = baker.clone().into();

    baker_active_model.delete(db).await.unwrap();

    Ok(Json(baker))
}

async fn fetch_baker(db: &DbConn, id: i32) -> super::baker::Model {
    Baker::find_by_id(id)
        .one(db)
        .await
        .expect("could not find bakery")
        .unwrap()
}
