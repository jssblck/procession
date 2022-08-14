//! This library is only intended to be used within the `procession` workspace.
//! The effect of this is that any level of change may occur on any version release.
//!
//! Unlike libraries, `procession` server releases are more like marketing version
//! releases than semantic versioning releases. Any version, even a patch,
//! may introduce large refactorings or breaking changes to this internal library.

#![deny(clippy::unwrap_used)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod api;
pub mod redis;
pub mod style;
