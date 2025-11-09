use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add is_epic column (defaults to false for existing tickets)
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .add_column(
                        ColumnDef::new(Ticket::IsEpic)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // Add epic_color column (nullable)
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .add_column(
                        ColumnDef::new(Ticket::EpicColor)
                            .string_len(50)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add parent_epic_id column
        // Note: SQLite doesn't support adding foreign key constraints to existing tables
        // The foreign key relationship will be enforced at the application level
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .add_column(
                        ColumnDef::new(Ticket::ParentEpicId)
                            .uuid()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on is_epic for faster epic filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_is_epic")
                    .table(Ticket::Table)
                    .col(Ticket::IsEpic)
                    .to_owned(),
            )
            .await?;

        // Create index on parent_epic_id for faster child ticket queries
        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_parent_epic_id")
                    .table(Ticket::Table)
                    .col(Ticket::ParentEpicId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes first
        manager
            .drop_index(
                Index::drop()
                    .name("idx_tickets_parent_epic_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tickets_is_epic")
                    .to_owned(),
            )
            .await?;

        // Remove columns (no foreign key to drop for SQLite)
        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .drop_column(Ticket::ParentEpicId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .drop_column(Ticket::EpicColor)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Ticket::Table)
                    .drop_column(Ticket::IsEpic)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Identifier for Ticket table columns
#[derive(Iden)]
enum Ticket {
    Table,
    Id,
    IsEpic,
    EpicColor,
    ParentEpicId,
}
