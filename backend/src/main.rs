use anyhow::Result;
use everything_clone_backend::{EverythingClone, init_logging, SearchQuery, SearchFilters};
use tokio::signal;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init_logging()?;
    
    info!("Starting Everything Clone...");

    // Initialize the application (Everything SDK handles indexing automatically)
    let mut app = EverythingClone::new().await?;

    // Start indexing (Everything SDK handles this automatically)
    if let Err(e) = app.start_indexing().await {
        error!("Failed to start indexing: {}", e);
        return Err(e);
    }

    // Print initial stats
    match app.get_stats().await {
        Ok(stats) => {
            info!("Index stats: {} files, {} directories, {} bytes indexed", 
                  stats.total_files, stats.total_directories, stats.index_size_bytes);
        }
        Err(e) => error!("Failed to get stats: {}", e),
    }

    // Example search
    let search_query = SearchQuery {
        query: "readme".to_string(),
        filters: SearchFilters {
            case_sensitive: false,
            file_types: vec!["md".to_string(), "txt".to_string()],
            ..Default::default()
        },
        limit: Some(10),
        offset: None,
    };

    match app.search(&search_query).await {
        Ok(result) => {
            info!("Search found {} results in {}ms", 
                  result.total_count, result.query_time_ms);
            
            for entry in result.entries.iter().take(5) {
                info!("Found: {} ({})", entry.name, entry.path);
            }
        }
        Err(e) => error!("Search failed: {}", e),
    }

    // Wait for shutdown signal
    info!("Everything Clone is running. Press Ctrl+C to shutdown.");
    signal::ctrl_c().await?;
    info!("Shutdown signal received, exiting...");

    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    #[tokio::test]
    async fn test_full_indexing_and_search() -> Result<()> {
        // Create a temporary directory with test files
        let temp_dir = TempDir::new()?;
        let test_path = temp_dir.path();

        // Create test files
        fs::write(test_path.join("readme.md"), "# Test README").await?;
        fs::write(test_path.join("config.json"), r#"{"test": true}"#).await?;
        fs::create_dir(test_path.join("subdir")).await?;
        fs::write(test_path.join("subdir").join("test.txt"), "test content").await?;

        // Initialize app with custom database
        let db_path = temp_dir.path().join("test.db");
        let mut app = EverythingClone::with_database_path(
            db_path.to_str().unwrap()
        ).await?;

        // Add only our test directory
        app.add_indexed_path(test_path.to_path_buf());

        // Start indexing
        app.start_indexing().await?;

        // Give some time for indexing to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Test search
        let query = SearchQuery {
            query: "readme".to_string(),
            filters: SearchFilters::default(),
            limit: Some(100),
            offset: None,
        };

        let result = app.search(&query).await?;
        assert!(result.total_count > 0, "Should find at least one readme file");

        // Test stats
        let stats = app.get_stats().await?;
        assert!(stats.total_files > 0, "Should have indexed some files");

        Ok(())
    }
}
