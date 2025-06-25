use anyhow::Result;
use regex::Regex;
use std::sync::Arc;
use std::time::Instant;
use tracing::debug;

use crate::database::Database;
use crate::types::{SearchQuery, SearchResult, SearchError};

pub struct SearchEngine {
    db: Arc<Database>,
}

impl SearchEngine {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        let start_time = Instant::now();
        
        // Validate and preprocess query
        let processed_query = self.preprocess_query(query)?;
        
        // Execute search
        let result = self.db.search(&processed_query).await?;
        
        let query_time = start_time.elapsed().as_millis() as u64;
        debug!("Search completed in {}ms", query_time);
        
        Ok(SearchResult {
            entries: result.entries,
            total_count: result.total_count,
            query_time_ms: query_time,
        })
    }

    fn preprocess_query(&self, query: &SearchQuery) -> Result<SearchQuery> {
        let mut processed = query.clone();

        // Validate regex if enabled
        if query.filters.use_regex {
            if let Err(e) = Regex::new(&query.query) {
                return Err(SearchError::InvalidQuery(
                    format!("Invalid regex pattern: {}", e)
                ).into());
            }
        }

        // Clean up query string
        processed.query = query.query.trim().to_string();

        // Apply default limits if not specified
        if processed.limit.is_none() {
            processed.limit = Some(1000); // Default limit
        }

        Ok(processed)
    }

    pub async fn suggest_completions(&self, partial_query: &str) -> Result<Vec<String>> {
        // TODO: Implement query completion suggestions
        // This could use the FTS table to find common prefixes
        
        let suggestions = vec![
            format!("{}*.txt", partial_query),
            format!("{}*.pdf", partial_query),
            format!("{}*.doc", partial_query),
        ];

        Ok(suggestions)
    }

    pub async fn get_recent_searches(&self) -> Result<Vec<String>> {
        // TODO: Implement recent search history
        Ok(Vec::new())
    }

    pub async fn save_search_history(&self, query: &str) -> Result<()> {
        // TODO: Implement search history persistence
        debug!("Saving search to history: {}", query);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SearchFilters;

    #[test]
    fn test_regex_validation() {
        let db = Arc::new(Database::new(":memory:").await.unwrap());
        let engine = SearchEngine::new(db);

        let valid_query = SearchQuery {
            query: r"test\d+".to_string(),
            filters: SearchFilters {
                use_regex: true,
                ..Default::default()
            },
            limit: None,
            offset: None,
        };

        let invalid_query = SearchQuery {
            query: r"test[".to_string(),
            filters: SearchFilters {
                use_regex: true,
                ..Default::default()
            },
            limit: None,
            offset: None,
        };

        assert!(engine.preprocess_query(&valid_query).is_ok());
        assert!(engine.preprocess_query(&invalid_query).is_err());
    }
}
