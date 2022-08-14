//! The Rust client for `procession`.
//!
//! # Versioning
//!
//! This library follows semantic versioning releases.
//!
//! # Compatibility
//!
//! The `procession` server supports all previous versions of client libraries:
//! it will never introduce a breaking change.
//!
//! However, newer libraries may be able to offer new features.
//! It's generally recommended to use the newest version of the library you're able.

#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

use std::ops::Deref;

use bytes::Bytes;
use reqwest::{Response, StatusCode, Url};
use thiserror::Error;

/// Errors encountered by the [`Client`].
#[derive(Error, Debug)]
pub enum Error {
    /// The client wasn't able to parse the provided URL.
    #[error("parse url")]
    ParseURL(#[from] url::ParseError),

    /// An error issued by the underlying transport.
    #[error("transport")]
    Transport(#[from] reqwest::Error),

    /// The server returned an unexpected status code.
    #[error("'{route}': status code '{expect}' expected, but got '{got}'; body:\n{body}")]
    StatusCode {
        /// The route that was requested.
        route: String,
        /// The expected status code.
        expect: StatusCode,
        /// The status code that was returned instead.
        got: StatusCode,
        /// The body returned in the response, in string form.
        body: String,
    },
}

/// An API client configured to communicate with `procession` at the given base URL.
#[derive(Debug)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
}

impl Client {
    /// Create a [`Client`] configured with the provided base URL.
    pub fn new(base_url: &str) -> Result<Self, Error> {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        Ok(Self {
            base_url: Url::parse(base_url)?,
            client: reqwest::Client::builder()
                .user_agent(format!("{name}/{version}"))
                .build()?,
        })
    }

    /// Checks that the server is online.
    pub async fn ping(&self) -> Result<String, Error> {
        let url = self.base_url.join("/api/v1/ping")?;
        let response = self.client.get(url).send().await?;
        let body = self.guard_status(response, StatusCode::OK).await?;
        Ok(String::from_utf8_lossy(body.deref()).to_string())
    }
}

impl Client {
    async fn guard_status(&self, res: Response, expect: StatusCode) -> Result<Bytes, Error> {
        let got = res.status();
        let route = res.url().path().to_owned();
        let body = res.bytes().await?;
        if got != expect {
            Err(Error::StatusCode {
                route,
                expect,
                got,
                body: String::from_utf8_lossy(body.deref()).to_string(),
            })
        } else {
            Ok(body)
        }
    }
}
