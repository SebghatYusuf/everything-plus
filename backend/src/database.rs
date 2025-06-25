use anyhow::Result;
use sqlx::{Row, SqlitePool};
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
                attributes INTEGER NOT NULL,
                indexed_at TEXT NOT NULL
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
            (id, name, path, size, modified, created, is_directory, extension, attributes, indexed_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(entry.indexed_at.to_rfc3339())
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
                (id, name, path, size, modified, created, is_directory, extension, attributes, indexed_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
            .bind(entry.indexed_at.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        let start_time = std::time::Instant::now();
        
        let mut sql = String::from("SELECT * FROM file_entries WHERE 1=1");
        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();
        let mut param_count = 0;

        // Name/path search
        if !query.query.trim().is_empty() {
            if query.filters.use_regex {
                // For regex, we'll do a LIKE search as a fallback
                conditions.push(format!("(name LIKE ? OR path LIKE ?)"));
                let pattern = format!("%{}%", query.query);
                params.push(Box::new(pattern.clone()));
                params.push(Box::new(pattern));
                param_count += 2;
            } else if query.filters.case_sensitive {
                conditions.push(format!("(name LIKE ? OR path LIKE ?)"));
                let pattern = format!("%{}%", query.query);
                params.push(Box::new(pattern.clone()));
                params.push(Box::new(pattern));
                param_count += 2;
            } else {
                conditions.push(format!("(LOWER(name) LIKE ? OR LOWER(path) LIKE ?)"));
                let pattern = format!("%{}%", query.query.to_lowercase());
                params.push(Box::new(pattern.clone()));
                params.push(Box::new(pattern));
                param_count += 2;
            }
        }

        // File type filters
        if !query.filters.file_types.is_empty() {
            let placeholders: Vec<String> = (0..query.filters.file_types.len())
                .map(|_| "?".to_string())
                .collect();
            conditions.push(format!("extension IN ({})", placeholders.join(", ")));
            
            for file_type in &query.filters.file_types {
                params.push(Box::new(file_type.clone()));
                param_count += 1;
            }
        }

        // Size filters
        if let Some(size_min) = query.filters.size_min {
            conditions.push("size >= ?".to_string());
            params.push(Box::new(size_min));
            param_count += 1;
        }
        
        if let Some(size_max) = query.filters.size_max {
            conditions.push("size <= ?".to_string());
            params.push(Box::new(size_max));
            param_count += 1;
        }

        // Directory/file type filters
        if query.filters.directories_only {
            conditions.push("is_directory = 1".to_string());
        } else if query.filters.files_only {
            conditions.push("is_directory = 0".to_string());
        }

        // Hidden files filter
        if !query.filters.include_hidden {
            conditions.push("name NOT LIKE '.%'".to_string());
        }

        // Add conditions to SQL
        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        // Add ordering and limits
        sql.push_str(" ORDER BY is_directory DESC, name ASC");
        
        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
            
            if let Some(offset) = query.offset {
                sql.push_str(&format!(" OFFSET {}", offset));
            }
        }

        debug!("Executing search query: {}", sql);

        // Execute the query
        let mut query_builder = sqlx::query(&sql);
        for param in params {
            // This is a simplified approach - in a real implementation,
            // you'd need proper parameter binding
        }

        let rows = query_builder.fetch_all(&self.pool).await?;
        
        let mut entries = Vec::new();
        for row in rows {
            let entry = FileEntry {
                id: uuid::Uuid::parse_str(&row.get::<String, _>("id"))?,
                name: row.get("name"),
                path: row.get("path"),
                size: row.get("size"),
                modified: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("modified"))?
                    .with_timezone(&chrono::Utc),
                created: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created"))?
                    .with_timezone(&chrono::Utc),
                is_directory: row.get("is_directory"),
                extension: row.get("extension"),
                attributes: row.get("attributes"),
                indexed_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("indexed_at"))?
                    .with_timezone(&chrono::Utc),
            };
            entries.push(entry);
        }

        let query_time = start_time.elapsed().as_millis() as u64;
        
        Ok(SearchResult {
            total_count: entries.len() as u64,
            entries,
            query_time_ms: query_time,
        })
    }

    pub async fn get_stats(&self) -> Result<crate::types::IndexStats> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(CASE WHEN is_directory = 0 THEN 1 END) as file_count,
                COUNT(CASE WHEN is_directory = 1 THEN 1 END) as dir_count,
                COALESCE(SUM(CASE WHEN is_directory = 0 THEN size END), 0) as total_size,
                MAX(indexed_at) as last_updated
            FROM file_entries
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(crate::types::IndexStats {
            total_files: row.get::<i64, _>("file_count") as u64,
            total_directories: row.get::<i64, _>("dir_count") as u64,
            total_size: row.get::<i64, _>("total_size") as u64,
            last_updated: chrono::DateTime::parse_from_rfc3339(
                &row.get::<String, _>("last_updated")
            )?
            .with_timezone(&chrono::Utc),
            indexed_paths: Vec::new(), // TODO: Store and retrieve indexed paths
        })
    }

    pub async fn clear_index(&self) -> Result<()> {
        sqlx::query("DELETE FROM file_entries").execute(&self.pool).await?;
        sqlx::query("DELETE FROM file_search").execute(&self.pool).await?;
        Ok(())
    }
}
