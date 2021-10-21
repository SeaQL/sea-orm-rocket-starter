use crate::domain::bakeries::bakery;
use crate::domain::bakers::baker;
use crate::domain::cakes_bakers;
use crate::domain::lineitems::lineitem;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

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
        belongs_to = "bakery::Entity",
        from = "Column::BakeryId",
        to = "bakery::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Bakery,
    #[sea_orm(has_many = "lineitem::Entity")]
    Lineitem,
}

impl Related<bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<baker::Entity> for Entity {
    fn to() -> RelationDef {
        cakes_bakers::Relation::Baker.def()
    }

    fn via() -> Option<RelationDef> {
        Some(cakes_bakers::Relation::Cake.def().rev())
    }
}

impl Related<lineitem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lineitem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
