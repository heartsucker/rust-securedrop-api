//! Error types and utilities.

use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

/// A error message from the API.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub(crate) struct ErrorResponse {
    message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.message, f)
    }
}

/// Wrapper type for all error types found in this crate.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    /// Create a new error.
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Context::new(kind),
        }
    }

    /// Access the `ErrorKind` enum.
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
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
        Error { inner }
    }
}

/// An enum containing the possible error returned by this crate.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Authorization error.
    #[fail(display = "Invalid credentials")]
    AuthError,

    /// Client error. Typically maps to 4xx error codes.
    #[fail(display = "Client error: {}", _0)]
    ClientError(String),

    /// There was an IO error, either with the network or disk.
    #[fail(display = "IO error: {}", _0)]
    IO(String),

    /// There was network error that prevented interaction with the server.
    #[fail(display = "Network error")]
    NetworkError,

    /// Error reserved for bugs in this crate. If is surfaces, please report it.
    #[fail(display = "Programming error (this is a bug): {}", _0)]
    ProgrammingError(String),

    /// Server error. Maps to 5xx error codes.
    #[fail(display = "Internal server error")]
    ServerError,

    /// Something unknown or unexpected happend and there are not enough details to report
    /// meaningfully. This may indicate a bug.
    #[fail(display = "Unknown error")]
    UnknownError,
}
