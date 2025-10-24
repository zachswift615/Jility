use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let db_backend = manager.get_database_backend();

        match db_backend {
            sea_orm::DatabaseBackend::Sqlite => {
                // Create FTS5 virtual table for tickets
                db.execute_unprepared(
                    r#"
                    CREATE VIRTUAL TABLE IF NOT EXISTS tickets_fts USING fts5(
                        ticket_id UNINDEXED,
                        ticket_number UNINDEXED,
                        title,
                        description,
                        content=ticket,
                        content_rowid=id,
                        tokenize='porter unicode61'
                    )
                    "#,
                )
                .await?;

                // Create FTS5 virtual table for comments
                db.execute_unprepared(
                    r#"
                    CREATE VIRTUAL TABLE IF NOT EXISTS comments_fts USING fts5(
                        comment_id UNINDEXED,
                        ticket_id UNINDEXED,
                        author UNINDEXED,
                        content,
                        content=comment,
                        content_rowid=id,
                        tokenize='porter unicode61'
                    )
                    "#,
                )
                .await?;

                // Populate initial FTS data for tickets (only if ticket table exists and has data)
                // Use INSERT OR IGNORE to handle any errors gracefully
                let _ = db.execute_unprepared(
                    r#"
                    INSERT OR IGNORE INTO tickets_fts(ticket_id, ticket_number, title, description)
                    SELECT id, ticket_number, title, description FROM ticket
                    "#,
                )
                .await; // Ignore errors since table might be empty

                // Populate initial FTS data for comments (only if comment table exists and has data)
                let _ = db.execute_unprepared(
                    r#"
                    INSERT OR IGNORE INTO comments_fts(comment_id, ticket_id, author, content)
                    SELECT id, ticket_id, author, content FROM comment
                    "#,
                )
                .await; // Ignore errors since table might be empty

                // Triggers to keep tickets FTS in sync
                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS tickets_ai AFTER INSERT ON ticket BEGIN
                        INSERT INTO tickets_fts(ticket_id, ticket_number, title, description)
                        VALUES (new.id, new.ticket_number, new.title, new.description);
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS tickets_au AFTER UPDATE ON ticket BEGIN
                        UPDATE tickets_fts
                        SET title = new.title, description = new.description
                        WHERE ticket_id = old.id;
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS tickets_ad AFTER DELETE ON ticket BEGIN
                        DELETE FROM tickets_fts WHERE ticket_id = old.id;
                    END
                    "#,
                )
                .await?;

                // Triggers to keep comments FTS in sync
                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS comments_ai AFTER INSERT ON comment BEGIN
                        INSERT INTO comments_fts(comment_id, ticket_id, author, content)
                        VALUES (new.id, new.ticket_id, new.author, new.content);
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS comments_au AFTER UPDATE ON comment BEGIN
                        UPDATE comments_fts
                        SET content = new.content
                        WHERE comment_id = old.id;
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS comments_ad AFTER DELETE ON comment BEGIN
                        DELETE FROM comments_fts WHERE comment_id = old.id;
                    END
                    "#,
                )
                .await?;
            }
            sea_orm::DatabaseBackend::Postgres => {
                // Add tsvector column to ticket table
                db.execute_unprepared(
                    "ALTER TABLE ticket ADD COLUMN IF NOT EXISTS search_vector tsvector",
                )
                .await?;

                // Create GIN index for fast full-text search
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_tickets_search ON ticket USING GIN(search_vector)",
                )
                .await?;

                // Create function to update search vector
                db.execute_unprepared(
                    r#"
                    CREATE OR REPLACE FUNCTION tickets_search_trigger() RETURNS trigger AS $$
                    BEGIN
                        NEW.search_vector :=
                            setweight(to_tsvector('english', coalesce(NEW.title, '')), 'A') ||
                            setweight(to_tsvector('english', coalesce(NEW.description, '')), 'B');
                        RETURN NEW;
                    END
                    $$ LANGUAGE plpgsql
                    "#,
                )
                .await?;

                // Create trigger to auto-update search vector
                db.execute_unprepared(
                    r#"
                    DROP TRIGGER IF EXISTS tickets_search_update ON ticket;
                    CREATE TRIGGER tickets_search_update
                        BEFORE INSERT OR UPDATE ON ticket
                        FOR EACH ROW
                        EXECUTE FUNCTION tickets_search_trigger()
                    "#,
                )
                .await?;

                // Update existing rows
                db.execute_unprepared(
                    r#"
                    UPDATE ticket SET search_vector =
                        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
                        setweight(to_tsvector('english', coalesce(description, '')), 'B')
                    WHERE search_vector IS NULL
                    "#,
                )
                .await?;

                // Add tsvector column to comment table
                db.execute_unprepared(
                    "ALTER TABLE comment ADD COLUMN IF NOT EXISTS search_vector tsvector",
                )
                .await?;

                // Create GIN index for comments
                db.execute_unprepared(
                    "CREATE INDEX IF NOT EXISTS idx_comments_search ON comment USING GIN(search_vector)",
                )
                .await?;

                // Create function for comments search vector
                db.execute_unprepared(
                    r#"
                    CREATE OR REPLACE FUNCTION comments_search_trigger() RETURNS trigger AS $$
                    BEGIN
                        NEW.search_vector := to_tsvector('english', coalesce(NEW.content, ''));
                        RETURN NEW;
                    END
                    $$ LANGUAGE plpgsql
                    "#,
                )
                .await?;

                // Create trigger for comments
                db.execute_unprepared(
                    r#"
                    DROP TRIGGER IF EXISTS comments_search_update ON comment;
                    CREATE TRIGGER comments_search_update
                        BEFORE INSERT OR UPDATE ON comment
                        FOR EACH ROW
                        EXECUTE FUNCTION comments_search_trigger()
                    "#,
                )
                .await?;

                // Update existing comments
                db.execute_unprepared(
                    "UPDATE comment SET search_vector = to_tsvector('english', coalesce(content, '')) WHERE search_vector IS NULL",
                )
                .await?;
            }
            _ => {
                return Err(DbErr::Custom(
                    "Unsupported database backend for FTS".to_string(),
                ))
            }
        }

        // Create saved_views table (database-agnostic)
        manager
            .create_table(
                Table::create()
                    .table(SavedView::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SavedView::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SavedView::UserId).string().not_null())
                    .col(ColumnDef::new(SavedView::Name).string().not_null())
                    .col(ColumnDef::new(SavedView::Description).text())
                    .col(ColumnDef::new(SavedView::Filters).text().not_null())
                    .col(
                        ColumnDef::new(SavedView::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SavedView::IsShared)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SavedView::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SavedView::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on user_id for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_saved_views_user_id")
                    .table(SavedView::Table)
                    .col(SavedView::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let db_backend = manager.get_database_backend();

        match db_backend {
            sea_orm::DatabaseBackend::Sqlite => {
                // Drop triggers
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_ai")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_au")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_ad")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ai")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_au")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ad")
                    .await?;

                // Drop FTS tables
                db.execute_unprepared("DROP TABLE IF EXISTS tickets_fts")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS comments_fts")
                    .await?;
            }
            sea_orm::DatabaseBackend::Postgres => {
                // Drop triggers
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_search_update ON ticket")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_search_update ON comment")
                    .await?;

                // Drop functions
                db.execute_unprepared("DROP FUNCTION IF EXISTS tickets_search_trigger()")
                    .await?;
                db.execute_unprepared("DROP FUNCTION IF EXISTS comments_search_trigger()")
                    .await?;

                // Drop indexes
                db.execute_unprepared("DROP INDEX IF EXISTS idx_tickets_search")
                    .await?;
                db.execute_unprepared("DROP INDEX IF EXISTS idx_comments_search")
                    .await?;

                // Drop columns
                db.execute_unprepared("ALTER TABLE ticket DROP COLUMN IF EXISTS search_vector")
                    .await?;
                db.execute_unprepared("ALTER TABLE comment DROP COLUMN IF EXISTS search_vector")
                    .await?;
            }
            _ => {}
        }

        // Drop saved_views table
        manager
            .drop_table(Table::drop().table(SavedView::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum SavedView {
    Table,
    Id,
    UserId,
    Name,
    Description,
    Filters,
    IsDefault,
    IsShared,
    CreatedAt,
    UpdatedAt,
}
