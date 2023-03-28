use clap::Parser;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, Connection, PgConnection, PgPool};
use url::Url;

#[derive(Debug, Clone, serde::Deserialize, clap::Parser)]
pub struct Config {
    /// The connection URL for the application's database
    #[clap(long, env, default_value = "postgresql://localhost/newsletter")]
    pub database_url: Secret<String>,
    /// The port number to run the application on
    #[clap(long, env, default_value = "3000")]
    pub port: u16,
}

pub struct TestingDatabase {
    pub pool: PgPool,
    pub db_name: String,
}

impl TestingDatabase {
    fn new(pool: PgPool, db_name: String) -> Self {
        Self { pool, db_name }
    }
}

impl Drop for TestingDatabase {
    fn drop(&mut self) {
        let name = self.db_name.clone();
        tokio::spawn(async move {
            let mut connection = PgConnection::connect(&get_config().database_url.expose_secret())
                .await
                .unwrap();
            sqlx::query(&format!(r#"DROP DATABASE "{}""#, &name))
                .execute(&mut connection)
                .await
                .unwrap();
        });
    }
}

impl Config {
    /// Creates a connection pool to the application's database
    pub async fn db(&self) -> PgPool {
        PgPoolOptions::new()
            .max_connections(20)
            .connect(&self.database_url.expose_secret())
            .await
            .unwrap()
    }

    /// Creates a temporary database for testing purposes
    ///
    /// The database is created with a random name and dropped when the connection is closed.
    pub async fn test_db(&self) -> TestingDatabase {
        let mut connection = PgConnection::connect(&self.database_url.expose_secret())
            .await
            .unwrap();
        let mut url = Url::parse(&self.database_url.expose_secret()).unwrap();
        let db_name: String =
            sqlx::query_scalar("SELECT current_database() || '_' || (random() * 100000)::int")
                .fetch_one(&mut connection)
                .await
                .unwrap();
        url.set_path(&db_name);

        sqlx::query(&format!(r#"CREATE DATABASE "{}""#, &db_name))
            .execute(&mut connection)
            .await
            .unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url.as_str())
            .await
            .unwrap();
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate the database");
        TestingDatabase::new(pool, db_name)
    }
}

pub fn get_config() -> Config {
    Config::parse()
}
