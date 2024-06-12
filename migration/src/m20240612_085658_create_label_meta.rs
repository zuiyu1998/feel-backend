use entity::label::{LabelMetaColumn, LabelMetaEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LabelMetaEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LabelMetaColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::Description)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::Effct)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::IsDelete)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::IsEnable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LabelMetaColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LabelMetaEntity).to_owned())
            .await
    }
}
