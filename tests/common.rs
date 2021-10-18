use sea_orm_rocket_starter;
use rocket::local::asynchronous::Client;
use uuid::Uuid;
use sea_orm::*;
use std::env;

pub struct TestContext {
    db_name: String,
    base_url: String,
    pub client: Client,
}

impl TestContext{
    /// Each integration test gets its own database
    /// The DB url is overridden by adding it to the env variables
    /// since the env variables override the values specified in the
    /// Rocket.toml file
    pub async fn init() -> Self {
        // TODO: get the base url from the existing url in the toml file
        // TODO: handle other DB types
        let base_url = "postgres://root:root@localhost".to_owned();
        let url = format!("{}/postgres", base_url);
        let db_name = format!("rocket_starter_test_{}", Uuid::new_v4().to_simple() );
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
        let client = Client::untracked(rocket).await.expect("valid rocket instance");

        Self{
            db_name: db_name,
            base_url: base_url,
            client: client,
        }
    }

    pub async fn tear_down(test_context: &TestContext) {
        let url = format!("{}/postgres", test_context.base_url);
        let db = Database::connect(&url).await.unwrap();

        let _r = db.execute(Statement::from_string(
            DatabaseBackend::Postgres,
            format!("DROP DATABASE IF EXISTS \"{}\" WITH (FORCE);", test_context.db_name)
        ))
        .await;
    }
}
