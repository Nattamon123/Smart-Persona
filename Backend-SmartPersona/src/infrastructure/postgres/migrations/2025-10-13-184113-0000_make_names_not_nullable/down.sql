-- This file should undo anything in `up.sql`
ALTER TABLE users
ALTER COLUMN first_name DROP NOT NULL,
ALTER COLUMN last_name DROP NOT NULL;