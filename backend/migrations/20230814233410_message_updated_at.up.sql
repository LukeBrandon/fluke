-- Add up migration script here
ALTER TABLE message ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
