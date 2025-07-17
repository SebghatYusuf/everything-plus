use anyhow::Result;
use std::path::PathBuf;
use tracing::info;
use crate::types::{FileEntry, SearchQuery, SearchResult, IndexStats};

#[cfg(windows)]
use {
    std::ffi::{CString, OsString},
    std::os::windows::ffi::OsStringExt,
    winapi::{
        shared::minwindef::*,
        um::libloaderapi::*,
    },
};

// Everything SDK constants
// #[cfg(windows)]
// const EVERYTHING_REQUEST_FILE_NAME: u32 = 0x00000001;
// #[cfg(windows)]
// const EVERYTHING_REQUEST_PATH: u32 = 0x00000002;
// #[cfg(windows)]
// const EVERYTHING_REQUEST_EXTENSION: u32 = 0x00000008;
#[cfg(windows)]
const EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME: u32 = 0x00000004;
#[cfg(windows)]
const EVERYTHING_REQUEST_SIZE: u32 = 0x00000010;
#[cfg(windows)]
const EVERYTHING_REQUEST_DATE_CREATED: u32 = 0x00000020;
#[cfg(windows)]
const EVERYTHING_REQUEST_DATE_MODIFIED: u32 = 0x00000040;
#[cfg(windows)]
const EVERYTHING_REQUEST_ATTRIBUTES: u32 = 0x00000080;

#[cfg(windows)]
const EVERYTHING_SORT_NAME_ASCENDING: u32 = 1;
#[cfg(windows)]
const EVERYTHING_SORT_SIZE_ASCENDING: u32 = 5;
#[cfg(windows)]
const EVERYTHING_SORT_DATE_MODIFIED_ASCENDING: u32 = 9;

// Everything SDK function types
#[cfg(windows)]
type EverythingSetSearchW = unsafe extern "stdcall" fn(*const u16);
#[cfg(windows)]
type EverythingSetRequestFlags = unsafe extern "stdcall" fn(u32);
#[cfg(windows)]
type EverythingQueryW = unsafe extern "stdcall" fn(BOOL) -> BOOL;
#[cfg(windows)]
type EverythingGetNumResults = unsafe extern "stdcall" fn() -> u32;
#[cfg(windows)]
type EverythingGetResultFullPathNameW = unsafe extern "stdcall" fn(u32, *mut u16, u32) -> u32;
#[cfg(windows)]
type EverythingGetResultSize = unsafe extern "stdcall" fn(u32, *mut i64) -> BOOL;
#[cfg(windows)]
type EverythingGetResultDateCreated = unsafe extern "stdcall" fn(u32, *mut u64) -> BOOL;
#[cfg(windows)]
type EverythingGetResultDateModified = unsafe extern "stdcall" fn(u32, *mut u64) -> BOOL;
#[cfg(windows)]
type EverythingGetResultAttributes = unsafe extern "stdcall" fn(u32) -> u32;
#[cfg(windows)]
type EverythingIsFileResult = unsafe extern "stdcall" fn(u32) -> BOOL;
#[cfg(windows)]
type EverythingIsFolderResult = unsafe extern "stdcall" fn(u32) -> BOOL;
#[cfg(windows)]
type EverythingSetSort = unsafe extern "stdcall" fn(u32);
#[cfg(windows)]
type EverythingSetMax = unsafe extern "stdcall" fn(u32);
#[cfg(windows)]
type EverythingReset = unsafe extern "stdcall" fn();
#[cfg(windows)]
type EverythingGetLastError = unsafe extern "stdcall" fn() -> u32;

/// Everything SDK wrapper for Windows file search
pub struct EverythingSDK {
    #[cfg(windows)]
    dll_handle: Option<HMODULE>,
    initialized: bool,
    fallback_mode: bool, // Use when Everything DLL is not available
}

// Implement Send and Sync for EverythingSDK
unsafe impl Send for EverythingSDK {}
unsafe impl Sync for EverythingSDK {}

