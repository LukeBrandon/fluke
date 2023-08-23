const DEFAULT_PORT: u16 = 8000;

#[derive(Clone)]
pub struct FlukePort(pub u16);

impl Default for FlukePort {
    fn default() -> Self {
        FlukePort(DEFAULT_PORT)
    }
}

impl From<u16> for FlukePort {
    fn from(value: u16) -> Self {
        FlukePort(value)
    }
}

#[derive(Clone)]
pub struct FlukeConfig {
    /// REQUIRED! There is not a default value
    pub database_url: String,
    /// Default 8000
    pub port: FlukePort,
    pub hmac_key: String,
}

impl Default for FlukeConfig {
    fn default() -> Self {
        FlukeConfig {
            database_url: "".to_string(),
            port: FlukePort::default(),
            hmac_key: "".to_string(),
        }
    }
}
