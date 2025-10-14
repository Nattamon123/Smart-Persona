use std::io::Write;

use chrono::NaiveDateTime;
use diesel::{prelude::*, deserialize::FromSqlRow, expression::AsExpression};
use diesel::pg::Pg;
use uuid::Uuid;
use crate::infrastructure::postgres::schema::{users, sql_types::{UserRole, UserStatus as UserStatusType}};

use diesel_derive_enum::DbEnum;
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)] // ← ต้องมี DbEnum
#[ExistingTypePath = "UserRole"] // เชื่อมกับ ENUM ชื่อ UserRole ใน SQL
pub enum Role {
    PersonaUser,
    CompanyUser,
    Admin,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id : Uuid,
    pub username : String,
    pub password_hash : String,
    pub display_name : Option<String>,
    pub role : Role,    
    // pub status : UserStatus,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
    pub first_name : String,
    pub last_name : String,
}

#[derive(Debug, Clone,Insertable,Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity {
    pub username : String,
    pub password_hash : String,
    pub first_name : String,
    pub last_name : String,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
}