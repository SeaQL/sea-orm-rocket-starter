use sea_orm::entity::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::app::cakes::cake as cake;
use crate::app::bakers::baker as baker;
use crate::app::orders::order as order;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "bakery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub profit_margin: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "baker::Entity")]
    Baker,
    #[sea_orm(has_many = "order::Entity")]
    Order,
    #[sea_orm(has_many = "cake::Entity")]
    Cake,
}

impl Related<baker::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Baker.def()
    }
}

impl Related<order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<cake::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cake.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
