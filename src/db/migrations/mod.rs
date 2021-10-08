use sea_orm::{DbConn, EntityTrait, Schema};
use crate::lil_lib::*;

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let stmt = builder.build(Schema::create_table_from_entity(entity).if_not_exists());

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn create_tables(db: &DbConn) {
    create_table(db, Bakery).await;
    create_table(db, Baker).await;
    create_table(db, Customer).await;
    create_table(db, Order).await;
    create_table(db, Cake).await;
    create_table(db, CakesBakers).await;
    create_table(db, Lineitem).await;
}
