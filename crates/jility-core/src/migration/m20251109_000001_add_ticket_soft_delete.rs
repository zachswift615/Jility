use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add deleted_at column to tickets table for soft delete
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .add_column(
                        ColumnDef::new(Ticket::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on deleted_at for faster filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_deleted_at")
                    .table(Ticket::Table)
                    .col(Ticket::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the index first
        manager
            .drop_index(
                Index::drop()
                    .name("idx_tickets_deleted_at")
                    .to_owned(),
            )
            .await?;

        // Remove deleted_at column
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .drop_column(Ticket::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Identifier for Ticket table
#[derive(Iden)]
enum Ticket {
    Table,
    DeletedAt,
}
