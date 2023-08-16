-- Add down migration script here
ALTER TABLE message DROP CONSTRAINT fk_message_channel_id;


-- This is the state of the FK constraint before this migration
ALTER TABLE message
ADD CONSTRAINT fk_message_channel_id
FOREIGN KEY (channel_id) REFERENCES channel(id);
