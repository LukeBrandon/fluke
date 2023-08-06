-- Add up migration script here
ALTER TABLE message
ADD COLUMN user_id BIGINT NOT NULL,
ADD COLUMN channel_id BIGINT NOT NULL,
ADD CONSTRAINT FOREIGN KEY (user_id) REFERENCES fluke_user(id),
ADD CONSTRAINT FOREIGN KEY (channel_id) REFERENCES channel(id);
