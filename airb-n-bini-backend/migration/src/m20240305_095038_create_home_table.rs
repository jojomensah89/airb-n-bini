use sea_orm_migration::prelude::*;
use crate::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Home::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Home::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Home::Title).string())
                    .col(ColumnDef::new(Home::Description).string())
                    .col(ColumnDef::new(Home::Guests).string())
                    .col(ColumnDef::new(Home::BedRooms).string())
                    .col(ColumnDef::new(Home::BathRooms).string())
                    .col(ColumnDef::new(Home::Country).string())
                    .col(ColumnDef::new(Home::Photo).string())
                    .col(ColumnDef::new(Home::Price).integer())
                    .col(ColumnDef::new(Home::CategoryName).integer())
                    .col(ColumnDef::new(Home::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Home::UserId).string())
                    .foreign_key(ForeignKey::create().name("fk-homes-users-id").from(Home::Table,Home::UserId).to(User::Table,User::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Home::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Home{
    Table,
    Id,
    Title,
    Description,
    Guests,
    BedRooms,
    BathRooms,
    Country,
    Photo,
    Price,
    CategoryName,
    CreatedAt,
    UserId,
}