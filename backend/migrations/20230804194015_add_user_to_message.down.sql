-- Add down migration script here
ALTER TABLE message
DROP COLUMN IF EXISTS user_id,
DROP COLUMN IF EXISTS channel_id;
