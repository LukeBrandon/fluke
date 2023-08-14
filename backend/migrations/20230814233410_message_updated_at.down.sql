-- Add down migration script here
ALTER TABLE message DROP COLUMN updated_at;
