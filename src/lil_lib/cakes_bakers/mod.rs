use sea_orm::entity::prelude::*;
use crate::lil_lib::cakes as cakes;
use crate::lil_lib::bakers as bakers;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cakes_bakers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cake_id: i32,
    #[sea_orm(primary_key)]
    pub baker_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "cakes::cake::Entity",
        from = "Column::CakeId",
        to = "cakes::cake::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Cake,
    #[sea_orm(
        belongs_to = "bakers::baker::Entity",
        from = "Column::BakerId",
        to = "bakers::baker::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Baker,
}

impl ActiveModelBehavior for ActiveModel {}
