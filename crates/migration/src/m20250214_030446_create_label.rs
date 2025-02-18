use entity::label::*;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LabelBaseEntity)
                    .if_not_exists()
                    .col(pk_auto(LabelBaseColumn::Id))
                    .col(string(LabelBaseColumn::Name))
                    .col(string(LabelBaseColumn::Description))
                    .col(integer(LabelBaseColumn::Effect))
                    .col(big_integer(LabelBaseColumn::CreateAt))
                    .col(big_integer(LabelBaseColumn::UpdateAt))
                    .col(boolean(LabelBaseColumn::IsDelete).default(false))
                    .col(boolean(LabelBaseColumn::IsEnable).default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LabelBaseEntity).to_owned())
            .await
    }
}
