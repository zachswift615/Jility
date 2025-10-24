use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add new columns to project table (one at a time for SQLite compatibility)
        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(ColumnDef::new(Project::Key).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(
                        ColumnDef::new(Project::Color)
                            .string()
                            .null()
                            .default("#5e6ad2"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(
                        ColumnDef::new(Project::AiPlanningEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(
                        ColumnDef::new(Project::AutoLinkGit)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(
                        ColumnDef::new(Project::RequireStoryPoints)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on project key for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_key")
                    .table(Project::Table)
                    .col(Project::Key)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the index first
        manager
            .drop_index(Index::drop().name("idx_projects_key").to_owned())
            .await?;

        // Remove columns
        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .drop_column(Project::Key)
                    .drop_column(Project::Color)
                    .drop_column(Project::AiPlanningEnabled)
                    .drop_column(Project::AutoLinkGit)
                    .drop_column(Project::RequireStoryPoints)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Identifier for Project table
#[derive(Iden)]
enum Project {
    Table,
    Key,
    Color,
    AiPlanningEnabled,
    AutoLinkGit,
    RequireStoryPoints,
}
