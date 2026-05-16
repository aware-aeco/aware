//! Read-only access to the AWARE registry index.
//!
//! Consumed by Tasks 5/7/10/11.

pub mod fetch;
pub mod index;

#[allow(unused_imports)] // BundleEntry / IndexEntry / VersionEntry consumed by tests + Task 11
pub use index::{BundleEntry, Index, IndexEntry, VersionEntry};
