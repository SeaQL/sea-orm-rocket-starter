pub mod common;

mod bakeries_test;
mod bakers_test;

#[rocket::async_test]
async fn main() {
  bakeries_test::main().await;
  bakers_test::main().await;
}
