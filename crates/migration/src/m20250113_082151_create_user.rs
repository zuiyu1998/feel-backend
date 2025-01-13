use entity::user::{UserBaseColumn, UserBaseEntity};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserBaseEntity)
                    .if_not_exists()
                    .col(big_integer(UserBaseColumn::Id).auto_increment())
                    .col(string(UserBaseColumn::Uid))
                    .col(string(UserBaseColumn::Nikename))
                    .col(big_integer(UserBaseColumn::CreateAt))
                    .col(big_integer(UserBaseColumn::UpdateAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserBaseEntity).to_owned())
            .await
    }
}
