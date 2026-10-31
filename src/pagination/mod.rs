//! Endpoint-specific pagination data.
//!
//! Continuations come from response metadata. Links are preserved for inspection
//! but are never followed by the SDK, preventing authenticated requests to a
//! server-selected destination.

mod cursor;
mod links;
mod numbered;

pub use cursor::{CursorMeta, CursorPage};
pub use links::PageLinks;
pub use numbered::{NumberedMeta, NumberedPage};

#[cfg(feature = "stream")]
mod stream;
#[cfg(feature = "stream")]
pub(crate) use stream::{
    cursor_stream, numbered_stream, numbered_stream_with_policy, validated_stream,
};
