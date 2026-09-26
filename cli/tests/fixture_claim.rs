//! The builder election that keeps a cold fixture cache from being copied by
//! every test binary at once (#578).
//!
//! These pin `common`'s claim primitives rather than the fixture build itself:
//! the election is the part that has to be right when a builder dies, and the
//! part whose failure mode — every waiter copying ~400 MB at the same time —
//! is the disk exhaustion #578 was filed about. Driving it through real
//! fixture builds would cost minutes and gigabytes per assertion; these run in
//! milliseconds against temp directories and never touch the catalogue.

mod common;

use common::{BuildClaim, claim_dir, evict_superseded, latest_generation};

#[test]
fn a_claim_admits_exactly_one_holder() {
    let tmp = tempfile::tempdir().unwrap();
    let claim = tmp.path().join("fixture.building");

    let first = BuildClaim::take(&claim).unwrap();
    assert!(first.is_some(), "the first taker must get the claim");
    assert!(
        BuildClaim::take(&claim).unwrap().is_none(),
        "a second taker must be told the claim is held, not handed a copy of it"
    );
}

#[test]
fn a_released_generation_is_kept_as_a_tombstone_and_never_reissued() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware-fixture-v1-abc");
    let claim = claim_dir(tmp.path(), &home, 0);

    drop(BuildClaim::take(&claim).unwrap().expect("first claim"));

    // Deleting it would make generation 0 reusable: a waiter that saw it
    // advances to 1, while a process arriving afterwards finds no claim at
    // all, starts again at 0, and both become builders.
    assert!(
        claim.is_dir(),
        "a released generation must stay on disk as its own tombstone"
    );
    assert!(
        claim.join(BuildClaim::RELEASED).exists(),
        "a released generation must say so, or waiters sit out the full wait"
    );
    assert!(
        BuildClaim::take(&claim).unwrap().is_none(),
        "a released generation must never be handed out a second time"
    );
    assert_eq!(
        latest_generation(tmp.path(), &home),
        0,
        "a released generation must still count, or its number is reused"
    );
}

#[test]
fn a_claim_is_not_retired_by_a_holder_that_lost_it() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware-fixture-v1-abc");
    let claim = claim_dir(tmp.path(), &home, 0);

    // A builder whose claim is removed out from under it — by the eviction
    // sweep, or by any hand — and re-taken by a replacement while it is still
    // copying.
    let outrun = BuildClaim::take(&claim).unwrap().expect("first claim");
    std::fs::remove_dir_all(&claim).expect("remove the claim out from under it");
    let replacement = BuildClaim::take(&claim)
        .unwrap()
        .expect("replacement claim");

    // The outrun builder now finishes. Retiring the replacement's claim would
    // send every waiter on to the next generation and admit a second copy.
    drop(outrun);
    assert!(
        !claim.join(BuildClaim::RELEASED).exists(),
        "a builder that lost its claim retired its successor's"
    );

    drop(replacement);
    assert!(
        claim.join(BuildClaim::RELEASED).exists(),
        "the holder that still owns the claim must be able to retire it"
    );
}

#[test]
fn a_claim_records_who_holds_it() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware-fixture-v1-abc");
    let claim = claim_dir(tmp.path(), &home, 0);

    let held = BuildClaim::take(&claim).unwrap().expect("claim");
    let owner = std::fs::read_to_string(claim.join(BuildClaim::OWNER)).expect("owner token");
    assert!(
        owner.starts_with(&format!("{}-", std::process::id())),
        "the token must name the holding process, got {owner:?}"
    );

    // Distinct per acquisition, not merely per process: one process takes a
    // succession of generations, and the holder of the second must not be
    // mistaken for the holder of the first.
    let next = claim_dir(tmp.path(), &home, 1);
    let later = BuildClaim::take(&next).unwrap().expect("next generation");
    let second = std::fs::read_to_string(next.join(BuildClaim::OWNER)).expect("owner token");
    assert_ne!(owner, second, "two acquisitions must not share a token");
    drop(held);
    drop(later);
}

