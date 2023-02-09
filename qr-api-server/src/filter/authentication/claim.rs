use qr_domain::{email::EmailAddress, user::UserId};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub email: Option<EmailAddress>,
    pub email_verified: bool,
    pub phone_number: Option<String>,
    pub name: Option<String>,
    pub sub: UserId,
}
