use sea_orm_rocket_starter;
// use once_cell::sync::OnceCell;
// use rocket::http::Status;
use rocket::local::asynchronous::Client;
use uuid::Uuid;
use sea_orm::*;
use std::env;

// pub fn test_client() -> &'static Client {
//     static INSTANCE: OnceCell<Client> = OnceCell::new();
//     INSTANCE.get_or_init(|| {
//         let rocket = sea_orm_rocket_starter::rocket();
//         Client::new(rocket).expect("valid rocket instance")
//     })
// }

/// Each integration test gets its own database
/// The DB url is overridden by adding it to the env variables
/// since the env variables override the values specified in the
/// Rocket.toml file
pub async fn test_client() -> Client {
    // TODO: get the base url from the existing url in the toml file
    // TODO: handle other DB types
    let base_url = "postgres://root:root@localhost";
    let url = format!("{}/postgres", base_url);
    let db_name = format!("rocket_starter_test_{}", Uuid::new_v4() );
    let db = sea_orm::Database::connect(&url).await.unwrap();
    let _drop_db_result = db
        .execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
        ))
        .await;

    let _create_db_result = db
        .execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            format!("CREATE DATABASE \"{}\";", db_name),
        ))
        .await;

    let url = format!("{}/{}", base_url, db_name);

    // Override the DB url by adding an env var
    env::set_var("ROCKET_APP_DATABASES+ROCKET_STARTER+URL", url);

    let rocket = sea_orm_rocket_starter::rocket();
    Client::tracked(rocket).await.expect("valid rocket instance")
}
