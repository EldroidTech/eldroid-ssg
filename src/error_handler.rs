use std::sync::Arc;
use tokio::sync::broadcast;
use anyhow::Result;
use log::error;

use crate::watcher::{FileChange, ChangeType};

pub struct ErrorHandler {
    tx: broadcast::Sender<FileChange>,
}

impl ErrorHandler {
    pub fn new(tx: broadcast::Sender<FileChange>) -> Self {
        Self { tx }
    }

    pub fn handle_error(&self, err: anyhow::Error, file_path: Option<&str>) -> Result<()> {
        let error_message = format!("Build Error: {}\n\nLocation: {}", 
            err, 
            file_path.unwrap_or("Unknown"));

        error!("{}", error_message);

        // Send error to client for overlay display
        let change = FileChange {
            path: file_path.map(std::path::PathBuf::from)
                .unwrap_or_else(|| std::path::PathBuf::from("unknown")),
            event_type: ChangeType::Error(error_message),
        };

        if let Err(e) = self.tx.send(change) {
            error!("Failed to send error to client: {}", e);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ErrorHandlerMiddleware {
    error_handler: Arc<ErrorHandler>,
}

impl ErrorHandlerMiddleware {
    pub fn new(tx: broadcast::Sender<FileChange>) -> Self {
        Self {
            error_handler: Arc::new(ErrorHandler::new(tx)),
        }
    }

    pub fn handle(&self, error: anyhow::Error, file_path: Option<&str>) -> Result<()> {
        self.error_handler.handle_error(error, file_path)
    }
}
