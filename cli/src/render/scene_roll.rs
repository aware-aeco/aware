use crate::error::AwareError;
use crate::render::geom::{Vec3, add3, cross3, dot3, normalized3, scale3};
use serde_json::Value;

pub const VERTICAL_EPSILON_SQ: f64 = 1e-6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SceneUp {
    Z,
    Y,
}

impl SceneUp {
    pub fn vector(self) -> Vec3 {
        match self {
            Self::Z => [0.0, 0.0, 1.0],
            Self::Y => [0.0, 1.0, 0.0],
        }
    }
}

/// Which of the two zero-frame rules produced a member's frame.
///
/// The rules do not agree where they change over, and no seed can make them —
/// see the discontinuity note on [`member_frame`]. A consumer that needs to know
/// whether a member's facing was derived from its geometry or fixed by convention
/// reads this, rather than re-deriving the threshold and risking a different answer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZeroFrameSource {
    /// Scene up projected perpendicular to the axis: the member's own geometry
    /// fixes the frame.
    ProjectedUp,
    /// Scene `+X` projected perpendicular to the axis, because the member is within
    /// [`VERTICAL_EPSILON_SQ`] of parallel to scene up, where projected up is
    /// degenerate. The resulting facing is convention, not geometry.
    VerticalSeed,
}

#[derive(Clone, Copy, Debug)]
pub struct MemberRollFrame {
    pub axis: Vec3,
    pub zero_x: Vec3,
    pub zero_y: Vec3,
    pub rolled_x: Vec3,
    pub rolled_y: Vec3,
    pub normalized_degrees: f64,
    /// Which rule produced `zero_x`/`zero_y`. See [`ZeroFrameSource`].
    pub zero_frame_source: ZeroFrameSource,
}

pub fn scene_up(scene: &Value, allow_y: bool, consumer: &str) -> Result<SceneUp, AwareError> {
    match scene.get("meta").and_then(|meta| meta.get("up")) {
        None => Ok(SceneUp::Z),
        Some(Value::String(value)) if value == "z" => Ok(SceneUp::Z),
        Some(Value::String(value)) if value == "y" && allow_y => Ok(SceneUp::Y),
        Some(Value::String(value)) if value == "y" => Err(AwareError::Validation(format!(
            "{consumer}: `scene.meta.up` value `y` is unsupported until an explicit Y-up export transform exists"
        ))),
        Some(Value::String(value)) => Err(AwareError::Validation(format!(
            "{consumer}: `scene.meta.up` must be exactly `z`{} (got `{value}`)",
            if allow_y { " or `y`" } else { "" }
        ))),
        Some(_) => Err(AwareError::Validation(format!(
            "{consumer}: `scene.meta.up` must be exactly `z`{}",
            if allow_y { " or `y`" } else { "" }
        ))),
    }
}

pub fn normalize_degrees(degrees: f64) -> Result<f64, AwareError> {
    if !degrees.is_finite() {
        return Err(AwareError::Validation(
            "member roll must be a finite JSON number".into(),
        ));
    }
    let mut normalized = ((degrees % 360.0) + 360.0) % 360.0;
    if normalized >= 180.0 {
        normalized -= 360.0;
    }
    Ok(if normalized == 0.0 { 0.0 } else { normalized })
}

pub fn member_roll(value: Option<&Value>, path: &str, consumer: &str) -> Result<f64, AwareError> {
    match value {
        None => Ok(0.0),
        Some(Value::Number(number)) => normalize_degrees(number.as_f64().ok_or_else(|| {
            AwareError::Validation(format!("{consumer}: `{path}` must be a finite JSON number"))
        })?)
        .map_err(|_| {
            AwareError::Validation(format!("{consumer}: `{path}` must be a finite JSON number"))
        }),
        Some(_) => Err(AwareError::Validation(format!(
            "{consumer}: `{path}` must be a finite JSON number"
        ))),
    }
}

