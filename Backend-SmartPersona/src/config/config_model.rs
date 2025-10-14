use serde::Deserialize;

// Struct หลักที่รวบรวม Config ทั้งหมด
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: Application,
    pub server: Server,
    pub database: Database,
    pub jwt: Jwt,
    pub services: Services,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub env: String,
    pub name: String,
    pub frontend_url: String,
    pub backend_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
     pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
}

// Struct สำหรับรวบรวมการตั้งค่า JWT ทั้งหมด
#[derive(Debug, Clone, Deserialize)]
pub struct Jwt {
    #[serde(flatten)] // บอกให้ serde มองหา field จาก struct ย่อยโดยตรง
    pub user: JwtSecret,
    #[serde(flatten)]
    pub admin: JwtAdminSecret, // แยก struct admin เพื่อความชัดเจน

    pub access_token_expiration: String,
    pub refresh_token_expiration: String,
}

// ใช้ชื่อกลางๆ ว่า JwtSecret สำหรับ User ทั่วไป
#[derive(Debug, Clone, Deserialize)]
pub struct JwtSecret {
    pub user_secret: String,
    pub user_refresh_secret: String,
}

// แยก struct สำหรับ Admin
#[derive(Debug, Clone, Deserialize)]
pub struct JwtAdminSecret {
    pub admin_secret: String,
    pub admin_refresh_secret: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Services {
    pub ai_service_url: String,
}

// Struct สำหรับรวมการตั้งค่า OAuth

