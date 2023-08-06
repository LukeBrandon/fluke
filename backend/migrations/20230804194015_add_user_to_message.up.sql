ALTER TABLE message
ADD COLUMN user_id BIGINT NOT NULL,
ADD COLUMN channel_id BIGINT NOT NULL;

ALTER TABLE message
ADD CONSTRAINT fk_message_user_id
FOREIGN KEY (user_id) REFERENCES fluke_user(id);

ALTER TABLE message
ADD CONSTRAINT fk_message_channel_id
FOREIGN KEY (channel_id) REFERENCES channel(id);
