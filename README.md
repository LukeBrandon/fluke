# Fluke
### Fluke is a discord-like app written in Rust

Files are served from backup/static, to build files run 
`cd frontend`
`trunk build --release` 
or run `dev/serve.sh`

### Stack
- [yew.rs](https://yew.rs/)
- [rocket.rs](https://rocket.rs/)
- [sqlx.rs](https://docs.rs/sqlx/0.6.3/sqlx/index.html)

### TODO

- [ ] HTTPS for production
- [ ] Password security 
- [ ] Validation for user inputs 
- [ ] Rate limiting routes
- [ ] Authentication (2FA, JWT) 
- [ ] Cookies 
