use futures::{SinkExt, StreamExt};
use warp::Filter;
use warp::ws::{WebSocket};

#[allow(opaque_hidden_inferred_bound)]
pub fn websocket() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async move {
                handle_client(websocket).await;
            })
        })
}

async fn handle_client(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    // Echo the message back to the client
                    if let Err(e) = tx.send(msg).await {
                        eprintln!("Error sending message: {:?}", e);
                        break;
                    }
                } else if msg.is_close() {
                    // Close the connection if the client requests it
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                break;
            }
        }
    }

    println!("WebSocket connection closed.");
}