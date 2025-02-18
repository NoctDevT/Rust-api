mod middleware;
mod models;
mod routes;
mod db; 
mod services;
pub mod schema;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use middleware::auth_middlewares::AuthMiddleware;
use dotenv::dotenv;
use routes::login::{user_login};
use crate::db::establish_connection;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::services::survey_ws::{survey_ws_route, Sessions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    //shared session storage for websockets 
    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));
    //db pool con 
    let pool = establish_connection();
    //so move moves the session and pool within its scope
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(sessions.clone()))  
        .app_data(web::Data::new(pool.clone()))      
        .wrap(Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_headers(vec!["Authorization"]))
            .route("/login", web::post().to(user_login)) 
            .route("/ws/survey", web::get().to(survey_ws_route))
            .service(
                web::scope("")
                    .wrap(AuthMiddleware)
                    .route("/", web::get().to(hello))
                    .route("/greet/{name}", web::get().to(greet_user))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn greet_user(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}