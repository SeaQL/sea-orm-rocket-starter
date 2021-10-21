use crate::domain::cakes;
use crate::domain::orders;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "lineitem")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub price: Decimal,
    pub quantity: i32,
    pub order_id: i32,
    pub cake_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "orders::order::Entity",
        from = "Column::OrderId",
        to = "orders::order::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Order,
    #[sea_orm(
        belongs_to = "cakes::cake::Entity",
        from = "Column::CakeId",
        to = "cakes::cake::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Cake,
}

impl Related<orders::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<cakes::cake::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cake.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
