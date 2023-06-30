-- Add up migration script here
CREATE TABLE IF NOT EXISTS message (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    message TEXT NOT NULL
);