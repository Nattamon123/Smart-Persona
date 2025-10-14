-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN first_name,
DROP COLUMN last_name;