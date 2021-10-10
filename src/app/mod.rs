// pub mod bakery_chain;
// pub mod setup;
// pub mod pool;

pub mod cakes;
pub mod bakers;
pub mod bakeries;
pub mod cakes_bakers;
pub mod customers;
pub mod lineitems;
pub mod orders;

pub use bakers::baker::Entity as Baker;
pub use bakeries::bakery::Entity as Bakery;
pub use cakes::cake::Entity as Cake;
pub use cakes_bakers::Entity as CakesBakers;
pub use customers::customer::Entity as Customer;
pub use lineitems::lineitem::Entity as Lineitem;
pub use orders::order::Entity as Order;

pub mod db;
pub use db::{pool, migrations};
