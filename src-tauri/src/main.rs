// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{
    Manager, Window, WindowEvent, GlobalShortcutManager, State
};
use everything_clone_backend::{EverythingClone, SearchQuery, SearchResult, IndexStats};

// Application state
struct AppState {
    app: Arc<Mutex<Option<Arc<EverythingClone>>>>,
}

// Tauri commands that can be called from the frontend
#[tauri::command]
async fn search_files(
    query: String,
    filters: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<SearchResult, String> {
    println!("Search called with query: '{}', filters: {}", query, filters);
    
    let app = {
        let app_guard = state.app.lock().map_err(|e| e.to_string())?;
        if let Some(ref app) = *app_guard {
            Arc::clone(app)
        } else {
            println!("Backend not initialized! Please wait for initialization to complete.");
            return Err("Backend is still initializing. Please wait a moment and try again.".to_string());
        }
    };
    
    let search_query = SearchQuery {
        query,
        filters: serde_json::from_value(filters).map_err(|e| e.to_string())?,
        limit: Some(100),
        offset: None,
    };
    
    println!("Calling backend search...");
    let result = app.search(&search_query).await.map_err(|e| e.to_string())?;
    println!("Search result: {} entries found", result.entries.len());
    Ok(result)
}

#[tauri::command]
async fn check_backend_status(state: State<'_, AppState>) -> Result<bool, String> {
    let app_guard = state.app.lock().map_err(|e| e.to_string())?;
    Ok(app_guard.is_some())
}

#[tauri::command]
async fn get_index_stats(state: State<'_, AppState>) -> Result<IndexStats, String> {
    println!("Getting index stats...");
    
    let app = {
        let app_guard = state.app.lock().map_err(|e| e.to_string())?;
        if let Some(ref app) = *app_guard {
            Arc::clone(app)
        } else {
            println!("Backend not initialized for stats!");
            return Err("Application not initialized".to_string());
        }
    };
    
    let stats = app.get_stats().await.map_err(|e| e.to_string())?;
    println!("Stats: {} files, {} folders", stats.total_files, stats.total_directories);
    Ok(stats)
}

#[tauri::command]
async fn open_file(path: String) -> Result<(), String> {
    // Check if the file exists first
    if !std::path::Path::new(&path).exists() {
        return Err(format!("File or folder does not exist: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, use the default program to open the file
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
    }
    
    Ok(())
}

#[tauri::command]
async fn open_file_location(path: String) -> Result<(), String> {
    // Check if the file or folder exists first
    let path_obj = std::path::Path::new(&path);
    if !path_obj.exists() {
        return Err(format!("File or folder does not exist: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        if path_obj.is_dir() {
            // If it's a directory, open it directly
            std::process::Command::new("explorer")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open folder '{}': {}", path, e))?;
        } else {
            // If it's a file, open the containing folder and select the file
            std::process::Command::new("explorer")
                .args(["/select,", &path])
                .spawn()
                .map_err(|e| format!("Failed to open file location '{}': {}", path, e))?;
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        let path_obj = std::path::Path::new(&path);
        if path_obj.is_dir() {
            std::process::Command::new("open")
                .arg(&path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            std::process::Command::new("open")
                .args(["-R", &path])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        let path_obj = std::path::Path::new(&path);
        if path_obj.is_dir() {
            std::process::Command::new("xdg-open")
                .arg(&path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            // For Linux, open the parent directory
            let parent = path_obj
                .parent()
                .unwrap_or_else(|| std::path::Path::new("/"));
            std::process::Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

#[tauri::command]
async fn show_window(window: Window) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn hide_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

fn setup_global_shortcut(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.clone();
    
    app.global_shortcut_manager().register("CommandOrControl+Space", move || {
        if let Some(window) = app_handle.get_window("main") {
            if window.is_visible().unwrap_or(false) {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
                // Focus the search input
                let _ = window.emit("focus-search", ());
            }
        }
    })?;
    
    Ok(())
}

async fn initialize_backend(state: Arc<Mutex<Option<Arc<EverythingClone>>>>, _app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing backend...");
    
    // Initialize Everything SDK directly - no database needed
    let mut app = EverythingClone::new().await?;
    
    println!("Starting indexing (Everything SDK handles this automatically)...");
    app.start_indexing().await?;
    
    println!("Getting initial stats...");
    let stats = app.get_stats().await?;
    println!("Initial stats: {} files, {} folders", stats.total_files, stats.total_directories);
    
    // Store in state
    let mut app_guard = state.lock().unwrap();
    *app_guard = Some(Arc::new(app));
    
    println!("Backend initialization complete!");
    Ok(())
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("everything_clone=debug,tauri=info")
        .init();

    // Create application state
    let app_state = AppState {
        app: Arc::new(Mutex::new(None)),
    };
    
    let app_state_clone = app_state.app.clone();

    tauri::Builder::default()
        .manage(app_state)
        // .system_tray(create_system_tray())
        // .on_system_tray_event(handle_system_tray_event)
        .setup(move |app| {
            // Setup global shortcut
            if let Err(e) = setup_global_shortcut(&app.handle()) {
                eprintln!("Failed to setup global shortcut: {}", e);
            }
            
            // Initialize backend synchronously during startup
            let state_clone = app_state_clone.clone();
            let handle = app.handle();
            
            tauri::async_runtime::spawn(async move {
                println!("Starting backend initialization...");
                match initialize_backend(state_clone, handle.clone()).await {
                    Ok(()) => {
                        println!("Backend initialization completed successfully!");
                        // Emit event to frontend that backend is ready
                        if let Some(window) = handle.get_window("main") {
                            let _ = window.emit("backend-ready", ());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize backend: {}", e);
                        // Show error dialog
                        if let Some(window) = handle.get_window("main") {
                            let _ = window.emit("backend-error", format!("Backend initialization failed: {}", e));
                        }
                    }
                }
            });
            
            Ok(())
        })
        .on_window_event(|event| {
            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide instead of closing when clicking X
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            search_files,
            check_backend_status,
            get_index_stats,
            open_file,
            open_file_location,
            show_window,
            hide_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
