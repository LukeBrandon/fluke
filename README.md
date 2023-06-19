# Fluke
### Fluke is a discord-like app written in Rust

Make sure you set: `rustup target add wasm32-unknown-unknown`

Files are served from backup/static, to build files run 
`cd frontend`
`trunk build --release` 
or run `dev/serve.sh`

### Stack
- [yew.rs](https://yew.rs/)
- [rocket.rs](https://rocket.rs/)
- [sqlx.rs](https://docs.rs/sqlx/0.6.3/sqlx/index.html)
