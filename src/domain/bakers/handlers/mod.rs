use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use sea_orm::{entity::*, query::*};

use super::baker;
use super::baker::Entity as Baker;
use crate::db::pool;

pub fn routes() -> Vec<rocket::Route> {
    routes![all, create, get, update, delete]
}

#[get("/")]
pub async fn all(conn: Connection<pool::Db>) -> Result<Json<Vec<super::baker::Model>>, Status> {
    Ok(Json(
        Baker::find()
            .all(&conn)
            .await
            .expect("could not retrieve bakers")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}

#[get("/<id>")]
pub async fn get(
    conn: Connection<pool::Db>,
    id: i32,
) -> Result<Json<Option<super::baker::Model>>, Status> {
    let baker = Baker::find_by_id(id).one(&conn).await.unwrap();

    Ok(Json(baker))
}

#[post("/", data = "<input_data>")]
async fn create(
    conn: Connection<pool::Db>,
    input_data: Json<super::baker::InputData>,
) -> Result<Json<super::baker::Model>, Status> {
    let new_baker = input_data.clone().into_inner();

    let baker = baker::ActiveModel {
        name: Set(new_baker.name.to_owned()),
        contact_details: Set(serde_json::json!(new_baker.contact_details)),
        ..Default::default()
    };

    let res = Baker::insert(baker)
        .exec(&conn)
        .await
        .expect("could not insert baker");

    Ok(Json(fetch_baker(conn, res.last_insert_id).await))
}

#[put("/<id>", data = "<input_data>")]
async fn update(
    conn: Connection<pool::Db>,
    id: i32,
    input_data: Json<super::baker::InputData>,
) -> Result<Json<super::baker::Model>, Status> {
    let input_data = input_data.clone().into_inner();

    let r = baker::ActiveModel {
        id: Set(id),
        name: Set(input_data.name.to_owned()),
        contact_details: Set(serde_json::json!(input_data.contact_details)),
        ..Default::default()
    }
    .update(&conn)
    .await;

    Ok(Json(fetch_baker(conn, id).await))
}

#[delete("/<id>")]
async fn delete(conn: Connection<pool::Db>, id: i32) -> Result<Json<super::baker::Model>, Status> {
    let baker = Baker::find_by_id(id).one(&conn).await.unwrap().unwrap();

    let baker_active_model: baker::ActiveModel = baker.clone().into();

    baker_active_model.delete(&conn).await.unwrap();

    Ok(Json(baker))
}

async fn fetch_baker(conn: Connection<pool::Db>, id: i32) -> super::baker::Model {
    Baker::find_by_id(id)
        .one(&conn)
        .await
        .expect("could not find bakery")
        .unwrap()
}
