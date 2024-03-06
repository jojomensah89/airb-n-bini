use crate::m20220101_000001_create_table::User;
use crate::m20240305_095038_create_home_table::Home;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Reservation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reservation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Reservation::StartDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reservation::EndDate).date_time().not_null())
                    .col(
                        ColumnDef::new(Reservation::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reservation::UserId).string().not_null())
                    .col(ColumnDef::new(Reservation::HomeId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reservations-users-id")
                            .from(Reservation::Table, Reservation::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reservations-homes-id")
                            .from(Reservation::Table, Reservation::HomeId)
                            .to(Home::Table, Home::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Reservation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reservation {
    Table,
    Id,
    StartDate,
    EndDate,
    CreatedAt,
    UserId,
    HomeId,
}
