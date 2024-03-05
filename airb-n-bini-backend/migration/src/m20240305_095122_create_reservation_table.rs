use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

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
                    .col(ColumnDef::new(Reservation::StartDate).date_time().not_null())
                    .col(ColumnDef::new(Reservation::EndDate).date_time().not_null())
                    .col(ColumnDef::new(Reservation::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Reservation::UserId).string())
                    .col(ColumnDef::new(Reservation::HomeId).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Reservation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reservation{
    Table,
    Id,
    StartDate,
    EndDate,
    CreatedAt,
    UserId,
    HomeId
}