-- create_users_table.up
CREATE TABLE IF NOT EXISTS user_profile(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email VARCHAR(254) NOT NULL UNIQUE,
    password TEXT NOT NULL
);