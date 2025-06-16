use std::path::{PathBuf};
use std::sync::Arc;
use tokio::sync::broadcast;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use warp::Filter;
use futures::StreamExt;
use futures::SinkExt;
use log::{info, error};
use portpicker::pick_unused_port;
use parking_lot::RwLock;
use std::collections::HashSet;
use std::time::Duration;
use std::fs;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DevServerError {
    #[error("Failed to create directory: {0}")]
    DirectoryCreation(#[from] io::Error),
    #[error("Watcher error: {0}")]
    Watcher(#[from] notify::Error),
}

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub event_type: ChangeType,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Create,
    Modify,
    Delete,
    CssChange,  // Special handling for CSS files
    Error(String),  // For tracking build/processing errors
}

pub struct DevServer {
    input_dir: PathBuf,
    output_dir: PathBuf,
    components_dir: PathBuf,
    port: u16,
    ws_port: u16,
    changed_files: Arc<RwLock<HashSet<PathBuf>>>,
}

impl DevServer {
    pub fn new(
        input_dir: impl Into<PathBuf>,
        output_dir: impl Into<PathBuf>,
        components_dir: impl Into<PathBuf>,
        port: Option<u16>,
        ws_port: Option<u16>,
    ) -> Self {
        Self {
            input_dir: input_dir.into(),
            output_dir: output_dir.into(),
            components_dir: components_dir.into(),
            port: port.unwrap_or_else(|| pick_unused_port().expect("No ports available")),
            ws_port: ws_port.unwrap_or_else(|| pick_unused_port().expect("No ports available")),
            changed_files: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    
    fn ensure_directory(&self, path: &PathBuf) -> Result<(), DevServerError> {
        if !path.exists() {
            fs::create_dir_all(path)?;
            info!("Created directory: {}", path.display());
        }
        Ok(())
    }

    fn initialize_directories(&self) -> Result<(), DevServerError> {
        // Ensure all required directories exist
        self.ensure_directory(&self.input_dir)?;
        self.ensure_directory(&self.output_dir)?;
        self.ensure_directory(&self.components_dir)?;
        
        info!("Initialized directory structure:");
        info!("  Input dir:      {}", self.input_dir.display());
        info!("  Output dir:     {}", self.output_dir.display());
        info!("  Components dir: {}", self.components_dir.display());
        
        Ok(())
    }

    pub async fn start(&self) -> Result<(), DevServerError> {
        // Initialize directories first
        self.initialize_directories()?;
        
        // Set up file watcher
        let (tx, _) = broadcast::channel(100);
        let tx_clone = tx.clone();
        
        let mut watcher = self.setup_watcher(tx)?;
        
        // Watch input and components directories
        watcher.watch(&self.input_dir, RecursiveMode::Recursive)?;
        watcher.watch(&self.components_dir, RecursiveMode::Recursive)?;

        // Set up WebSocket for live reload
        let ws_route = warp::path("ws")
            .and(warp::ws())
            .and(warp::any().map(move || tx_clone.subscribe()))
            .map(|ws: warp::ws::Ws, mut rx: broadcast::Receiver<FileChange>| {
                ws.on_upgrade(move |socket| async move {
                    let (mut tx, _) = socket.split();
                    while let Ok(change) = rx.recv().await {
                        let msg = match change.event_type {
                            ChangeType::CssChange => {
                                // For CSS changes, send a special message to reload only CSS
                                format!("{{\"type\":\"css\",\"path\":\"{}\"}}", 
                                    change.path.display())
                            },
                            ChangeType::Error(err) => {
                                // For errors, send error details to show in overlay
                                format!("{{\"type\":\"error\",\"message\":\"{}\"}}", err)
                            },
                            _ => {
                                // For other changes, do a full page reload
                                "reload".to_string()
                            }
                        };
                        
                        if let Err(e) = tx.send(warp::ws::Message::text(msg)).await {
                            error!("WebSocket send error: {}", e);
                            break;
                        }
                    }
                })
            });

        // Set up static file server
        let static_route = warp::fs::dir(self.output_dir.clone());
        let routes = ws_route.clone().or(static_route);

        // Start the servers
        let server_handle = tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], self.port)));
        let ws_handle = tokio::spawn(warp::serve(ws_route).run(([127, 0, 0, 1], self.ws_port)));

        info!("Development server running at http://localhost:{}", self.port);
        info!("WebSocket server running at ws://localhost:{}", self.ws_port);

        // Keep the server running
        tokio::select! {
            _ = server_handle => {},
            _ = ws_handle => {},
        }

        Ok(())
    }

    fn setup_watcher(&self, tx: broadcast::Sender<FileChange>) -> Result<RecommendedWatcher, DevServerError> {
        let changed_files = self.changed_files.clone();
        let mut last_event = std::time::Instant::now();
        let debounce_duration = Duration::from_millis(100);
        
        let watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let now = std::time::Instant::now();
                if now.duration_since(last_event) < debounce_duration {
                    return;
                }

                let change_type = match event.kind {
                    notify::EventKind::Create(_) => ChangeType::Create,
                    notify::EventKind::Modify(_) => {
                        // Special handling for CSS changes
                        if event.paths.iter().any(|p| p.extension().map_or(false, |ext| ext == "css")) {
                            ChangeType::CssChange
                        } else {
                            ChangeType::Modify
                        }
                    },
                    notify::EventKind::Remove(_) => ChangeType::Delete,
                    _ => return,
                };

                for path in event.paths {
                    changed_files.write().insert(path.clone());
                    let change = FileChange {
                        path,
                        event_type: change_type.clone(),
                    };
                    
                    if tx.send(change).is_err() {
                        error!("Failed to send file change event");
                    }
                }
                last_event = now;
            } else if let Err(e) = res {
                error!("File watcher error: {}", e);
            }
        })?;

        Ok(watcher)
    }

    pub fn get_changed_files(&self) -> HashSet<PathBuf> {
        self.changed_files.read().clone()
    }

    pub fn clear_changed_files(&self) {
        self.changed_files.write().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_ensure_directory() {
        let temp = tempdir().unwrap();
        let test_dir = temp.path().join("test");
        let server = DevServer::new(
            test_dir.clone(),
            temp.path().join("output"),
            temp.path().join("components"),
            Some(8080),
            Some(8081),
        );

        // Test directory creation
        assert!(!test_dir.exists());
        server.ensure_directory(&test_dir).unwrap();
        assert!(test_dir.exists());

        // Test idempotency
        server.ensure_directory(&test_dir).unwrap();
        assert!(test_dir.exists());
    }

    #[test]
    fn test_initialize_directories() {
        let temp = tempdir().unwrap();
        let input_dir = temp.path().join("input");
        let output_dir = temp.path().join("output");
        let components_dir = temp.path().join("components");

        let server = DevServer::new(
            input_dir.clone(),
            output_dir.clone(),
            components_dir.clone(),
            Some(8080),
            Some(8081),
        );

        // Test initial state
        assert!(!input_dir.exists());
        assert!(!output_dir.exists());
        assert!(!components_dir.exists());

        // Initialize directories
        server.initialize_directories().unwrap();

        // Verify all directories were created
        assert!(input_dir.exists());
        assert!(output_dir.exists());
        assert!(components_dir.exists());
    }
}