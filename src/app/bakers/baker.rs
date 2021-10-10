use sea_orm::entity::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::app::cakes as cakes;
use crate::app::bakeries as bakeries;
use crate::app::cakes_bakers as cakes_bakers;
use crate::app::lineitems as lineitems;
use crate::app::customers as customers;
use crate::app::orders as orders;


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
        belongs_to = "bakeries::bakery::Entity",
        from = "Column::BakeryId",
        to = "bakeries::bakery::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Bakery,
}

impl Related<bakeries::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<cakes::cake::Entity> for Entity {
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

    type ToEntity = customers::customer::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            cakes_bakers::Relation::Baker.def().rev(),
            cakes_bakers::Relation::Cake.def(),
            lineitems::lineitem::Relation::Cake.def().rev(),
            lineitems::lineitem::Relation::Order.def(),
            orders::order::Relation::Customer.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
