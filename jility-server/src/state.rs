use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub ws_state: Arc<WebSocketState>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db),
            ws_state: Arc::new(WebSocketState::new()),
        }
    }
}

// WebSocket state for broadcasting updates
pub struct WebSocketState {
    pub subscribers: RwLock<Vec<tokio::sync::mpsc::UnboundedSender<String>>>,
}

impl WebSocketState {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(Vec::new()),
        }
    }

    pub async fn subscribe(&self, tx: tokio::sync::mpsc::UnboundedSender<String>) {
        self.subscribers.write().await.push(tx);
    }

    pub async fn broadcast(&self, message: String) {
        let mut subscribers = self.subscribers.write().await;
        subscribers.retain(|tx| tx.send(message.clone()).is_ok());
    }
}

pub async fn connect_database(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}
