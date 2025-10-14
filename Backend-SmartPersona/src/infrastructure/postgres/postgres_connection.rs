use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use anyhow::{Context,Result};
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub fn create_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(15) 
        .build(manager)
        .context("Failed to create database connection pool.")?;
        
    Ok(pool)
}