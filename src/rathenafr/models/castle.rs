#[derive(Debug, Clone)]
pub struct CastleSummary {
    pub castle_id: i32,
    pub owner_name: Option<String>,
    pub economy: i32,
    pub defense: i32,
    pub visible_c: i32,
}

#[derive(Debug, Clone)]
pub struct CastleDetails {
    pub castle_id: i32,
    pub owner_guild_id: i64,
    pub owner_name: Option<String>,
    pub economy: i32,
    pub defense: i32,
    pub trigger_e: i32,
    pub trigger_d: i32,
    pub next_time: i64,
    pub pay_time: i64,
    pub create_time: i64,
    pub visible_c: i32,
}