impl EverythingSDK {
    pub fn new() -> Result<Self> {
        Ok(Self {
            #[cfg(windows)]
            dll_handle: None,
            initialized: false,
            fallback_mode: false,
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing Everything SDK...");
        
        #[cfg(windows)]
        {
            match self.load_everything_dll() {
                Ok(()) => {
                    info!("Everything SDK initialized successfully with DLL");
                    self.fallback_mode = false;
                }
                Err(e) => {
                    info!("Failed to load Everything DLL, using fallback mode: {}", e);
                    self.fallback_mode = true;
                }
            }
        }
        
        #[cfg(not(windows))]
        {
            info!("Running on non-Windows platform, using fallback mode");
            self.fallback_mode = true;
        }
        
        self.initialized = true;
        info!("Everything SDK initialized successfully");
        Ok(())
    }

    #[cfg(windows)]
    fn load_everything_dll(&mut self) -> Result<()> {
        unsafe {
            // Try different locations and DLL names
            let dll_paths = vec![
                // Current directory first
                "Everything64.dll",
                "Everything32.dll", 
                "Everything.dll",
                // Everything SDK DLL paths (from the SDK installation)
                "C:\\Program Files\\Everything\\Everything-SDK\\dll\\Everything64.dll",
                "C:\\Program Files\\Everything\\Everything-SDK\\dll\\Everything32.dll",
                "C:\\Program Files\\Everything\\Everything-SDK\\dll\\Everything.dll",
                // Common installation paths
                "C:\\Program Files\\Everything\\Everything64.dll",
                "C:\\Program Files\\Everything\\Everything32.dll",
                "C:\\Program Files\\Everything\\Everything.dll",
                "C:\\Program Files (x86)\\Everything\\Everything32.dll",
                "C:\\Program Files (x86)\\Everything\\Everything.dll",
                "C:\\Program Files (x86)\\Everything\\Everything-SDK\\dll\\Everything64.dll",
                "C:\\Program Files (x86)\\Everything\\Everything-SDK\\dll\\Everything32.dll",
                "C:\\Program Files (x86)\\Everything\\Everything-SDK\\dll\\Everything.dll",
                // Portable versions in common locations
                "C:\\Tools\\Everything\\Everything64.dll",
                "C:\\Tools\\Everything\\Everything32.dll",
                "C:\\Tools\\Everything\\Everything.dll",
                // In system PATH
                "Everything64.dll",
                "Everything32.dll",
            ];

            for dll_path in dll_paths {
                let dll_name_cstring = CString::new(dll_path)?;
                let handle = LoadLibraryA(dll_name_cstring.as_ptr());
                
                if !handle.is_null() {
                    info!("Successfully loaded Everything DLL from: {}", dll_path);
                    self.dll_handle = Some(handle);
                    return Ok(());
                }
            }
            
            // If we get here, none of the DLL paths worked
            Err(anyhow::anyhow!(
                "Failed to load Everything DLL from any location. Please ensure Everything is installed.\n\
                Tried locations:\n\
                - Current directory (Everything64.dll, Everything32.dll, Everything.dll)\n\
                - C:\\Program Files\\Everything\\\n\
                - C:\\Program Files (x86)\\Everything\\\n\
                - C:\\Tools\\Everything\\\n\
                \n\
                You can download Everything from: https://www.voidtools.com/downloads/\n\
                Make sure Everything is running in the background."
            ))
        }
    }

    #[cfg(windows)]
    fn get_function<T>(&self, name: &str) -> Result<T> {
        let handle = self.dll_handle.ok_or_else(|| anyhow::anyhow!("DLL not loaded"))?;
        
        unsafe {
            let name_cstring = CString::new(name)?;
            let proc_addr = GetProcAddress(handle, name_cstring.as_ptr());
            
            if proc_addr.is_null() {
                return Err(anyhow::anyhow!("Failed to get procedure address for {}", name));
            }
            
            Ok(std::mem::transmute_copy(&proc_addr))
        }
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResult> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Everything SDK not initialized"));
        }

        info!("Searching with query: '{:?}'", query);

        if self.fallback_mode {
            self.fallback_search(query)
        } else {
            #[cfg(windows)]
            {
                self.perform_search(query)
            }
            
            #[cfg(not(windows))]
            {
                self.fallback_search(query)
            }
        }
    }

    #[cfg(windows)]
    fn perform_search(&self, query: &SearchQuery) -> Result<SearchResult> {
        let start_time = std::time::Instant::now();
        
        unsafe {
            // Get function pointers
            let reset: EverythingReset = self.get_function("Everything_Reset")?;
            let set_request_flags: EverythingSetRequestFlags = self.get_function("Everything_SetRequestFlags")?;
            let set_sort: EverythingSetSort = self.get_function("Everything_SetSort")?;
            let set_max: EverythingSetMax = self.get_function("Everything_SetMax")?;
            let set_search: EverythingSetSearchW = self.get_function("Everything_SetSearchW")?;
            let query_fn: EverythingQueryW = self.get_function("Everything_QueryW")?;
            let get_num_results: EverythingGetNumResults = self.get_function("Everything_GetNumResults")?;
            let get_last_error: EverythingGetLastError = self.get_function("Everything_GetLastError")?;
            
            // Reset Everything state
            reset();
            
            // Set request flags for the data we want
            set_request_flags(
                EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME |
                EVERYTHING_REQUEST_SIZE |
                EVERYTHING_REQUEST_DATE_CREATED |
                EVERYTHING_REQUEST_DATE_MODIFIED |
                EVERYTHING_REQUEST_ATTRIBUTES
            );
            
            // Set sort order
            set_sort(EVERYTHING_SORT_NAME_ASCENDING);
            
            // Set maximum number of results
            set_max(query.limit.unwrap_or(1000));

            // Construct the search query string
            let search_string = self.build_search_string(query);
            
            // Convert search term to wide string
            let search_wide: Vec<u16> = search_string.encode_utf16().chain(std::iter::once(0)).collect();
            
            // Set the search term
            set_search(search_wide.as_ptr());
            
            // Execute the query
            let result = query_fn(1); // TRUE for wait
            if result == 0 {
                let error_code = get_last_error();
                return Err(anyhow::anyhow!("Everything query failed with error code: {}", error_code));
            }
            
            // Get number of results
            let num_results = get_num_results();
            
            info!("Everything returned {} results for query: {}", num_results, search_string);
            
            // Process results
            let mut entries = Vec::new();
            for i in 0..num_results {
                if let Ok(entry) = self.get_result_entry(i) {
                    entries.push(entry);
                }
            }
            
            let query_time = start_time.elapsed().as_millis() as u64;
            
            Ok(SearchResult {
                entries,
                total_count: num_results as u64,
                query_time_ms: query_time,
            })
        }
    }

    #[cfg(windows)]
    fn build_search_string(&self, query: &SearchQuery) -> String {
        let mut search_parts = Vec::new();

        // Main query
        if !query.query.trim().is_empty() {
            search_parts.push(query.query.clone());
        }

        // File type filters
        if !query.filters.file_types.is_empty() {
            let types_str = query.filters.file_types.iter()
                .map(|t| format!("ext:{}", t))
                .collect::<Vec<_>>()
                .join("|");
            search_parts.push(format!("({})", types_str));
        }

        // Size filters
        if let Some(min) = query.filters.size_min {
            search_parts.push(format!("size:>={}", min));
        }
        if let Some(max) = query.filters.size_max {
            search_parts.push(format!("size:<={}", max));
        }

        // Date filters (Everything uses 'dm:' for date modified)
        if let Some(from) = query.filters.date_from {
            search_parts.push(format!("dm:>={}", from.format("%Y-%m-%d")));
        }
        if let Some(to) = query.filters.date_to {
            search_parts.push(format!("dm:<={}", to.format("%Y-%m-%d")));
        }

        // Boolean flags
        if query.filters.directories_only {
            search_parts.push("folder:".to_string());
        } else if query.filters.files_only {
            search_parts.push("file:".to_string());
        }

        // Note: include_hidden is usually a default behavior in Everything
        // case_sensitive and use_regex are handled by Everything's own syntax if needed,
        // but here we rely on its default smart behavior. For more complex cases,
        // the main query string could be constructed to include `case:` or `regex:`.

        search_parts.join(" ")
    }

    #[cfg(windows)]
    unsafe fn get_result_entry(&self, index: u32) -> Result<FileEntry> {
        // Get function pointers
        let get_result_full_path: EverythingGetResultFullPathNameW = self.get_function("Everything_GetResultFullPathNameW")?;
        let is_folder_result: EverythingIsFolderResult = self.get_function("Everything_IsFolderResult")?;
        let get_result_size: EverythingGetResultSize = self.get_function("Everything_GetResultSize")?;
        let get_result_date_created: EverythingGetResultDateCreated = self.get_function("Everything_GetResultDateCreated")?;
        let get_result_date_modified: EverythingGetResultDateModified = self.get_function("Everything_GetResultDateModified")?;
        let get_result_attributes: EverythingGetResultAttributes = self.get_function("Everything_GetResultAttributes")?;
        
        // Get full path
        let mut path_buffer = vec![0u16; 32768]; // MAX_PATH * 16 for safety
        let path_len = get_result_full_path(index, path_buffer.as_mut_ptr(), path_buffer.len() as u32);
        
        if path_len == 0 {
            return Err(anyhow::anyhow!("Failed to get result path for index {}", index));
        }
        
        // Convert wide string to Rust string
        path_buffer.truncate(path_len as usize);
        let full_path = OsString::from_wide(&path_buffer);
        let path_str = full_path.to_string_lossy().to_string();
        
        // Get file/folder name from path
        let path_buf = PathBuf::from(&path_str);
        let name = path_buf.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        // Check if it's a file or folder
        let is_directory = is_folder_result(index) != 0;
        
        // Get file size
        let mut file_size: i64 = 0;
        if !is_directory {
            get_result_size(index, &mut file_size);
        }
        
        // Get dates
        let mut created_time: u64 = 0;
        let mut modified_time: u64 = 0;
        
        get_result_date_created(index, &mut created_time);
        get_result_date_modified(index, &mut modified_time);
        
        // Get file attributes
        let attributes = get_result_attributes(index);
        
        // Convert Windows FILETIME to chrono DateTime
        let created = self.filetime_to_datetime(created_time);
        let modified = self.filetime_to_datetime(modified_time);
        
        // Get file extension
        let extension = if !is_directory {
            path_buf.extension().map(|e| e.to_string_lossy().to_string())
        } else {
            None
        };
        
        Ok(FileEntry {
            id: format!("everything_{}", index),
            name,
            path: path_str,
            size: if is_directory { 0 } else { file_size },
            modified,
            created,
            is_directory,
            extension,
            attributes: attributes as i32,
        })
    }

    #[cfg(windows)]
    fn filetime_to_datetime(&self, filetime: u64) -> chrono::DateTime<chrono::Utc> {
        // Windows FILETIME is 100-nanosecond intervals since January 1, 1601 UTC
        // Unix timestamp is seconds since January 1, 1970 UTC
        // The difference is 11644473600 seconds
        
        if filetime == 0 {
            return chrono::Utc::now(); // fallback to current time
        }
        
        let windows_epoch_diff = 11644473600u64;
        let seconds = filetime / 10_000_000;
        
        if seconds > windows_epoch_diff {
            let unix_seconds = seconds - windows_epoch_diff;
            if let Some(dt) = chrono::DateTime::from_timestamp(unix_seconds as i64, 0) {
                return dt;
            }
        }
        
        chrono::Utc::now() // fallback
    }

    pub async fn get_stats(&self) -> Result<IndexStats> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Everything SDK not initialized"));
        }

