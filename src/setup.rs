use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{error::*, sea_query, DbConn, ExecResult};

mod schema;

pub async fn create_tables(db: &DbConn) -> Result<(), DbErr> {
    schema::create_bakery_table(&db).await.unwrap();
    schema::create_baker_table(&db).await.unwrap();
    schema::create_customer_table(&db).await.unwrap();
    schema::create_order_table(&db).await.unwrap();
    schema::create_cake_table(&db).await.unwrap();
    schema::create_cakes_bakers_table(&db).await.unwrap();
    schema::create_lineitem_table(&db).await.unwrap();
    schema::create_metadata_table(&db).await.unwrap();
    schema::create_log_table(&db).await.unwrap();

    Ok(())
}
