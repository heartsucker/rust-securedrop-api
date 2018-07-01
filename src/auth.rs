use oath::{totp_now, HashType};
use serde::ser::{Serialize, Serializer};
use std::fmt::{self, Display};

pub enum Credentials {
    UserPassTotp(UserPassTotp),
}

impl Serialize for Credentials {
    fn serialize<S>(&self, ser: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            &Credentials::UserPassTotp(ref c) => c.clone().now().serialize(ser),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserPassTotp {
    username: String,
    passphrase: String,
    otp_secret: String,
}

impl UserPassTotp {
    pub fn new(username: String, passphrase: String, otp_secret: String) -> Self {
        Self {
            username,
            passphrase,
            otp_secret,
        }
    }

    fn now(self) -> ValidUserPassOtp {
        let otp = totp_now(&self.otp_secret, 6, 0, 30, &HashType::SHA1).unwrap();
        // left pad with zeroes
        let otp = format!("{:0>6}", otp);
        ValidUserPassOtp {
            username: self.username,
            passphrase: self.passphrase,
            one_time_code: otp,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
struct ValidUserPassOtp {
    username: String,
    #[serde(rename = "password")]
    passphrase: String,
    one_time_code: String,
}

impl Into<Credentials> for UserPassTotp {
    fn into(self) -> Credentials {
        Credentials::UserPassTotp(self)
    }
}

pub enum Authorization {
    Token(AuthToken),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AuthToken {
    token: String,
    // TODO expires: i32,
}

impl Display for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.token, f)
    }
}