        if self.fallback_mode {
            // Return stats for fallback mode
            Ok(IndexStats {
                total_files: 100000, // Estimated
                total_directories: 10000, // Estimated
                indexed_paths: vec![
                    "C:\\Windows\\System32".to_string(),
                    "C:\\Program Files".to_string(),
                    "C:\\Program Files (x86)".to_string(),
                    "C:\\Users".to_string(),
                ],
                last_index_time: chrono::Utc::now(),
                index_size_bytes: 10 * 1024 * 1024, // 10MB estimated
            })
        } else {
            // Return stats for Everything SDK mode
            Ok(IndexStats {
                total_files: 0, // Could be queried with an empty search
                total_directories: 0,
                indexed_paths: vec![
                    "C:\\".to_string(), // Everything typically indexes all NTFS drives
                ],
                last_index_time: chrono::Utc::now(),
                index_size_bytes: 0, // Not available via SDK
            })
        }
    }

    fn fallback_search(&self, query: &SearchQuery) -> Result<SearchResult> {
        info!("Using fallback file system search for: '{:?}'", query);
        let start_time = std::time::Instant::now();
        
        if query.query.is_empty() {
            return Ok(SearchResult {
                entries: Vec::new(),
                total_count: 0,
                query_time_ms: 0,
            });
        }
        
        let mut entries = Vec::new();
        let search_term_lower = query.query.to_lowercase();
        let max_results = query.limit.unwrap_or(1000);
        
        // Search in common Windows directories
        let search_paths = vec![
            "C:\\Windows\\System32",
            "C:\\Program Files",
            "C:\\Program Files (x86)",
            "C:\\Users",
        ];
        
        for search_path in search_paths {
            if entries.len() >= max_results as usize {
                break;
            }
            
            if let Ok(read_dir) = std::fs::read_dir(search_path) {
                for entry in read_dir.flatten() {
                    if entries.len() >= max_results as usize {
                        break;
                    }
                    
                    let path = entry.path();
                    let file_name = path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    
                    if file_name.to_lowercase().contains(&search_term_lower) {
                        if let Ok(metadata) = entry.metadata() {
                            let is_directory = metadata.is_dir();
                            let size = if is_directory { 0 } else { metadata.len() as i64 };
                            
                            // Get timestamps
                            let created = metadata.created()
                                .map(|t| chrono::DateTime::<chrono::Utc>::from(t))
                                .unwrap_or_else(|_| chrono::Utc::now());
                            let modified = metadata.modified()
                                .map(|t| chrono::DateTime::<chrono::Utc>::from(t))
                                .unwrap_or_else(|_| chrono::Utc::now());
                            
                            let extension = if !is_directory {
                                path.extension().map(|e| e.to_string_lossy().to_string())
                            } else {
                                None
                            };
                            
                            entries.push(FileEntry {
                                id: format!("fallback_{}", entries.len()),
                                name: file_name,
                                path: path.to_string_lossy().to_string(),
                                size,
                                modified,
                                created,
                                is_directory,
                                extension,
                                attributes: 0,
                            });
                        }
                    }
                }
            }
        }
        
        let query_time = start_time.elapsed().as_millis() as u64;
        let total_count = entries.len() as u64;
        
        Ok(SearchResult {
            entries,
            total_count,
            query_time_ms: query_time,
        })
    }
}

impl Drop for EverythingSDK {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            if let Some(handle) = self.dll_handle {
                unsafe {
                    FreeLibrary(handle);
                }
            }
        }
    }
}
