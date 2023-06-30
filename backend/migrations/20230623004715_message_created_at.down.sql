-- Add down migration script here
ALTER TABLE message DROP COLUMN IF EXISTS created_at;