use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_ws::{handle, Message, Session, ProtocolError};
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

use crate::helper::tools::validate_jwt; 
use crate::db::DbPool;
use crate::services::db_service::get_user_id_by_username;

//Thread safe inremental variable for uniquely identifying web socket
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct WebSocketSession {
    pub session: Session,
    pub user_id: Uuid,
}

use serde::{Deserialize, Serialize};


//Annotation enables json message handling
//type is action, data contains relevant content
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ClientMessage {
    Authenticate { token: String },
    Chat { message: String },
    Ping,
}

// {
//     "type": "Authenticate",
//     "data": {
//       "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0dXNlcjIiLCJleHAiOjE3NDEwNTc5MDR9.VqZYof-bIHH6rYib-uQ4cTpDtZ6HTQtWZ6uyuhTnFh0"
//     }
// }
  
// {
//     "type": "Chat",
//     "data": {
//       "message": "Hello, everyone!"
//    }
// }

// {
//     "type": "Ping"
// }
  

//session map using hashmap, 

pub type Sessions = Arc<Mutex<HashMap<u64, WebSocketSession>>>;

pub async fn survey_ws_route(
    req: HttpRequest,
    payload: web::Payload,
    sessions: web::Data<Sessions>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let (response, session, msg_stream) = handle(&req, payload)?;

    let session_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    let sessions_clone = sessions.clone();
    let pool_clone = pool.clone();

    //this is moving the data from outside the scope within function scope 
    actix_web::rt::spawn(async move {
        handle_ws_messages(session_id, session, msg_stream, sessions_clone, pool_clone).await;
    });

    Ok(response)
}

//Multi-tier socket 
// first message authenticate using jwt token foundin /login route
// other messages so far only respond with "already authenticated"
// ping pong implementation for now 
//end of stream closes session

async fn handle_ws_messages(
    session_id: u64,
    mut session: Session, 
    mut msg_stream: impl StreamExt<Item = Result<Message, ProtocolError>> + Unpin,
    sessions: web::Data<Sessions>,
    pool: web::Data<DbPool>,
) {
    let mut authenticated = false;

    while let Some(Ok(msg)) = msg_stream.next().await {
        match msg {
            Message::Text(txt) => {
                match serde_json::from_str::<ClientMessage>(&txt) {
                    Ok(ClientMessage::Authenticate { token }) => {
                        if !authenticated {
                            match authenticate_user(&token, &pool).await {
                                Some(user_id) => {
                                    //building session
                                    let ws_session = WebSocketSession {
                                        session: session.clone(),
                                        user_id,
                                    };
                                    //storing it in the session map
                                    let mut map = sessions.lock().unwrap();
                                    map.insert(session_id, ws_session);

                                    let _ = session.text(r#"{"status": "ok", "message": "Authentication successful"}"#).await;
                                    authenticated = true;
                                }
                                None => {
                                    let _ = session.text(r#"{"status": "error", "message": "Authentication failed"}"#).await;
                                    let _ = session.close(None).await;
                                    break;
                                }
                            }
                        } else {
                            let _ = session.text(r#"{"status": "error", "message": "Already authenticated"}"#).await;
                        }
                    }
                    Ok(ClientMessage::Chat { message }) => {
                        if authenticated {
                            let map = sessions.lock().unwrap();
                            for (_, ws_session) in map.iter() {
                                let mut session_clone = ws_session.session.clone(); 
                                let _ = session_clone.text(format!(r#"{{"type": "chat", "message": "{}"}}"#, message)).await;
                            }
                        } else {
                            let _ = session.text(r#"{"status": "error", "message": "Authenticate first"}"#).await;
                        }
                    }
                    Ok(ClientMessage::Ping) => {
                        let _ = session.text(r#"{"type": "pong"}"#).await;
                    }
                    Err(_) => {
                        let _ = session.text(r#"{"status": "error", "message": "Invalid JSON format"}"#).await;
                    }
                }
            }
            Message::Ping(bytes) => {
                let _ = session.pong(&bytes).await;
            }
            Message::Close(reason) => {
                let _ = session.close(reason).await;
                break;
            }
            _ => {}
        }
    }

    let mut map = sessions.lock().unwrap();
    map.remove(&session_id);
}


/// Sanity test to validate JWT and to check if user exists 
async fn authenticate_user(token: &str, pool: &DbPool) -> Option<Uuid> {
    let username = validate_jwt(token).ok()?;

    let mut conn = pool.get().ok()?;
    let user_id = get_user_id_by_username(&mut conn, &username).ok()?;
    Some(user_id)
}


