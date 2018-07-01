/// Generic error message.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Response {
    message: String,
}

impl Response {
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Response for the endpoint `GET /api/v1/source`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Sources {
    sources: Vec<Source>,
}

impl Sources {
    pub fn sources(&self) -> &[Source] {
        &self.sources
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Source {
    filesystem_id: String,
    #[serde(rename = "flagged")]
    is_flagged: bool,
    last_updated: String, // TODO datetime
    interaction_count: u32,
    journalist_designation: String,
    number_of_documents: u32,
    number_of_messages: u32,
    public_key: String, // TODO better type
    source_id: u32,
}

impl Source {
    pub fn filesystem_id(&self) -> &str {
        &self.filesystem_id
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }

    pub fn last_updated(&self) -> &str {
        &self.last_updated
    }

    pub fn interaction_count(&self) -> u32 {
        self.interaction_count
    }

    pub fn journalist_designation(&self) -> &str {
        &self.journalist_designation
    }

    pub fn number_of_documents(&self) -> u32 {
        self.number_of_documents
    }

    pub fn number_of_messages(&self) -> u32 {
        self.number_of_messages
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn source_id(&self) -> u32 {
        self.source_id
    }
}

/// Response for the endpoint `GET /api/v1/source/<str:filesystem_id>/submissions`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Submissions {
    submissions: Vec<Submission>,
}

impl Submissions {
    pub fn submissions(&self) -> &[Submission] {
        &self.submissions
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Submission {
    filename: String,
    is_read: bool,
    size: u64,
    submission_id: u32,
}

impl Submission {
    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn is_read(&self) -> bool {
        self.is_read
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn submission_id(&self) -> u32 {
        self.submission_id
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Reply {
    reply: String,
}

impl Reply {
    pub fn new<S>(reply: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            reply: reply.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct User {
    user: UserInner,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.user.is_admin
    }

    pub fn last_login(&self) -> &str {
        &self.user.last_login
    }

    pub fn username(&self) -> &str {
        &self.user.username
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct UserInner {
    is_admin: bool,
    last_login: String, // TODO datetime
    username: String,
}
