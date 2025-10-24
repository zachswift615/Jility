use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::FullName).string())
                    .col(ColumnDef::new(User::AvatarUrl).string())
                    .col(
                        ColumnDef::new(User::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(User::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::LastLoginAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Create indexes on users table
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_username")
                    .table(User::Table)
                    .col(User::Username)
                    .to_owned(),
            )
            .await?;

        // Create api_keys table
        manager
            .create_table(
                Table::create()
                    .table(ApiKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiKey::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKey::UserId).uuid().not_null())
                    .col(ColumnDef::new(ApiKey::Name).string().not_null())
                    .col(ColumnDef::new(ApiKey::KeyHash).string().not_null())
                    .col(ColumnDef::new(ApiKey::Prefix).string().not_null())
                    .col(ColumnDef::new(ApiKey::Scopes).text().not_null())
                    .col(ColumnDef::new(ApiKey::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ApiKey::LastUsedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ApiKey::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ApiKey::RevokedAt).timestamp_with_time_zone())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_api_keys_user")
                            .from(ApiKey::Table, ApiKey::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes on api_keys table
        manager
            .create_index(
                Index::create()
                    .name("idx_api_keys_user_id")
                    .table(ApiKey::Table)
                    .col(ApiKey::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_api_keys_prefix")
                    .table(ApiKey::Table)
                    .col(ApiKey::Prefix)
                    .to_owned(),
            )
            .await?;

        // Create sessions table
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserId).uuid().not_null())
                    .col(ColumnDef::new(Session::TokenHash).string().not_null())
                    .col(ColumnDef::new(Session::IpAddress).string())
                    .col(ColumnDef::new(Session::UserAgent).string())
                    .col(
                        ColumnDef::new(Session::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Session::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Session::RevokedAt).timestamp_with_time_zone())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_sessions_user")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes on sessions table
        manager
            .create_index(
                Index::create()
                    .name("idx_sessions_user_id")
                    .table(Session::Table)
                    .col(Session::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sessions_token_hash")
                    .table(Session::Table)
                    .col(Session::TokenHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order (to handle foreign keys)
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ApiKey::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Identifier for User table
#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    Username,
    PasswordHash,
    FullName,
    AvatarUrl,
    IsActive,
    IsVerified,
    CreatedAt,
    UpdatedAt,
    LastLoginAt,
}

/// Identifier for ApiKey table
#[derive(Iden)]
enum ApiKey {
    Table,
    Id,
    UserId,
    Name,
    KeyHash,
    Prefix,
    Scopes,
    ExpiresAt,
    LastUsedAt,
    CreatedAt,
    RevokedAt,
}

/// Identifier for Session table
#[derive(Iden)]
enum Session {
    Table,
    Id,
    UserId,
    TokenHash,
    IpAddress,
    UserAgent,
    CreatedAt,
    ExpiresAt,
    RevokedAt,
}
