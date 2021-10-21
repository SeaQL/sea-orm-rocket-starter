use crate::domain::bakeries::bakery;
use crate::domain::cakes::cake;
use crate::domain::cakes_bakers;
use crate::domain::customers::customer;
use crate::domain::lineitems::lineitem;
use crate::domain::orders::order;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "baker")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub contact_details: Json,
    pub bakery_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, Deserialize)]
pub struct InputData {
    pub name: String,
    pub contact_details: Json,
    pub bakery_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "bakery::Entity",
        from = "Column::BakeryId",
        to = "bakery::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Bakery,
}

impl Related<bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<cake::Entity> for Entity {
    fn to() -> RelationDef {
        cakes_bakers::Relation::Cake.def()
    }

    fn via() -> Option<RelationDef> {
        Some(cakes_bakers::Relation::Baker.def().rev())
    }
}

pub struct BakedForCustomer;

impl Linked for BakedForCustomer {
    type FromEntity = Entity;

    type ToEntity = customer::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            cakes_bakers::Relation::Baker.def().rev(),
            cakes_bakers::Relation::Cake.def(),
            lineitem::Relation::Cake.def().rev(),
            lineitem::Relation::Order.def(),
            order::Relation::Customer.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
