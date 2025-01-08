use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct DraftMigration;

#[async_trait::async_trait]
impl MigrationTrait for DraftMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Draft::Table)
                    .if_not_exists()
                    .col(string(Draft::DraftId))
                    .primary_key(Index::create().col(Draft::DraftId))
                    .col(integer_null(Draft::BlueBan1))
                    .col(integer_null(Draft::BlueBan2))
                    .col(integer_null(Draft::BlueBan3))
                    .col(integer_null(Draft::BlueBan4))
                    .col(integer_null(Draft::BlueBan5))
                    .col(integer_null(Draft::RedBan1))
                    .col(integer_null(Draft::RedBan2))
                    .col(integer_null(Draft::RedBan3))
                    .col(integer_null(Draft::RedBan4))
                    .col(integer_null(Draft::RedBan5))
                    .col(integer_null(Draft::BluePick1))
                    .col(integer_null(Draft::BluePick2))
                    .col(integer_null(Draft::BluePick3))
                    .col(integer_null(Draft::BluePick4))
                    .col(integer_null(Draft::BluePick5))
                    .col(integer_null(Draft::RedPick1))
                    .col(integer_null(Draft::RedPick2))
                    .col(integer_null(Draft::RedPick3))
                    .col(integer_null(Draft::RedPick4))
                    .col(integer_null(Draft::RedPick5))
                    .col(date_time(Draft::DateCompleted))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Draft::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Draft {
    Table,
    DraftId,
    #[sea_orm(iden = "blue_ban_1")]
    BlueBan1,
    #[sea_orm(iden = "blue_ban_2")]
    BlueBan2,
    #[sea_orm(iden = "blue_ban_3")]
    BlueBan3,
    #[sea_orm(iden = "blue_ban_4")]
    BlueBan4,
    #[sea_orm(iden = "blue_ban_5")]
    BlueBan5,
    #[sea_orm(iden = "red_ban_1")]
    RedBan1,
    #[sea_orm(iden = "red_ban_2")]
    RedBan2,
    #[sea_orm(iden = "red_ban_3")]
    RedBan3,
    #[sea_orm(iden = "red_ban_4")]
    RedBan4,
    #[sea_orm(iden = "red_ban_5")]
    RedBan5,
    #[sea_orm(iden = "blue_pick_1")]
    BluePick1,
    #[sea_orm(iden = "blue_pick_2")]
    BluePick2,
    #[sea_orm(iden = "blue_pick_3")]
    BluePick3,
    #[sea_orm(iden = "blue_pick_4")]
    BluePick4,
    #[sea_orm(iden = "blue_pick_5")]
    BluePick5,
    #[sea_orm(iden = "red_pick_1")]
    RedPick1,
    #[sea_orm(iden = "red_pick_2")]
    RedPick2,
    #[sea_orm(iden = "red_pick_3")]
    RedPick3,
    #[sea_orm(iden = "red_pick_4")]
    RedPick4,
    #[sea_orm(iden = "red_pick_5")]
    RedPick5,
    DateCompleted,
}
