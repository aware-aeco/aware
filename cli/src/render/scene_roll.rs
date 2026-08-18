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

#[derive(Clone, Copy, Debug)]
pub struct MemberRollFrame {
    pub axis: Vec3,
    pub zero_x: Vec3,
    pub zero_y: Vec3,
    pub rolled_x: Vec3,
    pub rolled_y: Vec3,
    pub normalized_degrees: f64,
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

pub fn member_frame(
    from: Vec3,
    to: Vec3,
    degrees: f64,
    up: SceneUp,
) -> Result<MemberRollFrame, AwareError> {
    let axis =
        normalized3([to[0] - from[0], to[1] - from[1], to[2] - from[2]]).ok_or_else(|| {
            AwareError::Validation("member axis must have nonzero finite length".into())
        })?;
    let up = up.vector();
    let up_dot = dot3(axis, up);
    let perpendicular_squared = (1.0 - up_dot * up_dot).max(0.0);
    let (zero_x, zero_y) = if perpendicular_squared <= VERTICAL_EPSILON_SQ {
        let seed = [1.0, 0.0, 0.0];
        let zero_x = normalized3(add3(seed, scale3(axis, -dot3(seed, axis)))).ok_or_else(|| {
            AwareError::Validation("vertical member zero frame is degenerate".into())
        })?;
        let zero_y = normalized3(cross3(axis, zero_x)).ok_or_else(|| {
            AwareError::Validation("vertical member zero frame is degenerate".into())
        })?;
        (zero_x, zero_y)
    } else {
        let zero_y = normalized3(add3(up, scale3(axis, -up_dot)))
            .ok_or_else(|| AwareError::Validation("member zero frame is degenerate".into()))?;
        let zero_x = normalized3(cross3(zero_y, axis))
            .ok_or_else(|| AwareError::Validation("member zero frame is degenerate".into()))?;
        (zero_x, zero_y)
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.into_iter().zip(b).all(|(a, b)| (a - b).abs() <= 1e-10)
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
