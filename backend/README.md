# Fluke backend

- Fluke's backend is written using `rocket.rs` and `sqlx`

***

## Environment Setup

- To tell `sqlx` where to find the database, set the following if a `.env` file
  - `DATABASE_URL=postgres://<db_user>:<db_password>@<db_host>:<db_port>/<db_name>`
    - This needs to be set for `sqlx` to know where to do migrations and also for `sqlx::query!()` and `sqlx::query_as!()` macros to function correctly
      - These macros spin up test DB's and run sql against the DB to validate the SQL and give compiler errors on bad SQL
  - This is loaded in with the help of the [`dotenvy` create](https://docs.rs/dotenvy/0.15.7/dotenvy/)

### SQLx Migrations

- Install `sqlx-cli` with `cargo install sqlx-cli`
- To create a new migration: run `sqlx migrate add -r <migration-name>`
  - Make sure the migration name is useful and each migration is focused to one thing

- Write both a forward and reverse migration
- For more info [see here](https://crates.io/crates/sqlx-cli)

### Helpful Links

[Password hashing cheat-sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
> Websites should not hide which hashing algorithm they use

[RustCrypto password-hashes.rs](https://github.com/RustCrypto/password-hashes)

