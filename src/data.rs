//! API request / response data types.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::Result;
use error::ErrorKind;

/// Generic error message.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Response {
    message: String,
}

impl Response {
    /// The human friendly error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Metadata for all sources.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Sources {
    sources: Vec<Source>,
}

impl Sources {
    /// A list of all sources.
    pub fn sources(&self) -> &[Source] {
        &self.sources
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Source {
    uuid: Uuid,
    #[serde(rename = "flagged")]
    is_flagged: bool,
    last_updated: DateTime<Utc>,
    interaction_count: u32,
    journalist_designation: String,
    number_of_documents: u32,
    number_of_messages: u32,
    public_key: String, // TODO better type
}

impl Source {
    /// A unique identifier of the source.
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Boolean field indicating that the source has been flagged.
    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }

    /// Timestamp for when the source was last update.
    pub fn last_updated(&self) -> &DateTime<Utc> {
        &self.last_updated
    }

    /// Number of interactions with the source including number of messages or files a source
    /// submitted and the number of replies to the source.
    pub fn interaction_count(&self) -> u32 {
        self.interaction_count
    }

    /// A `$adjective-$noun` combination used to easily identify a source.
    pub fn journalist_designation(&self) -> &str {
        &self.journalist_designation
    }

    /// Number of documents a source has submitted
    pub fn number_of_documents(&self) -> u32 {
        self.number_of_documents
    }

    /// Number of messages a source has submitted
    pub fn number_of_messages(&self) -> u32 {
        self.number_of_messages
    }

    /// The source's public key as maintained by SecureDrop. Used to encyrpt messages to the
    /// source.
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}

/// Response for the endpoint `GET /api/v1/source/<uuid:uuid:>/submissions`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Submissions {
    submissions: Vec<Submission>,
}

impl Submissions {
    /// A list of all submissions.
    pub fn submissions(&self) -> &[Submission] {
        &self.submissions
    }
}

/// Metadata about a source submission.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Submission {
    filename: String,
    is_read: bool,
    size: u64,
    submission_id: u32,
}

impl Submission {
    /// The SecureDrop filename (e.g., `1-uninteresting_agglutination-msg.gpg`).
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Flag for wheter or not the submission has been read.
    pub fn is_read(&self) -> bool {
        self.is_read
    }

    /// The size of the submission in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// A unique identifier for the submission.
    pub fn submission_id(&self) -> u32 {
        self.submission_id
    }
}

/// A pre-encrypted reply to a source.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Reply {
    reply: String,
}

impl Reply {
    /// Create new `Reply`. This returns `Err` if the message does not appear to be PGP encrypted
    /// and in PEM format.
    pub fn new<S>(reply: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let reply = reply.into();
        if !reply.starts_with("-----BEGIN PGP MESSAGE-----")
            || !reply.ends_with("-----END PGP MESSAGE-----")
        {
            Err(ErrorKind::ClientError("Mesage not PGP encrypted".into()).into())
        } else {
            Ok(Self { reply })
        }
    }
}

/// Information about the current logged in user (journalist).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct User {
    user: UserInner,
}

impl User {
    /// Boolean flag for whether or not the user is a SecureDrop administrator.
    pub fn is_admin(&self) -> bool {
        self.user.is_admin
    }

    /// Timestamp of this user's last login/authentication from either the webapp or the API.
    pub fn last_login(&self) -> &DateTime<Utc> {
        &self.user.last_login
    }

    /// The current user's username.
    pub fn username(&self) -> &str {
        &self.user.username
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct UserInner {
    is_admin: bool,
    last_login: DateTime<Utc>,
    username: String,
}
