use clap::Parser;

#[derive(Debug, Clone, serde::Deserialize, clap::Parser)]
pub struct Config {
    /// The connection URL for the application's database
    #[clap(long, env, default_value = "postgresql://localhost/newsletter")]
    pub database_url: String,
    /// The port number to run the application on
    #[clap(long, env, default_value = "3000")]
    pub port: u16,
}

pub fn get_config() -> Config {
    Config::parse()
}
