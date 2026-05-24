//! OAuth + keychain + refresh for AWARE integrations.
//!
//! Submodules added across Tasks 2-5.

#![allow(dead_code)]

pub mod config;
pub mod device; // v0.13 — RFC 8628 device-code flow
pub mod keychain;
pub mod paste;
pub mod pkce; // Task 4
pub mod profile; // Tier 2 BYO OAuth app profiles (#146)
pub mod refresh; // Task 5
