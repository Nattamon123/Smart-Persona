-- Your SQL goes here
-- up.sql

-- เปิดใช้งาน Extension สำหรับสร้าง UUID
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- สร้างฟังก์ชันสำหรับอัปเดตคอลัมน์ updated_at โดยอัตโนมัติ
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- สร้างประเภทข้อมูล (ENUMs) สำหรับใช้ในตารางต่างๆ
CREATE TYPE user_role AS ENUM ('persona_user', 'company_user', 'admin');
CREATE TYPE user_status AS ENUM ('pending', 'active', 'suspended');
CREATE TYPE profile_status AS ENUM ('public', 'private');
CREATE TYPE job_status AS ENUM ('pending', 'completed', 'failed');

-- 1. ตารางผู้ใช้งาน (Users)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    role user_role NOT NULL DEFAULT 'persona_user',
    status user_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- 2. ตารางโปรไฟล์ (Profiles)
CREATE TABLE profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status profile_status NOT NULL DEFAULT 'private',
    -- ใช้ JSONB เพื่อเก็บข้อมูลที่มีโครงสร้างซับซ้อน เช่น ประวัติการทำงาน, การศึกษา
    content JSONB,
    layout_config JSONB,
    -- slug คือชื่อที่แสดงใน URL เช่น /p/john-doe
    shareable_link_slug VARCHAR(100) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON profiles
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
-- สร้าง Index เพื่อให้ค้นหาด้วย owner_id ได้เร็วขึ้น
CREATE INDEX idx_profiles_owner_id ON profiles(owner_id);

-- 3. ตารางการเชื่อมต่อโซเชียลมีเดีย (Social Connections)
CREATE TABLE social_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform VARCHAR(50) NOT NULL,
    platform_user_id VARCHAR(255) NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, platform) -- กำหนดให้ 1 user เชื่อมต่อ 1 platform ได้แค่ครั้งเดียว
);

-- 4. ตารางสำหรับ AI Generation Jobs
CREATE TABLE generation_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    requester_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status job_status NOT NULL DEFAULT 'pending',
    prompt TEXT,
    result JSONB, -- เก็บผลลัพธ์ที่ได้จาก AI
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- 5. ตารางสำหรับ Prompt Templates (จัดการโดย Admin)
CREATE TABLE prompt_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    template_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON prompt_templates
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();