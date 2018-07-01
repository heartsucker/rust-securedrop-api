#[macro_use]
extern crate failure;
extern crate oath;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;

pub mod auth;
pub mod client;
pub mod data;
pub mod error;

pub use client::Client;
pub use error::{Error, ErrorKind};

pub type Result<T> = ::std::result::Result<T, Error>;
