use sea_orm::entity::prelude::*;
use crate::lil_lib::cakes as cakes;
use crate::lil_lib::bakeries as bakeries;
use crate::lil_lib::bakers as bakers;
use crate::lil_lib::cakes_bakers as cakes_bakers;
use crate::lil_lib::lineitems as lineitems;
use crate::lil_lib::customers as customers;
use crate::lil_lib::orders as orders;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "orders::order::Entity")]
    Order,
}

impl Related<orders::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
