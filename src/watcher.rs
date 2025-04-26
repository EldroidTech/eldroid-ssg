use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::broadcast;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use warp::Filter;
use futures::StreamExt;
use log::{info, error};
use portpicker::pick_unused_port;
use parking_lot::RwLock;
use std::collections::HashSet;
use std::time::Duration;

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

    pub async fn start(&self) -> notify::Result<()> {
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
            .map(|ws: warp::ws::Ws, mut rx| {
                ws.on_upgrade(move |socket| async move {
                    let (tx, _) = socket.split();
                    while rx.recv().await.is_ok() {
                        // Send reload message to browser
                        if let Err(e) = warp::ws::Message::text("reload").forward(tx).await {
                            error!("WebSocket send error: {}", e);
                            break;
                        }
                    }
                })
            });

        // Set up static file server
        let static_route = warp::fs::dir(self.output_dir.clone());
        let routes = ws_route.or(static_route);

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

    fn setup_watcher(&self, tx: broadcast::Sender<FileChange>) -> notify::Result<RecommendedWatcher> {
        let changed_files = self.changed_files.clone();
        
        let mut debouncer = tokio::time::interval(Duration::from_millis(100));
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let change_type = match event.kind {
                    notify::EventKind::Create(_) => ChangeType::Create,
                    notify::EventKind::Modify(_) => ChangeType::Modify,
                    notify::EventKind::Remove(_) => ChangeType::Delete,
                    _ => return,
                };

                for path in event.paths {
                    changed_files.write().insert(path.clone());
                    
                    // Debounce and batch changes
                    if debouncer.tick().now_or_never().is_some() {
                        let change = FileChange {
                            path,
                            event_type: change_type.clone(),
                        };
                        
                        if tx.send(change).is_err() {
                            error!("Failed to send file change event");
                        }
                    }
                }
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