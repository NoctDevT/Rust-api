use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_ws::{handle, Message, Session, ProtocolError};
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

use crate::services::db_service::{fetch_survey_with_questions, fetch_survey_with_id, store_survey_response};


use crate::helper::tools::validate_jwt; 
use crate::db::DbPool;
use crate::services::db_service::{get_user_id_by_username};
use crate::models::surveys::{ClientMessage, QuestionResponse, NewResponse};

//Thread safe inremental variable for uniquely identifying web socket
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct WebSocketSession {
    pub session: Session,
    pub user_id: Uuid,
}

use serde::{Deserialize, Serialize};

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
    let mut user_id: Option<Uuid> = None;

    while let Some(Ok(msg)) = msg_stream.next().await {
        match msg {
            Message::Text(txt) => {
                println!("🔍 Received WebSocket message: {}", txt); 

                match serde_json::from_str::<ClientMessage>(&txt) {
                        // Ok(parsed_message) => {
                        //     println!("✅ Parsed ClientMessage: {:?}", parsed_message); 
                        // }
                        // Err(err) => {
                        //     println!("❌ JSON Parsing Error: {:?}", err);
                        //     let _ = session.text(r#"{"status": "error", "message": "Invalid JSON format"}"#).await;
                        // }
                    Ok(ClientMessage::Authenticate { token }) => {
                        if !authenticated {
                            if let Some(uid) = authenticate_user(&token, &pool).await {
                                let ws_session = WebSocketSession {
                                    session: session.clone(),
                                    user_id: uid,
                                };
                                let mut map = sessions.lock().unwrap();
                                map.insert(session_id, ws_session);

                                user_id = Some(uid);
                                authenticated = true;

                                let _ = session.text(r#"{"status": "ok", "message": "Authentication successful"}"#).await;
                            } else {
                                let _ = session.text(r#"{"status": "error", "message": "Authentication failed"}"#).await;
                                let _ = session.close(None).await;
                                break;
                            }
                        } else {
                            let _ = session.text(r#"{"status": "error", "message": "Already authenticated"}"#).await;
                        }
                    }
                    Ok(ClientMessage::RequestSurvey { survey_id }) => {
                        if authenticated {
                            let mut conn = pool.get().unwrap();
                            match fetch_survey_with_id(&mut conn, survey_id) {
                                Ok(survey) => {
                                    let survey_json = serde_json::to_string(&survey).unwrap();
                                    let _ = session.text(survey_json).await;
                                }
                                Err(_) => {
                                    let _ = session.text(r#"{"status": "error", "message": "Survey not found"}"#).await;
                                }
                            }
                        } else {
                            let _ = session.text(r#"{"status": "error", "message": "Authenticate first"}"#).await;
                        }
                    }
                    Ok(ClientMessage::SubmitResponses (data)) => {
                        if authenticated {
                            let mut conn = pool.get().unwrap();
                            let uid = user_id.unwrap();

                            let responses_to_store: Vec<NewResponse> = data.responses.into_iter()
                                .map(|r| NewResponse {
                                    survey_id: Some(data.survey_id),
                                    question_id: r.question_id,
                                    answer: r.answer,
                                    user_id: Some(uid),
                                })
                                .collect();

                            for response in responses_to_store {
                                store_survey_response(&mut conn, response);
                            }

                            let _ = session.text(r#"{"status": "ok", "message": "Responses submitted successfully"}"#).await;
                        } else {
                            let _ = session.text(r#"{"status": "error", "message": "Authenticate first"}"#).await;
                        }
                    }
                    _ => {
                        let _ = session.text(r#"{"status": "error", "message": "Invalid request"}"#).await;
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


