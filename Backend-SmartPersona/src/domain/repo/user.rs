use crate::domain::{entities::user::{UserEntity,RegisterUserEntity}};
use anyhow::Result;
use axum::async_trait;
use uuid::Uuid;
#[async_trait]
pub trait UserRepository {
    async fn register(&self,register_user_entity:RegisterUserEntity) -> Result<Uuid>;
    async fn find_by_username(&self,username:String) -> Result<UserEntity>;
}