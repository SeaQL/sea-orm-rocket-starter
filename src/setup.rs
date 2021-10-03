use sea_orm::{error::*, DbConn, ExecResult, EntityTrait, Schema};
use super::bakery_chain::*;

async fn create_table<E>(
    db: &DbConn,
    entity: E,
) -> Result<ExecResult, DbErr>
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let stmt = builder.build(&Schema::create_table_from_entity(entity));
    db.execute(stmt).await
}


pub async fn create_tables(db: &DbConn) -> Result<(), DbErr> {
    let _ = create_table(db, Bakery).await;
    let _ = create_table(db, Baker).await;
    let _ = create_table(db, Customer).await;
    let _ = create_table(db, Order).await;
    let _ = create_table(db, Lineitem).await;
    let _ = create_table(db, CakesBakers).await;
    let _ = create_table(db, Cake).await;
    let _ = create_table(db, Metadata).await;
    let _ = create_table(db, Applog).await;

    Ok(())
}

