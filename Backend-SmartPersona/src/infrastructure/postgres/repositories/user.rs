use std::sync::Arc;
use uuid::Uuid;
use axum::async_trait;
use anyhow::{Ok, Result};
use diesel::{dsl::insert_into, prelude::*};

use crate::{domain::{entities::user::{RegisterUserEntity, UserEntity}, repo::user::UserRepository}, infrastructure::postgres::{postgres_connection::DbPool, schema::users}};

pub struct UserPostgres{
    db_pool:Arc<DbPool>
}
impl UserPostgres {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}
#[async_trait]
impl UserRepository for UserPostgres {
        async fn register(&self,register_user_entity:RegisterUserEntity) -> Result<Uuid>{
            let mut conn = Arc::clone(&self.db_pool).get()?;
            let result = insert_into(users::table)
                 .values(register_user_entity)
                 .returning(users::id)
                 .get_result::<Uuid>(&mut conn)?;
                Ok(result)
        }
    async fn find_by_username(&self,username:String) -> Result<UserEntity>{
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = users::table
        .filter(users::username.eq(username))
        .select(UserEntity::as_select())
        .first::<UserEntity>(&mut conn)?;
        Ok(result)
    }
}