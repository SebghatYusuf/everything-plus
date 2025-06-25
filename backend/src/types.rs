use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub modified: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub is_directory: bool,
    pub extension: Option<String>,
    pub attributes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub filters: SearchFilters,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub file_types: Vec<String>,
    pub size_min: Option<i64>,
    pub size_max: Option<i64>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub include_hidden: bool,
    pub case_sensitive: bool,
    pub use_regex: bool,
    pub search_content: bool,
    pub directories_only: bool,
    pub files_only: bool,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            file_types: Vec::new(),
            size_min: None,
            size_max: None,
            date_from: None,
            date_to: None,
            include_hidden: false,
            case_sensitive: false,
            use_regex: false,
            search_content: false,
            directories_only: false,
            files_only: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub entries: Vec<FileEntry>,
    pub total_count: u64,
    pub query_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: u64,
    pub total_directories: u64,
    pub indexed_paths: Vec<String>,
    pub last_index_time: DateTime<Utc>,
    pub index_size_bytes: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    
    #[error("Everything SDK error: {0}")]
    EverythingSdk(String),
}
