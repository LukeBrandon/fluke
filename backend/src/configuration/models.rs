const DEFAULT_PORT: u16 = 8000;

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

pub struct FlukeConfiguration {
    /// REQUIRED! There is not default value
    pub database_url: String,
    /// Default 8000
    pub port: FlukePort,
}

impl Default for FlukeConfiguration {
    fn default() -> Self {
        FlukeConfiguration {
            database_url: "".to_string(),
            port: FlukePort::default(),
        }
    }
}
