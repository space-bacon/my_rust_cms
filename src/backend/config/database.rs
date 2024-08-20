use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::config::Config;

pub fn establish_connection() -> PgConnection {
    let config = Config::from_env();
    PgConnection::establish(&config.database_url)
        .expect(&format!("Error connecting to {}", config.database_url))
}
