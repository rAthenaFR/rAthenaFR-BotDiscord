#[derive(Debug, Clone)]
pub struct MonsterSearchEntry {
    pub monster_id: i64,
    pub sprite: String,
    pub display_name: String,
    pub level: i32,
    pub hp: i64,
    pub source_table: String,
}

#[derive(Debug, Clone)]
pub struct MonsterDropEntry {
    pub item_id: Option<i64>,
    pub item_name: String,
    pub aegis_name: Option<String>,
    pub server_rate: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct MonsterDrops {
    pub monster_id: i64,
    pub monster_name: String,
    pub drops: Vec<MonsterDropEntry>,
}
