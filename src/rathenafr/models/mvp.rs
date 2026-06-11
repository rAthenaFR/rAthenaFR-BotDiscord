#[derive(Debug, Clone)]
pub struct MvpKillEntry {
    pub mvp_date: Option<String>,
    pub mvp_timestamp: Option<i64>,
    pub killer_id: i64,
    pub killer_name: String,
    pub monster_id: i64,
    pub monster_name: String,
    pub monster_aegis_name: Option<String>,
    pub map: String,
    pub mvp_exp: Option<i64>,
    pub prize_id: i64,
    pub prize_name: String,
    pub prize_aegis_name: Option<String>,
}
