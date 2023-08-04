-- Add down migration script here
ALTER TABLE message
DROP CONSTRAINT IF EXISTS fk_user,
DROP CONSTRAINT IF EXISTS fk_channel,
DROP COLUMN IF EXISTS user_id,
DROP COLUMN IF EXISTS channel_id;
