#[derive(Debug, Clone)]
pub struct GuildSummary {
    pub name: String,
    pub master: String,
    pub level: i32,
    pub members: i64,
    pub online_members: i32,
    pub max_members: i32,
}

#[derive(Debug, Clone)]
pub struct GuildDetails {
    pub name: String,
    pub master: String,
    pub level: i32,
    pub members: i64,
    pub online_members: i32,
    pub max_members: i32,
    pub average_level: i32,
    pub exp: i64,
    pub next_exp: i64,
}

#[derive(Debug, Clone)]
pub struct GuildMemberSummary {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub position: i32,
    pub map: String,
}
