//! Central HTTP execution, authentication, decoding, and policy enforcement.
mod decoding;
mod endpoint;
mod execution;
pub(crate) mod rate_limit;
mod request;
pub mod retry;

pub(crate) use decoding::{empty, request};
pub(crate) use request::{OperationPolicy, body};
