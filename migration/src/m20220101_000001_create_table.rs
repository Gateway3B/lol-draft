use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct ChampionsMigration;

#[async_trait::async_trait]
impl MigrationTrait for ChampionsMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Champion::Table)
                    .if_not_exists()
                    .col(small_unsigned_uniq(Champion::Id))
                    .primary_key(Index::create().col(Champion::Id))
                    .col(string(Champion::Name))
                    .col(json(Champion::Roles))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Champion::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Champion {
    Table,
    Id,
    Name,
    Roles
}
