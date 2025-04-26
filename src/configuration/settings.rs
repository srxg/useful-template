
#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: String,
    pub redis_url: String,
    #[allow(dead_code)]
    pub smtp_host: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    pub ip: String,
    pub port: u16,
}
