#[derive(Debug, Clone)]
pub struct AccountCharacterSummary {
    pub slot: i32,
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub map: String,
    pub zeny: i64,
    pub guild_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AccountStatus {
    pub account_id: i64,
    pub userid: String,
    pub sex: String,
    pub group_id: i32,
    pub state: i64,
    pub unban_time: i64,
    pub expiration_time: i64,
    pub logincount: i64,
    pub character_slots: i32,
    pub characters: i64,
    pub online_characters: i64,
    pub total_zeny: i64,
    pub lastlogin: Option<String>,
}

pub const ACCOUNT_STATE_ACTIVE: i64 = 0;
pub const ACCOUNT_STATE_BLOCKED: i64 = 5;
pub const ACCOUNT_DEFAULT_GROUP_ID: i64 = 0;
pub const ACCOUNT_NO_UNBAN_TIME: i64 = 0;
pub const ACCOUNT_NO_EXPIRATION_TIME: i64 = 0;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountManageField {
    GroupId,
    State,
    UnbanTime,
    ExpirationTime,
    Logincount,
    Sex,
}

impl AccountManageField {
    pub const fn name(self) -> &'static str {
        match self {
            Self::GroupId => "group_id",
            Self::State => "state",
            Self::UnbanTime => "unban_time",
            Self::ExpirationTime => "expiration_time",
            Self::Logincount => "logincount",
            Self::Sex => "sex",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreatedAccount {
    pub account_id: i64,
    pub userid: String,
    pub sex: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct BanEntry {
    pub account_id: i64,
    pub userid: String,
    pub group_id: i32,
    pub state: i64,
    pub unban_time: i64,
    pub expiration_time: i64,
    pub lastlogin: Option<String>,
    pub characters: i64,
}
