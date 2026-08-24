//! Read-only access to the AWARE registry index.
//!
//! Consumed by Tasks 5/7/10/11.

pub mod catalog;
pub mod fetch;
pub mod index;

// `IndexEntry` / `VersionEntry` are reached through this facade only by
// `catalog`'s own tests, so the non-test build sees them as unused — hence the
// allow. `Index` is used by both. `BundleEntry` used to sit here too and was
// unused in *either* build; so was the whole
// `catalog::{Catalog, CatalogAgent, build_catalog, search}` re-export, since
// every caller names `catalog::` directly. Both are gone.
#[allow(unused_imports)]
pub use index::{Index, IndexEntry, VersionEntry, checkout_relative_subdir, normalize_subdir};
