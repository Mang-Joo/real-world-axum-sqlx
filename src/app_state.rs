use std::sync::Arc;
use clap::Parser;
use sqlx::{MySql, Pool};
use crate::db::DbPool;

#[derive(Debug)]
pub struct AppState {
    pub pool: DbPool,
}

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long)]
    pub database_url: String,
    #[arg(long)]
    pub hmac_key: String,
}

impl Config {
    pub fn database_url(&self) -> String {
        String::from(&self.database_url)
    }
}

impl AppState {
    pub fn new(db_pool: DbPool) -> Self {
        AppState {
            pool: db_pool
        }
    }
}