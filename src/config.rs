#[derive(Debug, Clone, Deserialize)]

pub struct Config {
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub twilio_phone_number: String,
    pub parent_phone_number: String,
}