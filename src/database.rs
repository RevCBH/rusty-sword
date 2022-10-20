use eyre::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions,
};
use std::str::FromStr;

use crate::config::DatabaseConfig;

// pub trait Connection {}
// impl<T: sqlx::Connection> Connection for T {}

pub async fn connect(cfg: &DatabaseConfig) -> Result<Box<dyn sqlx::Connection>> {
    let conn = match cfg {
        DatabaseConfig::SQLite { file } => {
            SqliteConnectOptions::from_str(file.as_str())?
                .journal_mode(SqliteJournalMode::Wal)
                .create_if_missing(true)
                .read_only(false)
                .connect()
                .await?
        }
    };

    Ok(Box::new(conn))
}
