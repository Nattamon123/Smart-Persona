-- This file should undo anything in `up.sql`
-- down.sql
ALTER TABLE users RENAME COLUMN username TO email;