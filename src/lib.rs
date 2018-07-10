//! # SecureDrop API
//!
//! This crate provides and interface to the experimental SecureDrop API.
//!
//! ## Quick Start
//!
//! ```no_run
//! # extern crate securedrop_api;
//! use securedrop_api::auth::UserPassTotp;
//! use securedrop_api::data::Reply;
//! use securedrop_api::{Client, Result};
//!
//! fn reply_to_source() -> Result<()> {
//!     let creds = UserPassTotp::new(
//!         "journalist".into(),
//!         "WEjwn8ZyczDhQSK24YKM8C9a".into(),
//!         "123123".into(),
//!     );
//!
//!     let client = Client::new(
//!         "http://localhost:8081".parse().unwrap(),
//!         creds,
//!         Some("MyApp/0.1.0".to_string()))?;
//!
//!     // Initialize client / authorize user
//!     let user = client.user()?;
//!
//!     // Get all sources
//!     let sources = client.sources()?;
//!
//!     // Get one source
//!     let source = client.source(sources.sources()[0].uuid())?;
//!
//!     // List submissions and download
//!     let submissions = client.source_submissions(source.uuid())?;
//!     let mut buf = Vec::new();
//!     client.download_submission(
//!         source.uuid(),
//!         submissions.submissions()[0].submission_id(),
//!         &mut buf,
//!     )?;
//!
//!     // Send a reply
//!     let reply_str =
//!         "-----BEGIN PGP MESSAGE-----\nshould be encrypted :(\n-----END PGP MESSAGE-----";
//!     let reply = Reply::new(reply_str)?;
//!     client.reply_to_source(source.uuid(), &reply)?;
//!     Ok(())
//! }
//! ```

extern crate chrono;
#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate uuid;

pub mod auth;
pub mod client;
pub mod data;
pub mod error;

pub use client::Client;
pub use error::{Error, ErrorKind};

/// Alias for `Result<T, Error>`.
pub type Result<T> = ::std::result::Result<T, Error>;
