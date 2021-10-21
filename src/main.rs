use rocket::tokio::runtime;
use sea_orm_rocket_starter;

fn main() {
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(sea_orm_rocket_starter::rocket().launch());
}
