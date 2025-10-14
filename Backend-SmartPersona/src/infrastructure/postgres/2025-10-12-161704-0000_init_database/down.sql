-- This file should undo anything in `up.sql`
-- down.sql

-- ลบตารางทั้งหมด (ในลำดับย้อนกลับจาก up.sql)
DROP TABLE IF EXISTS prompt_templates;
DROP TABLE IF EXISTS generation_jobs;
DROP TABLE IF EXISTS social_connections;
DROP TABLE IF EXISTS profiles;
DROP TABLE IF EXISTS users;

-- ลบประเภทข้อมูล (ENUMs)
DROP TYPE IF EXISTS job_status;
DROP TYPE IF EXISTS profile_status;
DROP TYPE IF EXISTS user_status;
DROP TYPE IF EXISTS user_role;

-- ลบฟังก์ชัน Trigger
DROP FUNCTION IF EXISTS trigger_set_timestamp();

-- ปิดการใช้งาน Extension (ปกติจะไม่ทำถ้ามีตารางอื่นใช้ แต่ใส่ไว้เพื่อความสมบูรณ์)
-- DROP EXTENSION IF EXISTS "pgcrypto";