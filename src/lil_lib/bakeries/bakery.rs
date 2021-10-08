use sea_orm::entity::prelude::*;
use crate::lil_lib::cakes as cakes;
use crate::lil_lib::bakeries as bakeries;
use crate::lil_lib::bakers as bakers;
use crate::lil_lib::cakes_bakers as cakes_bakers;
use crate::lil_lib::lineitems as lineitems;
use crate::lil_lib::customers as customers;
use crate::lil_lib::orders as orders;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bakery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub profit_margin: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "bakers::baker::Entity")]
    Baker,
    #[sea_orm(has_many = "orders::order::Entity")]
    Order,
    #[sea_orm(has_many = "cakes::cake::Entity")]
    Cake,
}

impl Related<bakers::baker::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Baker.def()
    }
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
