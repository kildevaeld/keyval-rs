use std::str::Utf8Error;
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("not found")]
    NotFound,
    #[error("expired")]
    Expired,
    #[error("parse error")]
    Utf8(#[from] Utf8Error),
    #[error("from")]
    FromUtf8(#[from] FromUtf8Error),
    #[cfg(feature = "cbor")]
    #[error("encode error")]
    Cbor(#[from] serde_cbor::Error),
    #[cfg(feature = "json")]
    #[error("encode error")]
    Json(#[from] serde_json::Error),
}
