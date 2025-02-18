
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify}; 
use chrono::{Utc, Duration};
use std::env;
use jsonwebtoken::{encode, EncodingKey, Header};
use actix_web::http::header::{AUTHORIZATION};
// use actix_web::cookie::{Cookie, SameSite};

use crate::models::login_model::LoginRequest;
use crate::models::claims::Claims;


pub async fn user_login(data: web::Json<LoginRequest>) -> impl Responder {
    let mock_username = "test_user";
    let mock_salt = "random_salt";
    let mock_hashed_password = hash_with_salt("password", mock_salt);

    if data.username != mock_username {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    let salted_password = format!("{}{}", data.password, mock_salt);
    match verify(&salted_password, &mock_hashed_password) {
        Ok(true) => {
            match generate_jwt(&data.username){
                Ok(token) => {
                    HttpResponse::Ok()
                        .insert_header((AUTHORIZATION, format!("Bearer {}", token))) // ✅ Correct way to set Authorization header
                        .json(serde_json::json!({ "token": token }))
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
            }
        }
        // Ok(true) => {
        //     match generate_jwt(&data.username){
        //         // Ok(token) => HttpResponse::Ok()
        //         //     .insert_header(("Authorization", format!("Bearer {}", token)))
        //         //     .json(serde_json::json!({ "token": token })),
        //         // Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
        //         Ok(token) => {
        //             let mut response = HttpResponse::Ok().json(serde_json::json!({ "token": token }));
        //             response.headers_mut().insert(
        //                 AUTHORIZATION,
        //                 HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        //             );
        //             println!("✅ Final Response Headers: {:?}", response.headers()); 
        //             response
        //         }
        //         Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
        //         // Ok(token) => {
        //         //     let cookie = Cookie::build("auth_token", token.clone())
        //         //         .http_only(true)  // ❌ Prevent JavaScript access (prevents XSS)
        //         //         .secure(false)  // ✅ Send over HTTPS only (change to false in dev)
        //         //         .same_site(SameSite::Strict)  // ✅ Prevent CSRF (use Lax if needed)
        //         //         .path("/")
        //         //         .finish();

        //         //     HttpResponse::Ok()
        //         //         .cookie(cookie) // ✅ Set cookie in response
        //         //         .json(serde_json::json!({ "message": "Login successful" }))
        //         // }
        //         // Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),
        //     }
        // }
        _ => HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

fn hash_with_salt(password: &str, salt: &str) -> String {
    let salted_password = format!("{}{}", password, salt);
    hash(salted_password, 4).expect("Failed to hash password")
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


