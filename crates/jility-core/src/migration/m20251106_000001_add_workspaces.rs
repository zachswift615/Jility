use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create workspace table
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Workspace::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Workspace::Name).string().not_null())
                    .col(ColumnDef::new(Workspace::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Workspace::CreatedByUserId).uuid().not_null())
                    .col(ColumnDef::new(Workspace::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Workspace::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Workspace::Table, Workspace::CreatedByUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create workspace_member table
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceMember::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkspaceMember::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkspaceMember::WorkspaceId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceMember::UserId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceMember::Role).string_len(50).not_null())
                    .col(ColumnDef::new(WorkspaceMember::InvitedByUserId).uuid())
                    .col(ColumnDef::new(WorkspaceMember::InvitedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkspaceMember::JoinedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceMember::Table, WorkspaceMember::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceMember::Table, WorkspaceMember::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint on workspace_id + user_id
        manager
            .create_index(
                Index::create()
                    .table(WorkspaceMember::Table)
                    .name("idx_workspace_member_unique")
                    .col(WorkspaceMember::WorkspaceId)
                    .col(WorkspaceMember::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create workspace_invite table
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceInvite::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkspaceInvite::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkspaceInvite::WorkspaceId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::Email).string().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::Role).string_len(50).not_null())
                    .col(ColumnDef::new(WorkspaceInvite::InvitedByUserId).uuid().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(WorkspaceInvite::ExpiresAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(WorkspaceInvite::AcceptedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkspaceInvite::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceInvite::Table, WorkspaceInvite::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkspaceInvite::Table, WorkspaceInvite::InvitedByUserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Add workspace_id to project table
        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .add_column(ColumnDef::new(Project::WorkspaceId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        // Add foreign key for project.workspace_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Project::Table, Project::WorkspaceId)
                    .to(Workspace::Table, Workspace::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Create indexes for performance
        manager
            .create_index(
                Index::create()
                    .table(WorkspaceMember::Table)
                    .name("idx_workspace_member_user")
                    .col(WorkspaceMember::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Project::Table)
                    .name("idx_project_workspace")
                    .col(Project::WorkspaceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkspaceInvite::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WorkspaceMember::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Project::Table)
                    .drop_column(Project::WorkspaceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
    Name,
    Slug,
    CreatedByUserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WorkspaceMember {
    Table,
    Id,
    WorkspaceId,
    UserId,
    Role,
    InvitedByUserId,
    InvitedAt,
    JoinedAt,
}

#[derive(Iden)]
enum WorkspaceInvite {
    Table,
    Id,
    WorkspaceId,
    Email,
    Role,
    InvitedByUserId,
    Token,
    ExpiresAt,
    AcceptedAt,
    CreatedAt,
}

#[derive(Iden)]
enum Project {
    Table,
    WorkspaceId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
