use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use sea_orm::{entity::*, query::*};

use super::bakery::Entity as Bakery;
use super::bakery;
use crate::db::pool;

pub fn routes() -> Vec<rocket::Route> {
    routes![all, create, get]
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

#[get("/<id>")]
pub async fn get(
    connection: Connection<pool::Db>, id: i32
) -> Result<Json<Option<super::bakery::Model>>, Status> {
println!("handler id: {:#?}", id);
let v =         Bakery::find()
            .all(&connection)
            .await
            .expect("could not retrieve bakeries")
            .into_iter()
            .collect::<Vec<_>>();

           println!("v: {:#?}", v);


    let bakery =         Bakery::find_by_id(id)
            .one(&connection)
            .await
            .unwrap();
println!("handler bakery: {:#?}", bakery);

    Ok(Json(bakery ))
}

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, serde::Serialize, Deserialize)]
pub struct BakeryInput {
    pub name: String,
    pub profit_margin: f64,
}


#[post("/", data = "<new_bakery>")]
async fn create(conn: Connection<pool::Db>, new_bakery: Json<BakeryInput>) -> Result<Json<super::bakery::Model>, Status> {
    let mut new_bakery = new_bakery.clone().into_inner();

    let bakery = bakery::ActiveModel {
        name: Set(new_bakery.name.to_owned()),
        profit_margin: Set(new_bakery.profit_margin),
        ..Default::default()
    };

        let res = Bakery::insert(bakery)
        .exec(&conn)
        .await
        .expect("could not insert bakery");


    // .save(&conn)
    // .await
    // .expect("error");
    println!("!! create bakery: {:#?}", res);

        let new_bakery: bakery::Model = Bakery::find_by_id(res.last_insert_id)
        .one(&conn)
        .await
        .expect("could not find bakery")
        .unwrap();

    // new_bakery.id = bakery.id.unwrap();
println!("new_bakery: {:#?}", new_bakery);

let result = Json(new_bakery);
println!("@@ json result: {:#?}", result);

    Ok(result)
}


    // let baker_bob = baker::ActiveModel {
    //     name: Set("Baker Bob".to_owned()),
    //     contact_details: Set(serde_json::json!(baker_bob_contact)),
    //     bakery_id: Set(Some(bakery_insert_res.last_insert_id as i32)),
    //     ..Default::default()
    // };
    // let res = Baker::insert(baker_bob)
    //     .exec(db)
    //     .await
    //     .expect("could not insert baker");

    // let baker: Option<baker::Model> = Baker::find_by_id(res.last_insert_id)
    //     .one(db)
    //     .await
    //     .expect("could not find baker");
