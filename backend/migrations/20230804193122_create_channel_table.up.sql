-- Add up migration script here
CREATE TABLE channel (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
