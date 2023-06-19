-- create_users_table.up
-- Prefer TEXT > VARCHAR: https://wiki.postgresql.org/wiki/Don%27t_Do_This#Don.27t_use_varchar.28n.29_by_defaultj
CREATE TABLE IF NOT EXISTS user(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email VARCHAR(254) NOT NULL UNIQUE,
    password TEXT NOT NULL,
);