#[test]
fn a_generation_is_a_distinct_claim() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware-fixture-v1-abc");

    let first = claim_dir(tmp.path(), &home, 0);
    let second = claim_dir(tmp.path(), &home, 1);
    assert_ne!(
        first, second,
        "two generations must not share a path, or electing a replacement \
         would contend with the claim it is replacing"
    );
    // The election's whole safety argument: a waiter still holding an
    // abandoned generation can do nothing to its successor, because it never
    // touches that path.
    let held = BuildClaim::take(&first).unwrap().expect("generation 0");
    // Bound, not asserted on as a temporary: a temporary would be dropped at
    // the end of the statement, releasing the very claim the next assertion is
    // about.
    let successor = BuildClaim::take(&second).unwrap();
    assert!(
        successor.is_some(),
        "the next generation must be electable while the abandoned one stands"
    );
    drop(held);
    assert!(
        second.is_dir(),
        "releasing an abandoned generation must leave its successor's claim alone"
    );
}

#[test]
fn a_run_joins_the_newest_generation() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware-fixture-v1-abc");

    assert_eq!(
        latest_generation(tmp.path(), &home),
        0,
        "with no claim present a run must start at generation 0"
    );

    std::fs::create_dir_all(claim_dir(tmp.path(), &home, 0)).unwrap();
    std::fs::create_dir_all(claim_dir(tmp.path(), &home, 3)).unwrap();
    assert_eq!(
        latest_generation(tmp.path(), &home),
        3,
        "a run must contend for the newest generation, not one already abandoned"
    );

    // Another fixture's claims, and the fixtures themselves, are not ours.
    std::fs::create_dir_all(claim_dir(
        tmp.path(),
        &tmp.path().join("aware-fixture-v1-other"),
        9,
    ))
    .unwrap();
    std::fs::create_dir_all(tmp.path().join("aware-fixture-v1-abc.building-notanumber")).unwrap();
    assert_eq!(
        latest_generation(tmp.path(), &home),
        3,
        "only this fixture's numbered claims may set the generation"
    );
}

#[test]
fn eviction_spares_the_live_fixture_and_its_generation_counter() {
    let tmp = tempfile::tempdir().unwrap();
    let live = tmp.path().join("aware-fixture-v1-live");
    std::fs::create_dir_all(&live).unwrap();

    // A retired generation of the LIVE fixture. It is the generation counter,
    // so evicting it would hand generation 0 out a second time — the hazard
    // `BuildClaim::drop` keeps the tombstone for in the first place.
    let tombstone = claim_dir(tmp.path(), &live, 0);
    drop(BuildClaim::take(&tombstone).unwrap().expect("claim"));

    let superseded = tmp.path().join("aware-fixture-v1-old");
    std::fs::create_dir_all(&superseded).unwrap();
    let superseded_claim = claim_dir(tmp.path(), &superseded, 0);
    std::fs::create_dir_all(&superseded_claim).unwrap();
    let foreign = tmp.path().join("unrelated-file");
    std::fs::write(&foreign, b"not ours").unwrap();

    // Everything here was made moments ago, so a real grace spares all of it.
    evict_superseded(
        tmp.path(),
        &live,
        std::time::Duration::from_secs(24 * 60 * 60),
    );
    assert!(
        superseded.is_dir(),
        "a fixture inside the grace may still be in use by a concurrent run"
    );

    // With no grace, every candidate is due — so what survives is what the
    // sweep refuses to touch on purpose, not merely what is too new.
    evict_superseded(tmp.path(), &live, std::time::Duration::ZERO);

    assert!(live.is_dir(), "the fixture in use must never be evicted");
    assert!(
        tombstone.is_dir(),
        "evicting the live fixture's retired generation makes its number reusable"
    );
    assert!(
        !superseded.exists(),
        "a superseded fixture past the grace is the whole point of the sweep"
    );
    assert!(
        !superseded_claim.exists(),
        "a superseded fixture's claims carry no counter worth keeping"
    );
    assert!(
        foreign.exists(),
        "the sweep must only touch its own directories"
    );
}
