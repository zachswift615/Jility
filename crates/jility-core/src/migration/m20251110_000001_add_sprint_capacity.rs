use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add capacity column to sprint table
        manager
            .alter_table(
                Table::alter()
                    .table(Sprint::Table)
                    .add_column(
                        ColumnDef::new(Sprint::Capacity)
                            .integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove capacity column
        manager
            .alter_table(
                Table::alter()
                    .table(Sprint::Table)
                    .drop_column(Sprint::Capacity)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Identifier for Sprint table
#[derive(Iden)]
enum Sprint {
    Table,
    Capacity,
}
