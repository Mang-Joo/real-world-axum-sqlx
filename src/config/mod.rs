pub mod app_state;
pub mod error;
pub mod validate;
pub mod db;

pub type Result<T> = anyhow::Result<T>;
