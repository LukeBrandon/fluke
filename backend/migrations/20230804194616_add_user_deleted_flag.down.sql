-- Add down migration script here
ALTER TABLE fluke_user
DROP COLUMN IF EXISTS deleted;
