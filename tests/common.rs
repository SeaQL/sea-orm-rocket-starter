use sea_orm_rocket_starter;
// use once_cell::sync::OnceCell;
// use rocket::http::Status;
use rocket::local::blocking::Client;

// pub fn test_client() -> &'static Client {
//     static INSTANCE: OnceCell<Client> = OnceCell::new();
//     INSTANCE.get_or_init(|| {
//         let rocket = sea_orm_rocket_starter::rocket();
//         Client::new(rocket).expect("valid rocket instance")
//     })
// }
pub fn test_client() -> Client {
    let rocket = sea_orm_rocket_starter::rocket();
    Client::tracked(rocket).expect("valid rocket instance")
}
