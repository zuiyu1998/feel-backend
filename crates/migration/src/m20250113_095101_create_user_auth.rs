use entity::user::{UserAuthColumn, UserAuthEntity};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserAuthEntity)
                    .if_not_exists()
                    .col(big_integer(UserAuthColumn::UserId))
                    .col(string(UserAuthColumn::LoginType))
                    .col(string(UserAuthColumn::AuthToken))
                    .col(string(UserAuthColumn::AuthName))
                    .primary_key(
                        Index::create()
                            .col(UserAuthColumn::UserId)
                            .col(UserAuthColumn::LoginType),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAuthEntity).to_owned())
            .await
    }
}
