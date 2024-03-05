pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240305_095038_create_home_table;
mod m20240305_095105_create_favorite_table;
mod m20240305_095122_create_reservation_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240305_095038_create_home_table::Migration),
            Box::new(m20240305_095105_create_favorite_table::Migration),
            Box::new(m20240305_095122_create_reservation_table::Migration),
        ]
    }
}
