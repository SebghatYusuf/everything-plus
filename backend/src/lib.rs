pub mod types;
pub mod everything_sdk;

use anyhow::Result;
use tracing::info;

pub use types::*;
pub use everything_sdk::EverythingSDK;

/// Main application structure - simplified to use Everything SDK directly
pub struct EverythingClone {
    pub everything_sdk: EverythingSDK,
}

impl EverythingClone {
    /// Initialize the application
    pub async fn new() -> Result<Self> {
        let mut everything_sdk = EverythingSDK::new()?;
        everything_sdk.initialize().await?;
        
        Ok(Self {
            everything_sdk,
        })
    }

    /// Perform a search query using Everything SDK
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        self.everything_sdk.search(query).await
    }

    /// Get indexing statistics from Everything
    pub async fn get_stats(&self) -> Result<IndexStats> {
        self.everything_sdk.get_stats().await
    }

    /// Everything handles indexing automatically, so we don't need manual indexing
    pub async fn start_indexing(&mut self) -> Result<()> {
        info!("Everything SDK handles indexing automatically - no manual indexing needed");
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
