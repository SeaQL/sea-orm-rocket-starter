use async_trait::async_trait;
use rocket_db_pools::{rocket::figment::Figment, Config};
use rocket_db_pools::{Database};
use std::env;

#[derive(Debug)]
pub struct RocketDbPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl rocket_db_pools::Pool for RocketDbPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        println!("ROCKET_ENV: {:#?}", env::var("ROCKET_ENV"));

        // TODO: fix this
        // let u = figment.clone().select(env::var("ROCKET_ENV").unwrap()).extract::<Config>().unwrap();
        // println!("figment: {:#?}", u);

        let config = figment.clone().select(env::var("ROCKET_ENV").unwrap()).extract::<Config>().unwrap();
        let conn = sea_orm::Database::connect(&config.url).await.unwrap();

        Ok(RocketDbPool { conn })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }
}

#[derive(Database, Debug)]
#[database("rocket_starter")]
pub struct Db(RocketDbPool);

