use axum::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

unsafe impl Send for RealClock {}
unsafe impl Sync for RealClock {}

pub struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> DateTime<Utc> {
        chrono::Utc::now()
    }
}