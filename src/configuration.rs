#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration.yaml"))?;
    settings.try_deserialize()
}
