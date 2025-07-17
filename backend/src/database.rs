use anyhow::Result;
use chrono::Utc;
use sqlx::{Row, Sqlite, SqlitePool};
use sqlx::query::Query;
use std::path::Path;
use tracing::{debug, info, warn};

use crate::types::{FileEntry, SearchError, SearchFilters, SearchQuery, SearchResult};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: &str) -> Result<Self> {
        // Create database directory if it doesn't exist
        if let Some(parent) = Path::new(database_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let database_url = format!("sqlite:{}", database_path);
        let pool = SqlitePool::connect(&database_url).await?;

        let db = Self { pool };
        db.initialize_schema().await?;
        db.optimize_database().await?;

        Ok(db)
    }

    async fn initialize_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS file_entries (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                size INTEGER NOT NULL,
                modified TEXT NOT NULL,
                created TEXT NOT NULL,
                is_directory BOOLEAN NOT NULL,
                extension TEXT,
                attributes INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_name ON file_entries(name);
            CREATE INDEX IF NOT EXISTS idx_path ON file_entries(path);
            CREATE INDEX IF NOT EXISTS idx_extension ON file_entries(extension);
            CREATE INDEX IF NOT EXISTS idx_size ON file_entries(size);
            CREATE INDEX IF NOT EXISTS idx_modified ON file_entries(modified);
            CREATE INDEX IF NOT EXISTS idx_is_directory ON file_entries(is_directory);
            CREATE INDEX IF NOT EXISTS idx_name_lower ON file_entries(LOWER(name));
            
            -- Full-text search virtual table for content search
            CREATE VIRTUAL TABLE IF NOT EXISTS file_search USING fts5(
                name, path, content='file_entries', content_rowid='rowid'
            );

            -- Triggers to keep FTS table in sync
            CREATE TRIGGER IF NOT EXISTS file_entries_ai AFTER INSERT ON file_entries BEGIN
                INSERT INTO file_search(rowid, name, path) VALUES (NEW.rowid, NEW.name, NEW.path);
            END;

            CREATE TRIGGER IF NOT EXISTS file_entries_ad AFTER DELETE ON file_entries BEGIN
                DELETE FROM file_search WHERE rowid = OLD.rowid;
            END;

            CREATE TRIGGER IF NOT EXISTS file_entries_au AFTER UPDATE ON file_entries BEGIN
                DELETE FROM file_search WHERE rowid = OLD.rowid;
                INSERT INTO file_search(rowid, name, path) VALUES (NEW.rowid, NEW.name, NEW.path);
            END;
            "#,
        )
        .execute(&self.pool)
        .await?;

        info!("Database schema initialized");
        Ok(())
    }

    async fn optimize_database(&self) -> Result<()> {
        // SQLite optimization settings
        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("PRAGMA synchronous = NORMAL")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("PRAGMA cache_size = 10000")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("PRAGMA temp_store = memory")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("PRAGMA mmap_size = 268435456") // 256MB
            .execute(&self.pool)
            .await?;

        debug!("Database optimization settings applied");
        Ok(())
    }

    pub async fn insert_file_entry(&self, entry: &FileEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO file_entries 
            (id, name, path, size, modified, created, is_directory, extension, attributes)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(entry.id.to_string())
        .bind(&entry.name)
        .bind(&entry.path)
        .bind(entry.size)
        .bind(entry.modified.to_rfc3339())
        .bind(entry.created.to_rfc3339())
        .bind(entry.is_directory)
        .bind(&entry.extension)
        .bind(entry.attributes)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn batch_insert_file_entries(&self, entries: &[FileEntry]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for entry in entries {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO file_entries 
                (id, name, path, size, modified, created, is_directory, extension, attributes)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(entry.id.to_string())
            .bind(&entry.name)
            .bind(&entry.path)
            .bind(entry.size)
            .bind(entry.modified.to_rfc3339())
            .bind(entry.created.to_rfc3339())
            .bind(entry.is_directory)
            .bind(&entry.extension)
            .bind(entry.attributes)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    

    pub async fn get_stats(&self) -> Result<crate::types::IndexStats> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(CASE WHEN is_directory = 0 THEN 1 END) as file_count,
                COUNT(CASE WHEN is_directory = 1 THEN 1 END) as dir_count,
                COALESCE(SUM(CASE WHEN is_directory = 0 THEN size END), 0) as total_size
            FROM file_entries
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(crate::types::IndexStats {
            total_files: row.get::<i64, _>("file_count") as u64,
            total_directories: row.get::<i64, _>("dir_count") as u64,
            index_size_bytes: row.get::<i64, _>("total_size") as u64,
            last_index_time: Utc::now(), // This should be updated to reflect actual index time
            indexed_paths: Vec::new(), // TODO: Store and retrieve indexed paths
        })
    }

    pub async fn clear_index(&self) -> Result<()> {
        sqlx::query("DELETE FROM file_entries").execute(&self.pool).await?;
        sqlx::query("DELETE FROM file_search").execute(&self.pool).await?;
        Ok(())
    }
}
