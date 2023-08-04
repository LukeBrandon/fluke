-- Add up migration script here
ALTER TABLE fluke_user
ADD COLUMN deleted BOOLEAN DEFAULT FALSE;
