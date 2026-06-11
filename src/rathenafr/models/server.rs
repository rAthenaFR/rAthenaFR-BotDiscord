#[derive(Debug, Clone)]
pub struct DatabaseStatus {
    pub database_name: String,
    pub database_engine: String,
    pub online_characters: i64,
    pub characters: i64,
    pub accounts: i64,
    pub guilds: i64,
}
