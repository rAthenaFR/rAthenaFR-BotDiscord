#[derive(Debug, Clone)]
pub struct PlayerProfile {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub map: String,
    pub zeny: i64,
    pub guild_name: Option<String>,
}
