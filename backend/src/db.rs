use sqlx::{Pool, Postgres};
use dotenv::dotenv;
use std::env;

pub async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    Pool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
