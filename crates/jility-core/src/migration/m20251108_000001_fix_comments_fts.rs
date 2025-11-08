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
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ai")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_au")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ad")
                    .await?;

                // Drop old FTS table
                db.execute_unprepared("DROP TABLE IF EXISTS comments_fts")
                    .await?;

                // Recreate FTS5 table as standalone (not using content= or content_rowid=)
                // This matches the working pattern from tickets_fts
                db.execute_unprepared(
                    r#"
                    CREATE VIRTUAL TABLE comments_fts USING fts5(
                        comment_id UNINDEXED,
                        ticket_id UNINDEXED,
                        author UNINDEXED,
                        content
                    )
                    "#,
                )
                .await?;

                // Recreate triggers using comment_id instead of rowid
                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER comments_ai AFTER INSERT ON comment BEGIN
                        INSERT INTO comments_fts(comment_id, ticket_id, author, content)
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
                        WHERE comment_id = old.id;
                    END
                    "#,
                )
                .await?;

                db.execute_unprepared(
                    r#"
                    CREATE TRIGGER comments_ad AFTER DELETE ON comment BEGIN
                        DELETE FROM comments_fts WHERE comment_id = old.id;
                    END
                    "#,
                )
                .await?;

                // Rebuild FTS index with existing comments
                db.execute_unprepared(
                    r#"
                    INSERT INTO comments_fts(comment_id, ticket_id, author, content)
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
        let db = manager.get_connection();
        let db_backend = manager.get_database_backend();

        match db_backend {
            sea_orm::DatabaseBackend::Sqlite => {
                // Drop current triggers
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ai")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_au")
                    .await?;
                db.execute_unprepared("DROP TRIGGER IF EXISTS comments_ad")
                    .await?;

                // Drop FTS table
                db.execute_unprepared("DROP TABLE IF EXISTS comments_fts")
                    .await?;

                // Recreate old (broken) structure
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
            }
            _ => {}
        }

        Ok(())
    }
}
