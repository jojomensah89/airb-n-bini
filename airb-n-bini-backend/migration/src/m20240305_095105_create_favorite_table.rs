use sea_orm_migration::prelude::*;
use crate::m20220101_000001_create_table::User;
use crate::m20240305_095038_create_home_table::Home;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Favorite::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Favorite::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Favorite::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Favorite::UserId).string())
                    .col(ColumnDef::new(Favorite::HomeId).uuid())
                    .foreign_key(ForeignKey::create().name("fk-favorites-users-id").from(Favorite::Table,Favorite::UserId).to(User::Table,User::Id))
                    .foreign_key(ForeignKey::create().name("fk-favorites-homes-id").from(Favorite::Table,Favorite::HomeId).to(Home::Table,Home::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Favorite::Table).to_owned())
            .await
    }
}

enum Favorite{
    Table,
    Id,
    UserId,
    HomeId,
    CreatedAt
}