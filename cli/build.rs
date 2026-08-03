// `build.rs` is its own crate root, so the gate in `src/main.rs` does not reach
// it. Repeat it here rather than leave a hole the sibling `[lints.clippy]` gate
// (`undocumented_unsafe_blocks`, #344) would have covered — `[lints]` applies to
// every target in the package, a crate-root attribute only to one.
#![deny(clippy::unwrap_used, clippy::expect_used)]

fn main() {
    // `auth::config` reads option_env!("AWARE_GOOGLE_CLIENT_SECRET") at compile
    // time to bake the Google desktop-client secret into release binaries. Tell
    // Cargo to rebuild when that value changes so a release never serves a stale
    // incremental build with an old or empty secret. The secret is never committed
    // — it is injected by the release CI build env (see .github/workflows/release.yml).
    println!("cargo:rerun-if-env-changed=AWARE_GOOGLE_CLIENT_SECRET");
}
