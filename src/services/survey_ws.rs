use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_ws::{handle, Message, Session};
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub type Sessions = Arc<Mutex<HashMap<u64, Session>>>;

pub async fn survey_ws_route(
    req: HttpRequest,
    payload: web::Payload,
    sessions: web::Data<Sessions>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = handle(&req, payload)?;

    let session_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    {
        let mut sessions_map = sessions.lock().unwrap();
        sessions_map.insert(session_id, session.clone());
    }

    let sessions_clone = sessions.clone();

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


        let mut sessions_map = sessions_clone.lock().unwrap();
        sessions_map.remove(&session_id);
    });

    Ok(response)
}
