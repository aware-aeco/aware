//! Read-only access to the AWARE registry index.
//!
//! Consumed by Tasks 5/7/10/11.

pub mod catalog;
pub mod fetch;
pub mod index;

#[allow(unused_imports)] // consumed by the agent catalog/search/has commands
pub use catalog::{Catalog, CatalogAgent, build_catalog, search};
#[allow(unused_imports)] // BundleEntry / IndexEntry / VersionEntry consumed by tests + Task 11
pub use index::{BundleEntry, Index, IndexEntry, VersionEntry};
