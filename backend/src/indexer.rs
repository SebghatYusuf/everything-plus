use anyhow::Result;
use chrono::Utc;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::database::Database;
use crate::types::{FileEntry, SearchError};

pub struct FileIndexer {
    db: Arc<Database>,
    indexed_paths: HashSet<PathBuf>,
    exclude_patterns: Vec<String>,
    watcher: Option<RecommendedWatcher>,
}

impl FileIndexer {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            indexed_paths: HashSet::new(),
            exclude_patterns: vec![
                "System Volume Information".to_string(),
                "$Recycle.Bin".to_string(),
                "Windows/WinSxS".to_string(),
                "ProgramData/Microsoft".to_string(),
                ".git".to_string(),
                "node_modules".to_string(),
                ".vs".to_string(),
                ".vscode".to_string(),
                "target".to_string(), // Rust build directory
                "dist".to_string(),
                "build".to_string(),
            ],
            watcher: None,
        }
    }

    pub fn add_indexed_path(&mut self, path: PathBuf) {
        self.indexed_paths.insert(path);
    }

    pub fn set_exclude_patterns(&mut self, patterns: Vec<String>) {
        self.exclude_patterns = patterns;
    }

    pub async fn start_initial_indexing(&self) -> Result<()> {
        info!("Starting initial file indexing...");

        for path in &self.indexed_paths {
            if path.exists() {
                info!("Indexing path: {}", path.display());
                self.index_directory_recursive(path).await?;
            } else {
                warn!("Path does not exist: {}", path.display());
            }
        }

        info!("Initial indexing completed");
        Ok(())
    }

    pub async fn start_file_watching(&mut self) -> Result<mpsc::Receiver<Event>> {
        let (tx, rx) = mpsc::channel(1000);

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    if let Err(e) = tx.blocking_send(event) {
                        error!("Failed to send file system event: {}", e);
                    }
                }
                Err(e) => error!("File system watcher error: {}", e),
            }
        })?;

        for path in &self.indexed_paths {
            if path.exists() {
                watcher.watch(path, RecursiveMode::Recursive)?;
                info!("Started watching path: {}", path.display());
            }
        }

        self.watcher = Some(watcher);
        Ok(rx)
    }

    pub async fn handle_file_system_event(&self, event: Event) -> Result<()> {
        debug!("File system event: {:?}", event);

        match event.kind {
            EventKind::Create(_) => {
                for path in &event.paths {
                    if self.should_index_path(path) {
                        self.index_single_path(path).await?;
                    }
                }
            }
            EventKind::Modify(_) => {
                for path in &event.paths {
                    if self.should_index_path(path) {
                        self.index_single_path(path).await?;
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in &event.paths {
                    self.remove_from_index(path).await?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    async fn index_directory_recursive(&self, dir_path: &Path) -> Result<()> {
        if !self.should_index_path(dir_path) {
            return Ok(());
        }

        let mut entries = Vec::new();
        let mut stack = vec![dir_path.to_path_buf()];

        while let Some(current_path) = stack.pop() {
            if !current_path.exists() {
                continue;
            }

            // Index the current directory
            if let Ok(entry) = self.create_file_entry(&current_path).await {
                entries.push(entry);
            }

            // Process directory contents
            if current_path.is_dir() {
                match tokio::fs::read_dir(&current_path).await {
                    Ok(mut dir) => {
                        while let Ok(Some(dir_entry)) = dir.next_entry().await {
                            let path = dir_entry.path();
                            
                            if self.should_index_path(&path) {
                                if path.is_dir() {
                                    stack.push(path);
                                } else {
                                    if let Ok(entry) = self.create_file_entry(&path).await {
                                        entries.push(entry);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read directory {}: {}", current_path.display(), e);
                    }
                }
            }

            // Batch insert entries every 1000 files for performance
            if entries.len() >= 1000 {
                if let Err(e) = self.db.batch_insert_file_entries(&entries).await {
                    error!("Failed to batch insert entries: {}", e);
                }
                entries.clear();
            }
        }

        // Insert remaining entries
        if !entries.is_empty() {
            if let Err(e) = self.db.batch_insert_file_entries(&entries).await {
                error!("Failed to insert remaining entries: {}", e);
            }
        }

        Ok(())
    }

    async fn index_single_path(&self, path: &Path) -> Result<()> {
        if let Ok(entry) = self.create_file_entry(path).await {
            self.db.insert_file_entry(&entry).await?;
            debug!("Indexed file: {}", path.display());
        }
        Ok(())
    }

    async fn remove_from_index(&self, path: &Path) -> Result<()> {
        // TODO: Implement removal from database
        debug!("Removed from index: {}", path.display());
        Ok(())
    }

    async fn create_file_entry(&self, path: &Path) -> Result<FileEntry> {
        let metadata = tokio::fs::metadata(path).await?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let extension = if metadata.is_file() {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_lowercase())
        } else {
            None
        };

        let modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let created = metadata
            .created()
            .unwrap_or(metadata.modified()?)
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        Ok(FileEntry {
            id: Uuid::new_v4(),
            name,
            path: path.to_string_lossy().to_string(),
            size: metadata.len() as i64,
            modified: chrono::DateTime::from_timestamp(modified as i64, 0)
                .unwrap_or_default()
                .with_timezone(&Utc),
            created: chrono::DateTime::from_timestamp(created as i64, 0)
                .unwrap_or_default()
                .with_timezone(&Utc),
            is_directory: metadata.is_dir(),
            extension,
            attributes: 0, // TODO: Extract Windows file attributes
            indexed_at: Utc::now(),
        })
    }

    fn should_index_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();

        // Check exclude patterns
        for pattern in &self.exclude_patterns {
            if path_str.contains(&pattern.to_lowercase()) {
                return false;
            }
        }

        // Skip system files and hidden files by default
        if let Some(file_name) = path.file_name() {
            let file_name_str = file_name.to_string_lossy();
            if file_name_str.starts_with('.') && file_name_str != "." && file_name_str != ".." {
                return false;
            }
        }

        true
    }
}

#[cfg(windows)]
mod windows_integration {
    use super::*;
    use winapi::um::fileapi::GetFileAttributesW;
    use winapi::um::winnt::{FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    pub fn get_windows_attributes(path: &Path) -> i32 {
        let wide_path: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            let attrs = GetFileAttributesW(wide_path.as_ptr());
            if attrs == winapi::um::fileapi::INVALID_FILE_ATTRIBUTES {
                0
            } else {
                attrs as i32
            }
        }
    }

    pub fn is_hidden_or_system(attributes: i32) -> bool {
        (attributes & FILE_ATTRIBUTE_HIDDEN as i32) != 0
            || (attributes & FILE_ATTRIBUTE_SYSTEM as i32) != 0
    }
}

#[cfg(not(windows))]
mod windows_integration {
    use super::*;

    pub fn get_windows_attributes(_path: &Path) -> i32 {
        0
    }

    pub fn is_hidden_or_system(_attributes: i32) -> bool {
        false
    }
}
