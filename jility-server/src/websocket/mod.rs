use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;

use crate::state::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for this websocket client
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Subscribe this client to broadcasts
    state.ws_state.subscribe(tx).await;

    // Spawn a task to send messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from this client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    tracing::debug!("Received WebSocket message: {}", text);
                    // TODO: Handle client messages (e.g., subscriptions, filters)
                }
                Message::Close(_) => {
                    tracing::debug!("WebSocket client disconnected");
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    tracing::debug!("WebSocket connection closed");
}

pub fn websocket_routes() -> axum::Router<AppState> {
    axum::Router::new().route("/ws", axum::routing::get(websocket_handler))
}
