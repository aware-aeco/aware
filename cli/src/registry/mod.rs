//! Read-only access to the AWARE registry index.
//!
//! Consumed by Tasks 5/7/10/11.

#![allow(dead_code)]

pub mod fetch;
pub mod index;

#[allow(unused_imports)]
pub use index::{BundleEntry, Index, IndexEntry, VersionEntry};
