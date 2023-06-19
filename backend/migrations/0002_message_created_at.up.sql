-- Add up migration script here
ALTER TABLE message ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();