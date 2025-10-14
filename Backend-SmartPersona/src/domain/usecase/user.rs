use std::sync::Arc;

use crate::{domain::{entities::user, repo::user::UserRepository, value_object::user::RegisterUserModel}, infrastructure::hashingpassword};
use anyhow::{Ok, Result};
use uuid::Uuid;
pub struct UserUseCase<T>
where
    T: UserRepository + Send + Sync,
{
    user_repository: Arc<T>,
}
impl<T> UserUseCase<T>
where
    T: UserRepository + Send + Sync,
{
    pub fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }
    pub async fn register(&self, mut register_user_model: RegisterUserModel) -> anyhow::Result<Uuid> {
        let hashed_password = hashingpassword::hash(register_user_model.password.clone())?;
    
        register_user_model.password = hashed_password;
        let register_user_entity = register_user_model.to_entity();
        let user_id = self.user_repository.register(register_user_entity).await?;
        Ok(user_id)
    }
}
