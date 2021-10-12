use sea_orm::entity::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::domain::cakes::cake as cake;
use crate::domain::bakeries::bakery as bakery;
use crate::domain::cakes_bakers as cakes_bakers;
use crate::domain::lineitems::lineitem as lineitem;
use crate::domain::customers::customer as customer;
use crate::domain::orders::order as order;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "baker")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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
