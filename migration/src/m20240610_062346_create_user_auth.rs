use entity::user::{UserAuthColumn, UserAuthEntity};
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(UserAuthColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::AuthName)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserAuthColumn::AuthCode).binary().not_null())
                    .col(
                        ColumnDef::new(UserAuthColumn::IsDelete)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::IsEnable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAuthColumn::AuthType)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserAuthColumn::UserId).integer().not_null())
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
