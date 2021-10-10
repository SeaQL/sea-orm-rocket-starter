use sea_orm::entity::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::app::bakeries as bakeries;
use crate::app::bakers as bakers;
use crate::app::cakes_bakers as cakes_bakers;
use crate::app::lineitems as lineitems;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Decimal(Some((19, 4)))")]
    pub price: Decimal,
    pub bakery_id: Option<i32>,
    pub gluten_free: bool,
    pub serial: Uuid,
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
    #[sea_orm(has_many = "lineitems::lineitem::Entity")]
    Lineitem,
}

impl Related<bakeries::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<bakers::baker::Entity> for Entity {
    fn to() -> RelationDef {
        cakes_bakers::Relation::Baker.def()
    }

    fn via() -> Option<RelationDef> {
        Some(cakes_bakers::Relation::Cake.def().rev())
    }
}

impl Related<lineitems::lineitem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lineitem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
