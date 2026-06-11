#[derive(Debug, Clone)]
pub struct RankingEntry {
    pub rank: usize,
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub map: String,
}

#[derive(Debug, Clone)]
pub struct ZenyRankingEntry {
    pub rank: usize,
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub zeny: i64,
}

#[derive(Debug, Clone)]
pub struct MapStatsEntry {
    pub rank: usize,
    pub map: String,
    pub characters: i64,
    pub online_characters: i64,
}
