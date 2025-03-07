use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("✅ Database connection pool successfully created.");

    pool

}


