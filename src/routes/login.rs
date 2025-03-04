
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify; 
use chrono::{Utc, Duration};
use std::env;
use jsonwebtoken::{encode, EncodingKey, Header};
use actix_web::http::header::{AUTHORIZATION};
// use actix_web::cookie::{Cookie, SameSite};

use crate::db::DbPool;
use crate::models::login_model::LoginRequest;
use crate::models::claims::Claims;

use crate::services::db_service::get_user_by_username;

// Simplified logic of login page following code review. 
// Treating it more like a traditional rest api control flow 

//Memory efficiency to keep space low. 
const INVALID_CREDENTIALS: &str = "Invalid username or password provided"; 
const SERVER_ERROR: &str = "Server error, please try again later"; 

pub async fn user_login(
    pool: web::Data<DbPool>, 
    data: web::Json<LoginRequest>
) -> impl Responder {

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(SERVER_ERROR),
    };

    let user = match get_user_by_username(&mut conn, &data.username) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body(INVALID_CREDENTIALS),
    };

    match verify(&data.password, &user.password) {
        Ok(true) => {} 
        Ok(false) => return HttpResponse::Unauthorized().body(INVALID_CREDENTIALS), 
        Err(_) => return HttpResponse::InternalServerError().body(SERVER_ERROR), 
    }

    if let Ok(token) = generate_jwt(&user.username) {
        return HttpResponse::Ok()
            .insert_header((AUTHORIZATION, format!("Bearer {}", token)))
            .json(serde_json::json!({ "token": token }));
    }

    HttpResponse::InternalServerError().body(SERVER_ERROR)
}

fn generate_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").expect("The secret key is not set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60)) // 60 minutes
        .expect("Valid timestamps")
        .timestamp() as usize; 

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}


