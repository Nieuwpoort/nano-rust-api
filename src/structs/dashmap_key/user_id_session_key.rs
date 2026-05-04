#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserIdSessionKey {
    pub user_id: i64,
    pub session_key: String,
}