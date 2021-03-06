use crate::domain::bakeries;
use crate::domain::customers;
use crate::domain::lineitems;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub total: Decimal,
    pub bakery_id: i32,
    pub customer_id: i32,
    pub placed_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "bakeries::bakery::Entity",
        from = "Column::BakeryId",
        to = "bakeries::bakery::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Bakery,
    #[sea_orm(
        belongs_to = "customers::customer::Entity",
        from = "Column::CustomerId",
        to = "customers::customer::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Customer,
    #[sea_orm(has_many = "lineitems::lineitem::Entity")]
    Lineitem,
}

impl Related<bakeries::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<customers::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<lineitems::lineitem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lineitem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
