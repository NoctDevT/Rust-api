use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_ws::{handle, Message, Session};
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// We'll need an atomic to assign unique IDs ourselves,
// since actix_ws::Session no longer provides an .id().
use std::sync::atomic::{AtomicU64, Ordering};

// A global (or static) atomic counter for generating unique session IDs.
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Shared type for your session map
pub type Sessions = Arc<Mutex<HashMap<u64, Session>>>;

pub async fn survey_ws_route(
    req: HttpRequest,
    payload: web::Payload,
    sessions: web::Data<Sessions>,
) -> Result<HttpResponse, Error> {
    // 1) Complete the WebSocket handshake
    let (response, mut session, mut msg_stream) = handle(&req, payload)?;

    // 2) Generate a unique ID for this connection
    let session_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    // 3) Insert the session into the shared sessions map
    {
        let mut sessions_map = sessions.lock().unwrap();
        sessions_map.insert(session_id, session.clone());
    }

    // 4) Clone sessions Arc for use inside the spawned task
    let sessions_clone = sessions.clone();

    // 5) Spawn a background task to handle messages
    actix_web::rt::spawn(async move {
            while let Some(Ok(msg)) = msg_stream.next().await {
                match msg {
                    Message::Text(text) => {
                        let mut sessions_map = sessions_clone.lock().unwrap();
                        for (_id, s) in sessions_map.iter_mut() {
                            let _ = s.text(format!("Echo: {}", text)).await;
                        }
                    }
                    Message::Binary(bin) => {
                        let mut sessions_map = sessions_clone.lock().unwrap();
                        for (_id, s) in sessions_map.iter_mut() {
                            let _ = s.binary(bin.clone()).await;
                        }
                    }
                    Message::Ping(ping) => {
                        let _ = session.pong(&ping).await;
                    }
                    _ => {}
                }
            }


        // When the stream ends, remove the session from the map
        let mut sessions_map = sessions_clone.lock().unwrap();
        sessions_map.remove(&session_id);
    });

    // 6) Return the handshake response
    Ok(response)
}
