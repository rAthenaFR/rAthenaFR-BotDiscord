#[derive(Debug, Clone)]
pub struct CharacterSummary {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub map: String,
}

#[derive(Debug, Clone)]
pub struct CharacterQuestEntry {
    pub quest_id: i64,
    pub state: String,
    pub time: i64,
    pub count1: i64,
    pub count2: i64,
    pub count3: i64,
}

#[derive(Debug, Clone)]
pub struct CharacterItemEntry {
    pub item_id: i64,
    pub amount: i64,
    pub equip: i64,
    pub refine: i32,
    pub identify: bool,
    pub bound: i32,
    pub unique_id: i64,
    pub enchant_grade: i32,
    pub card0: i64,
    pub card1: i64,
    pub card2: i64,
    pub card3: i64,
}
