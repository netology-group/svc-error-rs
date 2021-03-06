#![deny(missing_debug_implementations, missing_docs)]

//! SvcError is an implementation of RFC7807: Problem Details for HTTP APIs.
//!
//! It's possible to attach some metadata to the error (only `payload` now).

use http::StatusCode;
use std::collections::HashMap;

/// Problem Details
pub trait ProblemDetails: ProblemDetailsReadOnly + ProblemDetailsMut + serde::Serialize {}

/// Getters for Problem Details and accompanying metadata

impl ProblemDetails for Error {}

/// Getters for Problem Details
pub trait ProblemDetailsReadOnly {
    /// Return a URI reference RFC3986 that identifies the
    /// problem type.  This specification encourages that, when
    /// dereferenced, it provide human-readable documentation for the
    /// problem type. When this member is not present, its value is assumed
    /// to be "about:blank".
    fn kind(&self) -> &str;

    /// Return a short, human-readable summary of the problem
    /// type. It SHOULD NOT change from occurrence to occurrence of the
    /// problem, except for purposes of localization
    fn title(&self) -> &str;

    /// Return the HTTP status code generated by the origin server for this
    /// occurrence of the problem.
    fn status_code(&self) -> StatusCode;

    /// Return a human-readable explanation specific to this occurrence of the problem.
    fn detail(&self) -> Option<&str>;

    /// Return all error related extra values.
    fn extras(&self) -> &HashMap<String, String>;
}

impl ProblemDetailsReadOnly for Error {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }

    fn kind(&self) -> &str {
        self.kind()
    }

    fn title(&self) -> &str {
        self.title()
    }

    fn detail(&self) -> Option<&str> {
        self.detail()
    }

    fn extras(&self) -> &HashMap<String, String> {
        self.extras()
    }
}

/// Mutation methods for Problem Details.
/// This methods are in separate trait to make ProblemDetailsReadOnly object safe.
pub trait ProblemDetailsMut {
    /// Set a kind and a title.
    fn set_kind(&mut self, kind: &str, title: &str) -> &mut Self;

    /// Set a status code.
    fn set_status_code(&mut self, value: StatusCode) -> &mut Self;

    /// Set a detail.
    fn set_detail(&mut self, detail: &str) -> &mut Self;
}

impl ProblemDetailsMut for Error {
    fn set_kind(&mut self, kind: &str, title: &str) -> &mut Self {
        self.set_kind(kind, title)
    }

    fn set_status_code(&mut self, value: StatusCode) -> &mut Self {
        self.set_status_code(value)
    }

    fn set_detail(&mut self, detail: &str) -> &mut Self {
        self.set_detail(detail)
    }
}

pub use self::error::{Builder, Error};
mod error;

/// Extensions
pub mod extension;
