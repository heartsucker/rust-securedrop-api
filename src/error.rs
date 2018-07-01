use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.message, f)
    }
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Context::new(kind),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Invalid credentials")]
    AuthError,

    #[fail(display = "Client error: {}", _0)]
    ClientError(ErrorResponse),

    #[fail(display = "IO error: {}", _0)]
    IO(String),

    #[fail(display = "Network error")]
    NetworkError,

    #[fail(display = "Programming error (this is a bug): {}", _0)]
    ProgrammingError(String),

    #[fail(display = "Internal server error")]
    ServerError,

    #[fail(display = "Unknown error")]
    UnknownError,
}
