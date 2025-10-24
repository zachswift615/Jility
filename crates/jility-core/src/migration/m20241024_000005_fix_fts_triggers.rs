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
                // Drop old triggers
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_ai")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_au")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS tickets_ad")
                    .await?;

                // Drop old FTS table
                db.execute_unprepared("DROP TABLE IF EXISTS tickets_fts")
                    .await?;

                // Recreate FTS5 table as contentless (can't use content= with UUID rowids)
                db.execute_unprepared(
                    r#"
                    CREATE VIRTUAL TABLE tickets_fts USING fts5(
                        ticket_id UNINDEXED,
                        ticket_number UNINDEXED,
                        title,
                        description
                    )
                    "#,
                )
                .await?;

                // Recreate triggers using ticket_id
                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER tickets_ai AFTER INSERT ON ticket BEGIN
                        INSERT INTO tickets_fts(ticket_id, ticket_number, title, description)
                        VALUES (new.id, new.ticket_number, new.title, new.description);
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER tickets_au AFTER UPDATE ON ticket BEGIN
                        UPDATE tickets_fts
                        SET ticket_number = new.ticket_number, title = new.title, description = new.description
                        WHERE ticket_id = old.id;
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER tickets_ad AFTER DELETE ON ticket BEGIN
                        DELETE FROM tickets_fts WHERE ticket_id = old.id;
                    END
                    "#,
                )
                .await?;

                // Rebuild FTS index with existing tickets
                db.execute_unprepared(
                    r#"
                    INSERT INTO tickets_fts(ticket_id, ticket_number, title, description)
                    SELECT id, ticket_number, title, description FROM ticket
                    "#,
                )
                .await?;

                // Same fix for comments_fts
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ai")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_au")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ad")
                    .await?;

                db.execute_unprepared("DROP TABLE IF EXISTS comments_fts")
                    .await?;

                db.execute_unprepared(
                    r#"
                    CREATE VIRTUAL TABLE comments_fts USING fts5(
                        ticket_id UNINDEXED,
                        author UNINDEXED,
                        content,
                        content=comment,
                        content_rowid=id
                    )
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER comments_ai AFTER INSERT ON comment BEGIN
                        INSERT INTO comments_fts(rowid, ticket_id, author, content)
                        VALUES (new.id, new.ticket_id, new.author, new.content);
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER comments_au AFTER UPDATE ON comment BEGIN
                        UPDATE comments_fts
                        SET ticket_id = new.ticket_id, author = new.author, content = new.content
                        WHERE rowid = old.id;
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER comments_ad AFTER DELETE ON comment BEGIN
                        DELETE FROM comments_fts WHERE rowid = old.id;
                    END
                    "#,
                )
                .await?;

                // Rebuild comments FTS index
                db.execute_unprepared(
                    r#"
                    INSERT INTO comments_fts(rowid, ticket_id, author, content)
                    SELECT id, ticket_id, author, content FROM comment
                    "#,
                )
                .await?;
            }
            _ => {
                // PostgreSQL doesn't need this fix - it uses tsvector which works correctly
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert to the old (broken) structure
        let db = manager.get_connection();
        let db_backend = manager.get_database_backend();

        match db_backend {
            sea_orm::DatabaseBackend::Sqlite => {
                // Drop current triggers
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

                // Recreate old structure (with bug)
                db.execute_unprepared(
                    r#"
                    CREATE VIRTUAL TABLE tickets_fts USING fts5(
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

                // Old (broken) triggers
                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER tickets_ai AFTER INSERT ON ticket BEGIN
                        INSERT INTO tickets_fts(ticket_id, ticket_number, title, description)
                        VALUES (new.id, new.ticket_number, new.title, new.description);
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER tickets_au AFTER UPDATE ON ticket BEGIN
                        UPDATE tickets_fts
                        SET title = new.title, description = new.description
                        WHERE ticket_id = old.id;
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER tickets_ad AFTER DELETE ON ticket BEGIN
                        DELETE FROM tickets_fts WHERE ticket_id = old.id;
                    END
                    "#,
                )
                .await?;

                // Rebuild old index
                db.execute_unprepared(
                    r#"
                    INSERT OR IGNORE INTO tickets_fts(ticket_id, ticket_number, title, description)
                    SELECT id, ticket_number, title, description FROM ticket
                    "#,
                )
                .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
