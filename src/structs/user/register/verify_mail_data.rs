use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct VerifyMailData {
    pub verify_mail_token: String,
    pub created_at: NaiveDateTime,
}