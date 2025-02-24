use diesel::prelude::*;
use crate::models::user::User;
use crate::schema::users::dsl::*; 

pub fn get_user_by_username( conn: &mut PgConnection, username_query: &str, ) 
-> Result<User, diesel::result::Error> 
{
    users
        .filter(username.eq(username_query))
        .first::<User>(conn)
}