pub mod types;
pub mod everything_sdk;
pub mod search;
pub mod database;

use anyhow::Result;
use std::sync::Arc;
use tracing::info;

pub use types::*;
pub use everything_sdk::EverythingSDK;
use crate::search::SearchEngine;

/// Main application structure
pub struct EverythingClone {
    pub search_engine: Arc<SearchEngine>,
    pub everything_sdk: Arc<EverythingSDK>,
}

impl EverythingClone {
    /// Initialize the application
    pub async fn new() -> Result<Self> {
        let mut sdk = EverythingSDK::new()?;
        sdk.initialize().await?;
        let sdk_arc = Arc::new(sdk);
        
        let search_engine = Arc::new(SearchEngine::new(sdk_arc.clone()));
        
        Ok(Self {
            search_engine,
            everything_sdk: sdk_arc,
        })
    }

    /// Perform a search query
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        self.search_engine.search(query).await
    }

    /// Get indexing statistics
    pub async fn get_stats(&self) -> Result<IndexStats> {
        self.everything_sdk.get_stats().await
    }

    /// Everything handles indexing automatically
    pub async fn start_indexing(&mut self) -> Result<()> {
        info!("Everything SDK handles indexing automatically.");
        Ok(())
    }
}

/// Initialize tracing for logging
pub fn init_logging() -> Result<()> {
    use tracing_subscriber::filter::EnvFilter;
    
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("everything_clone_backend=debug".parse()?)
        )
        .init();
    
    Ok(())
}
