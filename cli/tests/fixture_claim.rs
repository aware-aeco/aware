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

use common::{BuildClaim, claim_dir, latest_generation};

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
fn releasing_a_claim_lets_the_next_builder_take_it() {
    let tmp = tempfile::tempdir().unwrap();
    let claim = tmp.path().join("fixture.building");

    drop(BuildClaim::take(&claim).unwrap().expect("first claim"));
    assert!(
        !claim.exists(),
        "a released claim must leave nothing behind"
    );
    assert!(
        BuildClaim::take(&claim).unwrap().is_some(),
        "the next builder must be able to take the released claim"
    );
}

#[test]
fn a_claim_is_not_retired_by_a_holder_that_lost_it() {
    let tmp = tempfile::tempdir().unwrap();
    let claim = tmp.path().join("fixture.building");

    // A builder slower than BUILD_WAIT: its claim is stolen and re-taken by a
    // replacement while it is still copying.
    let outrun = BuildClaim::take(&claim).unwrap().expect("first claim");
    std::fs::remove_dir_all(&claim).expect("steal the abandoned-looking claim");
    let replacement = BuildClaim::take(&claim)
        .unwrap()
        .expect("replacement claim");

    // The slow builder now finishes. It must not retire the claim the
    // replacement is holding — that would admit a third concurrent copy.
    drop(outrun);
    assert!(
        claim.is_dir(),
        "a builder that lost its claim deleted its successor's"
    );

    drop(replacement);
    assert!(
        !claim.exists(),
        "the holder that still owns the claim must be able to release it"
    );
}

#[test]
fn a_claim_records_who_holds_it() {
    let tmp = tempfile::tempdir().unwrap();
    let claim = tmp.path().join("fixture.building");

    let held = BuildClaim::take(&claim).unwrap().expect("claim");
    let owner = std::fs::read_to_string(claim.join(BuildClaim::OWNER)).expect("owner token");
    assert!(
        owner.starts_with(&format!("{}-", std::process::id())),
        "the token must name the holding process, got {owner:?}"
    );

    // Distinct per acquisition, not merely per process: the same pid can lose
    // a claim and take it again, and the second holder must not be mistaken
    // for the first.
    drop(held);
    let retaken = BuildClaim::take(&claim).unwrap().expect("reclaim");
    let second = std::fs::read_to_string(claim.join(BuildClaim::OWNER)).expect("owner token");
    assert_ne!(owner, second, "two acquisitions must not share a token");
    drop(retaken);
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
