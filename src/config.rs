#[derive(Debug)]
pub struct Config {
    database_url: String,
}

impl Config {
    pub fn new(database_url: String) -> Config {
        Config { database_url }
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}