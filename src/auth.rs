//! Authentication data types and utilities.

use chrono::{DateTime, Utc};
use serde::ser::{Serialize, Serializer};
use std::fmt::{self, Display};

/// Wrapper type for know types of credentials.
pub enum Credentials {
    /// Username, password, TOTP.
    UserPassTotp(UserPassTotp),
    /// Username, password, HOTP.
    UserPassHotp(UserPassHotp),
}

impl Serialize for Credentials {
    fn serialize<S>(&self, ser: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Credentials::UserPassTotp(ref c) => c.serialize(ser),
            Credentials::UserPassHotp(ref c) => c.serialize(ser),
        }
    }
}

/// Authentication via username, passphrase, and TOTP.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserPassTotp {
    username: String,
    #[serde(rename = "password")]
    passphrase: String,
    one_time_code: String,
}

impl UserPassTotp {
    /// Construct a new `UserPassTotp` using the given parameters. The OTP value is passed in, and
    /// is reused for each authentication attempt. Because this client will used interactively, we
    /// don't accept the OTP secret and instead use the initial OTP to get an auth token.
    pub fn new(username: String, passphrase: String, one_time_code: String) -> Self {
        Self {
            username,
            passphrase,
            one_time_code,
        }
    }
}

/// Authentication via username, passphrase, and HOTP.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserPassHotp {
    username: String,
    #[serde(rename = "password")]
    passphrase: String,
    one_time_code: String,
}

impl UserPassHotp {
    /// Construct a new `UserPassHotp` using the given parameters. The OTP value is passed in, and
    /// is reused for each authentication attempt. Since we assume a journalist is using a hardware
    /// token, we cant' (and shouldn't) extract the secret.
    pub fn new(username: String, passphrase: String, one_time_code: String) -> Self {
        Self {
            username,
            passphrase,
            one_time_code,
        }
    }
}

impl Into<Credentials> for UserPassHotp {
    fn into(self) -> Credentials {
        Credentials::UserPassHotp(self)
    }
}

impl Into<Credentials> for UserPassTotp {
    fn into(self) -> Credentials {
        Credentials::UserPassTotp(self)
    }
}

/// Wrapper to hold known authorization types.
pub(crate) enum Authorization {
    Credentials(Credentials),
    Token(AuthToken),
}

/// The return value from the API.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct AuthToken {
    token: String,
    expires: DateTime<Utc>,
}

impl Display for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.token, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use json;

    // This is a sanity check beacuse serializing these use different codepaths
    #[test]
    fn serialized_totp_and_hotp_fields_equal() {
        let totp: Credentials =
            UserPassTotp::new("user".into(), "pass".into(), "DEADBEEF".into()).into();
        let hotp: Credentials =
            UserPassHotp::new("user".into(), "pass".into(), "DEADBEEF".into()).into();

        let totp_value = json::to_value(totp).unwrap();
        let totp_value = totp_value.as_object().unwrap();
        let totp_keys: Vec<_> = totp_value.keys().collect();

        let hotp_value = json::to_value(hotp).unwrap();
        let hotp_value = hotp_value.as_object().unwrap();
        let hotp_keys: Vec<_> = hotp_value.keys().collect();

        assert_eq!(totp_keys, hotp_keys);
    }
}
