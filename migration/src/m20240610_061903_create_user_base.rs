use entity::user::{UserBaseColumn, UserBaseEntity};
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(UserBaseColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::Nikename)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserBaseColumn::Avatar).string().not_null())
                    .col(ColumnDef::new(UserBaseColumn::Uid).string().not_null())
                    .col(
                        ColumnDef::new(UserBaseColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserBaseColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
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
