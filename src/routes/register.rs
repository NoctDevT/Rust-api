
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::register_model::{RegisterRequest, RegisterResponse}; 
use crate::models::user::NewUser;
use crate::schema::users;
use crate::services::db_service::get_user_by_username;

    //add to Reduce unncessary heap allocation
const SERVER_ERROR: &str = "Server error, please try again later"; 

pub async fn register_user(
    pool: web::Data<DbPool>,
    data: web::Json<RegisterRequest>,
) -> impl Responder {

        // let mut conn = &mut pool.get().expect(SERVER_ERROR);
        //Gracefully handling db failure & using a mut conn due to diesel operations modifying internal state 
        // Need to deference conn (naturally pooledCollection<connectionManager<Dbconnection> to <Dbconnection > only)
        //  using *conn dereferences it into pgreference allowing &mut *conn to be a referencable DbConnection 
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body(SERVER_ERROR),
    };
    
    let user = match get_user_by_username(&mut *conn, &data.username) {
        Ok(user) => return HttpResponse::Unauthorized().body("You already have an account"),
        Err(_) => {},
    };


    let hashed_password = match hash(&data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json(SERVER_ERROR),
    };

    let new_user = NewUser {
        username: &data.username,
        password: &hashed_password,
        name: data.name.as_deref(),
        description: &data.description,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut *conn) 
    {
        Ok(_) => HttpResponse::Created().json(RegisterResponse {
            message: "The account has been registered".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().body(SERVER_ERROR),
    }

}