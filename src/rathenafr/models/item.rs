#[derive(Debug, Clone)]
pub struct ItemSearchEntry {
    pub item_id: i64,
    pub aegis_name: String,
    pub display_name: String,
    pub item_type: String,
}

#[derive(Debug, Clone)]
pub struct ItemOwnerEntry {
    pub source: String,
    pub owner_name: String,
    pub account_id: Option<i64>,
    pub amount: i64,
}
