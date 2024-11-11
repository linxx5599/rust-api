use dotenvy::dotenv;
use mysql::*;
use std::env;

pub struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    pass: String,
    db: String,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, env::VarError> {
        dotenv().ok();
        let host = env::var("DATABASE_HOME")?;
        let port = env::var("DATABASE_PORT")?.parse::<u16>().unwrap();
        let user = env::var("DATABASE_USER")?;
        let pass = env::var("DATABASE_PASSWORD")?;
        let db = env::var("DATABASE_DB")?;

        Ok(DatabaseConfig {
            host,
            port,
            user,
            pass,
            db,
        })
    }

    pub fn create_pool(&self) -> Result<Pool, mysql::Error> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(&self.host))
            .tcp_port(self.port)
            .user(Some(&self.user))
            .pass(Some(&self.pass))
            .db_name(Some(&self.db));
        Pool::new(opts)
    }
}
