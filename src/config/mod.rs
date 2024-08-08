use error::AppError;

pub mod app_state;
pub mod db;
pub mod di_factory;
pub mod error;
pub mod validate;

pub type RealWorldResult<T> = anyhow::Result<T>;
