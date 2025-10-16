-- ================================
-- 1. ลบ trigger จาก prompt_templates
-- ================================
DROP TRIGGER IF EXISTS set_timestamp ON prompt_templates;

-- ================================
-- 2. ลบตาราง prompt_templates
-- ================================
DROP TABLE IF EXISTS prompt_templates;

-- ================================
-- 3. ลบตาราง generation_jobs
-- ================================
DROP TABLE IF EXISTS generation_jobs;

-- ================================
-- 4. ลบตาราง social_connections
-- ================================
DROP TABLE IF EXISTS social_connections;

-- ================================
-- 5. ลบ trigger จาก profiles
-- ================================
DROP TRIGGER IF EXISTS set_timestamp ON profiles;

-- ================================
-- 6. ลบตาราง profiles
-- ================================
DROP TABLE IF EXISTS profiles;

-- ================================
-- 7. ลบ trigger จาก users
-- ================================
DROP TRIGGER IF EXISTS set_timestamp ON users;

-- ================================
-- 8. ลบตาราง users
-- ================================
DROP TABLE IF EXISTS users;

-- ================================
-- 9. ลบ ENUMs
-- ================================
DROP TYPE IF EXISTS user_role;
DROP TYPE IF EXISTS user_status;
DROP TYPE IF EXISTS profile_status;
DROP TYPE IF EXISTS job_status;

-- ================================
-- 10. ลบฟังก์ชัน trigger_set_timestamp
-- ================================
DROP FUNCTION IF EXISTS trigger_set_timestamp();

-- ================================
-- 11. ลบ extension UUID
-- ================================
DROP EXTENSION IF EXISTS "pgcrypto";
