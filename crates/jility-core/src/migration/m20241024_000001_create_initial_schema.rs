use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create projects table
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Name).string().not_null())
                    .col(ColumnDef::new(Project::Description).text())
                    .col(
                        ColumnDef::new(Project::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on project name
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_name")
                    .table(Project::Table)
                    .col(Project::Name)
                    .to_owned(),
            )
            .await?;

        // Create tickets table
        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ticket::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ticket::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(Ticket::TicketNumber).integer().not_null())
                    .col(ColumnDef::new(Ticket::Title).string().not_null())
                    .col(
                        ColumnDef::new(Ticket::Description)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Ticket::Status)
                            .string()
                            .not_null()
                            .default("backlog")
                            .check(
                                Expr::col(Ticket::Status).is_in([
                                    "backlog",
                                    "todo",
                                    "in_progress",
                                    "review",
                                    "done",
                                    "blocked",
                                ]),
                            ),
                    )
                    .col(ColumnDef::new(Ticket::StoryPoints).integer())
                    .col(ColumnDef::new(Ticket::EpicId).uuid())
                    .col(ColumnDef::new(Ticket::ParentId).uuid())
                    .col(
                        ColumnDef::new(Ticket::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ticket::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Ticket::CreatedBy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_project")
                            .from(Ticket::Table, Ticket::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_epic")
                            .from(Ticket::Table, Ticket::EpicId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_parent")
                            .from(Ticket::Table, Ticket::ParentId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on project_id + ticket_number
        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_number")
                    .table(Ticket::Table)
                    .col(Ticket::ProjectId)
                    .col(Ticket::TicketNumber)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create indexes on tickets
        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_project_id")
                    .table(Ticket::Table)
                    .col(Ticket::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_status")
                    .table(Ticket::Table)
                    .col(Ticket::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_created_by")
                    .table(Ticket::Table)
                    .col(Ticket::CreatedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_epic_id")
                    .table(Ticket::Table)
                    .col(Ticket::EpicId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tickets_parent_id")
                    .table(Ticket::Table)
                    .col(Ticket::ParentId)
                    .to_owned(),
            )
            .await?;

        // Create ticket_assignees table
        manager
            .create_table(
                Table::create()
                    .table(TicketAssignee::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketAssignee::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TicketAssignee::TicketId).uuid().not_null())
                    .col(ColumnDef::new(TicketAssignee::Assignee).string().not_null())
                    .col(
                        ColumnDef::new(TicketAssignee::AssignedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketAssignee::AssignedBy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_assignee_ticket")
                            .from(TicketAssignee::Table, TicketAssignee::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on ticket_id + assignee
        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_assignees_unique")
                    .table(TicketAssignee::Table)
                    .col(TicketAssignee::TicketId)
                    .col(TicketAssignee::Assignee)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_assignees_ticket_id")
                    .table(TicketAssignee::Table)
                    .col(TicketAssignee::TicketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_assignees_assignee")
                    .table(TicketAssignee::Table)
                    .col(TicketAssignee::Assignee)
                    .to_owned(),
            )
            .await?;

        // Create ticket_labels table
        manager
            .create_table(
                Table::create()
                    .table(TicketLabel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketLabel::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TicketLabel::TicketId).uuid().not_null())
                    .col(ColumnDef::new(TicketLabel::Label).string().not_null())
                    .col(
                        ColumnDef::new(TicketLabel::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_label_ticket")
                            .from(TicketLabel::Table, TicketLabel::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on ticket_id + label
        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_labels_unique")
                    .table(TicketLabel::Table)
                    .col(TicketLabel::TicketId)
                    .col(TicketLabel::Label)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_labels_ticket_id")
                    .table(TicketLabel::Table)
                    .col(TicketLabel::TicketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_labels_label")
                    .table(TicketLabel::Table)
                    .col(TicketLabel::Label)
                    .to_owned(),
            )
            .await?;

        // Create ticket_dependencies table
        manager
            .create_table(
                Table::create()
                    .table(TicketDependency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketDependency::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TicketDependency::TicketId).uuid().not_null())
                    .col(ColumnDef::new(TicketDependency::DependsOnId).uuid().not_null())
                    .col(
                        ColumnDef::new(TicketDependency::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketDependency::CreatedBy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_dependency_ticket")
                            .from(TicketDependency::Table, TicketDependency::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_dependency_depends_on")
                            .from(TicketDependency::Table, TicketDependency::DependsOnId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(TicketDependency::TicketId)
                            .ne(Expr::col(TicketDependency::DependsOnId)),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on ticket_id + depends_on_id
        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_dependencies_unique")
                    .table(TicketDependency::Table)
                    .col(TicketDependency::TicketId)
                    .col(TicketDependency::DependsOnId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_dependencies_ticket_id")
                    .table(TicketDependency::Table)
                    .col(TicketDependency::TicketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_dependencies_depends_on_id")
                    .table(TicketDependency::Table)
                    .col(TicketDependency::DependsOnId)
                    .to_owned(),
            )
            .await?;

        // Create comments table
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::TicketId).uuid().not_null())
                    .col(ColumnDef::new(Comment::Author).string().not_null())
                    .col(ColumnDef::new(Comment::Content).text().not_null())
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comment::UpdatedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_ticket")
                            .from(Comment::Table, Comment::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_ticket_id")
                    .table(Comment::Table)
                    .col(Comment::TicketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_author")
                    .table(Comment::Table)
                    .col(Comment::Author)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_created_at")
                    .table(Comment::Table)
                    .col(Comment::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create commit_links table
        manager
            .create_table(
                Table::create()
                    .table(CommitLink::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommitLink::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CommitLink::TicketId).uuid().not_null())
                    .col(ColumnDef::new(CommitLink::CommitHash).string().not_null())
                    .col(ColumnDef::new(CommitLink::CommitMessage).text())
                    .col(
                        ColumnDef::new(CommitLink::LinkedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CommitLink::LinkedBy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commit_link_ticket")
                            .from(CommitLink::Table, CommitLink::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on ticket_id + commit_hash
        manager
            .create_index(
                Index::create()
                    .name("idx_commit_links_unique")
                    .table(CommitLink::Table)
                    .col(CommitLink::TicketId)
                    .col(CommitLink::CommitHash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commit_links_ticket_id")
                    .table(CommitLink::Table)
                    .col(CommitLink::TicketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commit_links_commit_hash")
                    .table(CommitLink::Table)
                    .col(CommitLink::CommitHash)
                    .to_owned(),
            )
            .await?;

        // Create sprints table
        manager
            .create_table(
                Table::create()
                    .table(Sprint::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sprint::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sprint::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(Sprint::Name).string().not_null())
                    .col(ColumnDef::new(Sprint::Goal).text())
                    .col(ColumnDef::new(Sprint::StartDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(Sprint::EndDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Sprint::Status)
                            .string()
                            .not_null()
                            .default("planning")
                            .check(
                                Expr::col(Sprint::Status).is_in(["planning", "active", "completed"]),
                            ),
                    )
                    .col(
                        ColumnDef::new(Sprint::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Sprint::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sprint_project")
                            .from(Sprint::Table, Sprint::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sprints_project_id")
                    .table(Sprint::Table)
                    .col(Sprint::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sprints_status")
                    .table(Sprint::Table)
                    .col(Sprint::Status)
                    .to_owned(),
            )
            .await?;

        // Create sprint_tickets table
        manager
            .create_table(
                Table::create()
                    .table(SprintTicket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SprintTicket::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SprintTicket::SprintId).uuid().not_null())
                    .col(ColumnDef::new(SprintTicket::TicketId).uuid().not_null())
                    .col(
                        ColumnDef::new(SprintTicket::AddedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SprintTicket::AddedBy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sprint_ticket_sprint")
                            .from(SprintTicket::Table, SprintTicket::SprintId)
                            .to(Sprint::Table, Sprint::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sprint_ticket_ticket")
                            .from(SprintTicket::Table, SprintTicket::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on sprint_id + ticket_id
        manager
            .create_index(
                Index::create()
                    .name("idx_sprint_tickets_unique")
                    .table(SprintTicket::Table)
                    .col(SprintTicket::SprintId)
                    .col(SprintTicket::TicketId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sprint_tickets_sprint_id")
                    .table(SprintTicket::Table)
                    .col(SprintTicket::SprintId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sprint_tickets_ticket_id")
                    .table(SprintTicket::Table)
                    .col(SprintTicket::TicketId)
                    .to_owned(),
            )
            .await?;

        // Create ticket_changes table (event sourcing)
        manager
            .create_table(
                Table::create()
                    .table(TicketChange::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketChange::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TicketChange::TicketId).uuid().not_null())
                    .col(ColumnDef::new(TicketChange::ChangeType).string().not_null())
                    .col(ColumnDef::new(TicketChange::FieldName).string())
                    .col(ColumnDef::new(TicketChange::OldValue).text())
                    .col(ColumnDef::new(TicketChange::NewValue).text())
                    .col(ColumnDef::new(TicketChange::ChangedBy).string().not_null())
                    .col(
                        ColumnDef::new(TicketChange::ChangedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketChange::Message).text())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_change_ticket")
                            .from(TicketChange::Table, TicketChange::TicketId)
                            .to(Ticket::Table, Ticket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_changes_ticket_id")
                    .table(TicketChange::Table)
                    .col(TicketChange::TicketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_changes_changed_at")
                    .table(TicketChange::Table)
                    .col(TicketChange::ChangedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_changes_changed_by")
                    .table(TicketChange::Table)
                    .col(TicketChange::ChangedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ticket_changes_change_type")
                    .table(TicketChange::Table)
                    .col(TicketChange::ChangeType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order (respecting foreign keys)
        manager
            .drop_table(Table::drop().table(TicketChange::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SprintTicket::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Sprint::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CommitLink::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TicketDependency::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TicketLabel::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TicketAssignee::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Ticket::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Table identifiers for migration
#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Ticket {
    Table,
    Id,
    ProjectId,
    TicketNumber,
    Title,
    Description,
    Status,
    StoryPoints,
    EpicId,
    ParentId,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
}

#[derive(DeriveIden)]
enum TicketAssignee {
    Table,
    Id,
    TicketId,
    Assignee,
    AssignedAt,
    AssignedBy,
}

#[derive(DeriveIden)]
enum TicketLabel {
    Table,
    Id,
    TicketId,
    Label,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TicketDependency {
    Table,
    Id,
    TicketId,
    DependsOnId,
    CreatedAt,
    CreatedBy,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    TicketId,
    Author,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CommitLink {
    Table,
    Id,
    TicketId,
    CommitHash,
    CommitMessage,
    LinkedAt,
    LinkedBy,
}

#[derive(DeriveIden)]
enum Sprint {
    Table,
    Id,
    ProjectId,
    Name,
    Goal,
    StartDate,
    EndDate,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum SprintTicket {
    Table,
    Id,
    SprintId,
    TicketId,
    AddedAt,
    AddedBy,
}

#[derive(DeriveIden)]
enum TicketChange {
    Table,
    Id,
    TicketId,
    ChangeType,
    FieldName,
    OldValue,
    NewValue,
    ChangedBy,
    ChangedAt,
    Message,
}
