use ethers_core::types::U256;
use eyre::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, Sqlite, SqliteConnection,
};
use std::str::FromStr;

use crate::{config::DatabaseConfig, hero};

// pub trait Connection {}
// impl<T: sqlx::Connection> Connection for T {}

pub async fn connect_sqlite(cfg: &DatabaseConfig) -> Result<SqliteConnection> {
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

    Ok(conn)
}

pub async fn save_hero<'a>(conn: &mut SqliteConnection, hero: &hero::Hero) -> Result<()> {
    let hero = hero.clone();
    let id = hero.id.to_string();
    let visual_genes = hero.visual_genes.to_string();
    let stat_genes = hero.stat_genes.to_string();
    let xp = hero.xp.to_string();
    let q = sqlx::query_file!(
        "db/query/insert_hero.sql",
        id,
        visual_genes,
        stat_genes,
        hero.first_name,
        hero.last_name,
        hero.strength,
        hero.dexterity,
        hero.agility,
        hero.vitality,
        hero.endurance,
        hero.intelligence,
        hero.wisdom,
        hero.luck,
        xp,
        hero.level
    );

    q.execute(conn).await?;

    Ok(())
}