/// The canonical zero-section frame for a member, rolled once about `from→to`.
///
/// # The vertical seam is deliberate, and it is not closed by the seed
///
/// The two rules below meet at [`VERTICAL_EPSILON_SQ`] without agreeing. Writing the
/// axis as `[sinθ·cosφ, sinθ·sinφ, cosθ]` under Z-up: as `θ` shrinks the projected-up
/// rule tends to `zero_x → (−sinφ, cosφ, 0)`, while the seeded rule holds `zero_x ≈ +X`
/// for every `φ`. Across the threshold the frame therefore rotates about the axis by
/// `φ + 90°` — nothing at `φ = 270°`, and a full 180° inversion at `φ = 90°`, which is
/// the case reported in issue #432.
///
/// Projecting the seed perpendicular to the axis (v0.124.0) did not remove that
/// inversion and could not have: the direction the projected-up rule approaches depends
/// on the member's azimuth, so a *fixed* seed that agrees at one azimuth necessarily
/// disagrees at another. Moving or deleting the threshold does not remove it either — a
/// frame varying continuously over every axis direction does not exist at all (the
/// hairy-ball theorem), so a discontinuity is forced wherever the changeover is put.
///
/// The policy is accordingly deterministic rather than continuous: a fixed inclusive
/// threshold, two exactly specified rules, and the side a member landed on reported as
/// [`MemberRollFrame::zero_frame_source`] so a consumer can tell a geometry-derived
/// facing from a conventional one. Conforming producers and consumers match AWARE on a
/// near-vertical asymmetric member only by agreeing on the branch first, which is why
/// the branch test below is specified to the bit and mirrored in
/// `cli-tekla/TeklaMemberRollContract.cs` and `viewer-3d/skills/scene-schema.md`.
pub fn member_frame(
    from: Vec3,
    to: Vec3,
    degrees: f64,
    up: SceneUp,
) -> Result<MemberRollFrame, AwareError> {
    let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let axis = normalized3(delta).ok_or_else(|| {
        AwareError::Validation("member axis must have nonzero finite length".into())
    })?;
    let up = up.vector();
    let up_dot = dot3(axis, up);
    // Which side of the threshold a member falls on, decided two ways at once so that no
    // rounding an implementation is free to choose differently can move it. Both matter
    // because the branches disagree by up to 180° (see above), so a single-bit
    // disagreement here is a whole facing disagreement downstream.
    //
    // As a SUM OF SQUARES, never `1 - up_dot * up_dot`: the subtraction cancels away
    // about six significant digits at the threshold and more than twelve deeper into the
    // band, and real members land on opposite sides of the two readings.
    // `the_branch_test_does_not_cancel_at_the_threshold` pins one.
    //
    // From the RAW delta rather than `axis`, compared as a ratio against `VERTICAL_
    // EPSILON_SQ * |delta|²` (equivalent exactly, since `|p|²/|delta|² <= eps` iff
    // `|p|² <= eps*|delta|²`). Normalizing first would make the branch depend on HOW an
    // implementation normalizes: dividing each component by the length, as `normalized3`
    // does, and multiplying by the reciprocal, as this contract's C# mirror and three.js
    // both do, differ by an ulp — enough to straddle the threshold and hand one member
    // frames 57° apart. `the_branch_test_does_not_depend_on_how_the_axis_is_normalized`
    // pins the member that found it.
    let raw_perpendicular = add3(delta, scale3(up, -dot3(delta, up)));
    let seeded =
        dot3(raw_perpendicular, raw_perpendicular) <= VERTICAL_EPSILON_SQ * dot3(delta, delta);
    let (zero_x, zero_y, zero_frame_source) = if seeded {
        let seed = [1.0, 0.0, 0.0];
        let zero_x = normalized3(add3(seed, scale3(axis, -dot3(seed, axis)))).ok_or_else(|| {
            AwareError::Validation("vertical member zero frame is degenerate".into())
        })?;
        let zero_y = normalized3(cross3(axis, zero_x)).ok_or_else(|| {
            AwareError::Validation("vertical member zero frame is degenerate".into())
        })?;
        (zero_x, zero_y, ZeroFrameSource::VerticalSeed)
    } else {
        let zero_y = normalized3(add3(up, scale3(axis, -up_dot)))
            .ok_or_else(|| AwareError::Validation("member zero frame is degenerate".into()))?;
        let zero_x = normalized3(cross3(zero_y, axis))
            .ok_or_else(|| AwareError::Validation("member zero frame is degenerate".into()))?;
        (zero_x, zero_y, ZeroFrameSource::ProjectedUp)
    };
    let normalized_degrees = normalize_degrees(degrees)?;
    let radians = normalized_degrees.to_radians();
    let rolled_x = add3(scale3(zero_x, radians.cos()), scale3(zero_y, radians.sin()));
    let rolled_y = add3(
        scale3(zero_y, radians.cos()),
        scale3(zero_x, -radians.sin()),
    );
    Ok(MemberRollFrame {
        axis,
        zero_x,
        zero_y,
        rolled_x,
        rolled_y,
        normalized_degrees,
        zero_frame_source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.into_iter().zip(b).all(|(a, b)| (a - b).abs() <= 1e-10)
    }

    fn degrees_between(a: Vec3, b: Vec3) -> f64 {
        dot3(a, b).clamp(-1.0, 1.0).acos().to_degrees()
    }

    /// A unit axis `sin θ` off scene `+Z`, leaning toward azimuth `phi_degrees`.
    fn axis_at(sin_theta: f64, phi_degrees: f64) -> Vec3 {
        let phi = phi_degrees.to_radians();
        [
            sin_theta * phi.cos(),
            sin_theta * phi.sin(),
            (1.0 - sin_theta * sin_theta).sqrt(),
        ]
    }

    #[test]
    fn boundary_angles_share_the_canonical_half_open_range() {
        for (input, expected) in [
            (-540.0, -180.0),
            (-360.0, 0.0),
            (-180.0, -180.0),
            (-0.0, 0.0),
            (0.0, 0.0),
            (180.0, -180.0),
            (360.0, 0.0),
            (540.0, -180.0),
        ] {
            let actual = normalize_degrees(input).unwrap();
            assert_eq!(actual, expected);
            if actual == 0.0 {
                assert!(!actual.is_sign_negative());
            }
        }
    }

    #[test]
    fn z_up_golden_vectors_cover_axis_reversal_and_general_case() {
        let up = member_frame([0.0; 3], [0.0, 0.0, 1.0], 90.0, SceneUp::Z).unwrap();
        assert!(close(up.rolled_x, [0.0, 1.0, 0.0]));
        assert!(close(up.rolled_y, [-1.0, 0.0, 0.0]));

        let down = member_frame([0.0; 3], [0.0, 0.0, -1.0], 90.0, SceneUp::Z).unwrap();
        assert!(close(down.rolled_x, [0.0, -1.0, 0.0]));
        assert!(close(down.rolled_y, [-1.0, 0.0, 0.0]));

        let horizontal = member_frame([0.0; 3], [1.0, 0.0, 0.0], 0.0, SceneUp::Z).unwrap();
        assert!(close(horizontal.zero_x, [0.0, 1.0, 0.0]));
        assert!(close(horizontal.zero_y, [0.0, 0.0, 1.0]));
    }

    #[test]
    fn inclusive_vertical_threshold_stays_orthonormal() {
        for horizontal in [1e-3 * 0.999, 1e-3, 1e-3 * 1.001] {
            let z = (1.0_f64 - horizontal * horizontal).sqrt();
            let frame = member_frame([0.0; 3], [horizontal, 0.0, z], 82.7, SceneUp::Z).unwrap();
            assert!(dot3(frame.axis, frame.rolled_x).abs() <= 1e-10);
            assert!(dot3(frame.axis, frame.rolled_y).abs() <= 1e-10);
            assert!(dot3(frame.rolled_x, frame.rolled_y).abs() <= 1e-10);
            assert!(dot3(cross3(frame.axis, frame.rolled_x), frame.rolled_y) >= 1.0 - 1e-10);
        }
    }

    #[test]
    fn the_vertical_seam_inverts_an_asymmetric_member_by_180_degrees() {
        // Issue #432: at azimuth 90° — `axis = normalize([0, +eps, 1])` — the zero frame
        // reads `+X` on the seeded side of the threshold and `-X` one step past it.
        // PINNED, NOT FIXED: `member_frame` documents why no seed and no threshold
        // placement can close this seam. The test is here so that changing the accepted
        // policy has to be deliberate instead of silent, which is what issue #432 found
        // — `inclusive_vertical_threshold_stays_orthonormal` proves both sides are
        // orthonormal, and two frames 180° apart both are.
        let inside = member_frame([0.0; 3], axis_at(1e-3 * 0.999, 90.0), 0.0, SceneUp::Z).unwrap();
        let outside = member_frame([0.0; 3], axis_at(1e-3 * 1.001, 90.0), 0.0, SceneUp::Z).unwrap();

        assert_eq!(inside.zero_frame_source, ZeroFrameSource::VerticalSeed);
        assert_eq!(outside.zero_frame_source, ZeroFrameSource::ProjectedUp);
        assert!(close(inside.zero_x, [1.0, 0.0, 0.0]));
        assert!(close(outside.zero_x, [-1.0, 0.0, 0.0]));
        assert!((degrees_between(inside.zero_x, outside.zero_x) - 180.0).abs() <= 1e-3);
    }

    #[test]
    fn the_seam_rotation_tracks_azimuth_so_no_fixed_seed_can_close_it() {
        // The seeded rule holds `zero_x ≈ +X` at every azimuth, while the projected-up
        // rule leaves the threshold at exactly `(-sin φ, cos φ, 0)`. The jump across the
        // seam is therefore `acos(-sin φ)`: nothing at φ=270°, a full inversion at φ=90°.
        // That it VARIES with azimuth is the whole argument — a fixed seed agreeing at
        // one azimuth must disagree at another, so the seam is policy, not a bug in the
        // seed, and the v0.124.0 projected seed never could have removed it.
        let mut worst_error: f64 = 0.0;
        let mut worst_seam: f64 = 0.0;
        for step in 0..24 {
            let phi = f64::from(step) * 15.0;
            let inside = member_frame([0.0; 3], axis_at(1e-3 * 0.999, phi), 0.0, SceneUp::Z);
            let outside = member_frame([0.0; 3], axis_at(1e-3 * 1.001, phi), 0.0, SceneUp::Z);
            let (inside, outside) = (inside.unwrap(), outside.unwrap());
            assert_eq!(inside.zero_frame_source, ZeroFrameSource::VerticalSeed);
            assert_eq!(outside.zero_frame_source, ZeroFrameSource::ProjectedUp);

            let seam = degrees_between(inside.zero_x, outside.zero_x);
            let expected = (-phi.to_radians().sin())
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            worst_error = worst_error.max((seam - expected).abs());
            worst_seam = worst_seam.max(seam);
        }
        assert!(
            worst_error <= 1e-3,
            "seam departed from acos(-sin phi) by {worst_error} deg"
        );
        assert!(
            (worst_seam - 180.0).abs() <= 1e-3,
            "the seam must still reach a full inversion somewhere, peaked at {worst_seam} deg"
        );
    }

    #[test]
    fn zero_frame_source_names_the_rule_and_the_threshold_stays_inclusive() {
        let vertical = member_frame([0.0; 3], [0.0, 0.0, 1.0], 0.0, SceneUp::Z).unwrap();
        assert_eq!(vertical.zero_frame_source, ZeroFrameSource::VerticalSeed);
        let horizontal = member_frame([0.0; 3], [1.0, 0.0, 0.0], 0.0, SceneUp::Z).unwrap();
        assert_eq!(horizontal.zero_frame_source, ZeroFrameSource::ProjectedUp);

        // Scene up is axis-aligned under both declared ups, so clearing its component is
        // exact in each — a member parallel to `+Y` under Y-up seeds just as `+Z` does
        // under Z-up, rather than landing on whichever side rounding picks.
        let vertical_y = member_frame([0.0; 3], [0.0, 1.0, 0.0], 0.0, SceneUp::Y).unwrap();
        assert_eq!(vertical_y.zero_frame_source, ZeroFrameSource::VerticalSeed);

        // `<=`, so a member sitting exactly on the threshold is seeded.
        let on = member_frame([0.0; 3], axis_at(1e-3, 0.0), 0.0, SceneUp::Z).unwrap();
        assert_eq!(on.zero_frame_source, ZeroFrameSource::VerticalSeed);
        let past = member_frame([0.0; 3], axis_at(1e-3 * 1.001, 0.0), 0.0, SceneUp::Z).unwrap();
        assert_eq!(past.zero_frame_source, ZeroFrameSource::ProjectedUp);
    }

    #[test]
    fn the_branch_test_does_not_cancel_at_the_threshold() {
        // This axis is the reason the branch test is a sum of squares rather than
        // `1 - up_dot * up_dot`. Its true perpendicular component is 1.0000000000198e-6,
        // just OUTSIDE the threshold, so it is a projected-up member; the cancelling form
        // computes 9.9999999992e-7 and seeds it instead. Its azimuth is ~191.5°, where the
        // seam is ~78°, so the two forms hand the same member two frames 78° apart — one
        // implementation conforming to the contract by the cancelling reading and another
        // by this one would disagree about a real member's facing. Sampling 400k axes
        // within ±3e-9 of the threshold puts ~0.7% of them in this disagreeing band.
        let axis = [
            -0.000_980_074_099_488_643_5,
            -0.000_198_632_221_785_167_07,
            0.999_999_499_999_875,
        ];
        let frame = member_frame([0.0; 3], axis, 0.0, SceneUp::Z).unwrap();
        assert_eq!(frame.zero_frame_source, ZeroFrameSource::ProjectedUp);
    }

    #[test]
    fn the_branch_test_does_not_depend_on_how_the_axis_is_normalized() {
        // Codex found this input on #435. Normalized by DIVIDING each component by the
        // length — what `normalized3` does — its perpendicular component is exactly
        // 1e-6 and it seeds; normalized by MULTIPLYING by the reciprocal — what this
        // contract's C# mirror and three.js both do — it is 1.0000000000000002e-6 and
        // projects. One ulp of normalization, and the same member gets frames 57° apart
        // from two implementations both following the contract to the letter.
        //
        // So the branch is decided on the RAW delta as a ratio, which uses no
        // normalization at all and cannot be moved by that choice.
        //
        // WHAT THIS TEST CAN AND CANNOT SEE. It pins the frame this input must produce,
        // which is the behaviour a conforming consumer has to match. It does NOT catch a
        // revert to the normalized-axis test here, and that was measured, not assumed:
        // swapping the implementation back leaves all of these green, because Rust's
        // `normalized3` divides and so happens to agree with the raw ratio — a search
        // over 600k near-threshold axes found no input where those two disagree. The
        // divergence is BETWEEN implementations, so no test in this language can observe
        // it. The mirrors carry the guard instead, each where its own normalization does
        // diverge: `BranchesFromTheRawAxisSoNormalizationCannotMoveTheSeam` in
        // `cli-tekla/Tests`, and the source assertions in
        // `the_viewer_selects_the_zero_frame_branch_the_export_sinks_do`.
        let to = [
            26.550_754_660_495_15,
            -17.294_594_180_470_543,
            31_686.662_128_779_197,
        ];
        let frame = member_frame([0.0; 3], to, 0.0, SceneUp::Z).unwrap();
        assert_eq!(frame.zero_frame_source, ZeroFrameSource::VerticalSeed);
        assert!(close(
            frame.zero_x,
            [
                0.999_999_648_948_850_4,
                4.573_345_136_052_488e-7,
                -0.000_837_915_250_350_489_3,
            ]
        ));

        // Scale invariance, checked on members that are NOT sitting on the boundary. It
        // deliberately excludes the axis above: that one satisfies the test with exact
        // equality (`|p|²` and `eps*|delta|²` are the same double), so scaling it re-rounds
        // both sides of an equality and a large enough factor tips it — measured, ×1000
        // does. No ratio test can be bit-invariant there, and claiming otherwise would be
        // asserting something false about the contract rather than testing it.
        for (sin_theta, expected) in [
            (1e-3 * 0.99, ZeroFrameSource::VerticalSeed),
            (1e-3 * 1.01, ZeroFrameSource::ProjectedUp),
        ] {
            for scale in [1e-3, 0.5, 7.0, 1e3, 1e6] {
                let unit = axis_at(sin_theta, 37.0);
                let scaled = [unit[0] * scale, unit[1] * scale, unit[2] * scale];
                let frame = member_frame([0.0; 3], scaled, 0.0, SceneUp::Z).unwrap();
                assert_eq!(
                    frame.zero_frame_source, expected,
                    "sin(theta)={sin_theta} scaled by {scale} changed branch"
                );
            }
        }
    }

    #[test]
    fn the_branch_test_is_cross_multiplied_and_never_a_quotient() {
        // The third rearrangement, and the one most likely to be written by someone
        // reading "ratio against |d|²" as a division. Codex found this input on #435:
        // `|q|² = 1.7497609055789547` exceeds `1e-6*|d|² = 1.7497609055789545`, so the
        // cross-multiplied comparison this contract specifies PROJECTS it — while
        // `|q|²/|d|²` rounds to exactly 1e-6 and seeds it, for frames 64.9° apart.
        //
        // Unlike `the_branch_test_does_not_depend_on_how_the_axis_is_normalized`, this
        // one really does guard the implementation: swapping in the quotient here flips
        // this member and fails the assertion.
        let to = [
            1.197_682_677_589_816_4,
            -0.561_531_040_442_327_3,
            1_322.784_621_855_746,
        ];
        let frame = member_frame([0.0; 3], to, 0.0, SceneUp::Z).unwrap();
        assert_eq!(frame.zero_frame_source, ZeroFrameSource::ProjectedUp);
        // Projected-up leaves the threshold at exactly `(-sin φ, cos φ, 0)`; the seeded
        // rule would have put zero X within a whisker of `+X` instead.
        assert!(close(
            frame.zero_x,
            [0.424_506_567_735_086, 0.905_424_858_257_038, 0.0]
        ));
    }

    #[test]
    fn equivalent_y_up_and_z_up_axes_have_equivalent_frames() {
        let y = member_frame([0.0; 3], [3.0, 4.0, 2.0], 65.2, SceneUp::Y).unwrap();
        // Swapping scene Y/Z is reflective, so an equivalent right-handed roll
        // negates the angle and one in-plane basis axis.
        let z = member_frame([0.0; 3], [3.0, 2.0, 4.0], -65.2, SceneUp::Z).unwrap();
        let swap = |value: Vec3| [value[0], value[2], value[1]];
        assert!(close(swap(y.axis), z.axis));
        assert!(close(scale3(swap(y.rolled_x), -1.0), z.rolled_x));
        assert!(close(swap(y.rolled_y), z.rolled_y));
    }
}
