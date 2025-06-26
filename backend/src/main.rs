use anyhow::Result;
use everything_clone_backend::{EverythingClone, init_logging, SearchQuery, SearchFilters};
use tokio::signal;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init_logging()?;
    
    info!("Starting Everything Plus...");

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
    info!("Everything Plus is running. Press Ctrl+C to shutdown.");
    signal::ctrl_c().await?;
    info!("Shutdown signal received, exiting...");

    Ok(())
}