//! Rust SDK for the AEO Protocol v0.1.
//!
//! Parse, build, validate, and fetch AEO declaration documents.
//!
//! Specification: <https://github.com/mizcausevic-dev/aeo-protocol-spec>
//!
//! # Example
//!
//! ```no_run
//! use aeo_protocol::{Document, fetch_well_known};
//!
//! let doc = fetch_well_known("https://mizcausevic-dev.github.io")?;
//! println!("{}", doc.entity.name);
//! # Ok::<(), aeo_protocol::AeoError>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

mod document;
mod error;

#[cfg(feature = "client")]
mod client;

pub use document::{
    AnswerConstraints, Audit, AuditMode, Authority, CitationPreferences, Claim, Confidence,
    Document, Entity, EntityType, Verification, VerificationType,
};
pub use error::AeoError;

#[cfg(feature = "client")]
pub use client::{fetch_well_known, well_known_url};

/// The AEO Protocol version supported by this SDK.
pub const PROTOCOL_VERSION: &str = "0.1";

/// The version of this SDK crate.
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
