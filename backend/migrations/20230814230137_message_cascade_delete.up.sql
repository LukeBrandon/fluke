-- Add up migration script here
ALTER TABLE message DROP CONSTRAINT fk_message_channel_id;

ALTER TABLE message
ADD CONSTRAINT fk_message_channel_id
    FOREIGN KEY (channel_id) 
    REFERENCES channel(id)
    ON DELETE CASCADE;
