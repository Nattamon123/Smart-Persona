// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_status"))]
    pub struct JobStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "profile_status"))]
    pub struct ProfileStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JobStatus;

    generation_jobs (id) {
        id -> Uuid,
        requester_id -> Uuid,
        status -> JobStatus,
        prompt -> Nullable<Text>,
        result -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProfileStatus;

    profiles (id) {
        id -> Uuid,
        owner_id -> Uuid,
        status -> ProfileStatus,
        content -> Nullable<Jsonb>,
        layout_config -> Nullable<Jsonb>,
        #[max_length = 100]
        shareable_link_slug -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    prompt_templates (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        template_text -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    social_connections (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 50]
        platform -> Varchar,
        #[max_length = 255]
        platform_user_id -> Varchar,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;
    use super::sql_types::UserStatus;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        role -> UserRole,
        status -> UserStatus,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
    }
}

diesel::joinable!(generation_jobs -> users (requester_id));
diesel::joinable!(profiles -> users (owner_id));
diesel::joinable!(social_connections -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    generation_jobs,
    profiles,
    prompt_templates,
    social_connections,
    users,
);
