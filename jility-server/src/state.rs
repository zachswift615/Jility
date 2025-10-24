use jility_core::search::SearchService;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub ws_state: Arc<WebSocketState>,
    pub auth_service: AuthService,
    pub search_service: Arc<SearchService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        let db = Arc::new(db);
        Self {
            search_service: Arc::new(SearchService::new(db.clone())),
            db,
            ws_state: Arc::new(WebSocketState::new()),
            auth_service: AuthService::new(jwt_secret),
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
