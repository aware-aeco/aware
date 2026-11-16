//! The canonical member section `xsection {shape:"outline", outer, holes?}` (#797).
//!
//! One validator shared by every sink that reads it (`ifc.write`, `viewer-3d`), so a section one
//! sink accepts is a section the other accepts, and an invalid one is refused with the same reason
//! everywhere. The check is errors-as-data: `parse_outline` returns the exact rule that failed as a
//! string and never coerces, repairs or guesses a section.
//!
//! Contract (mm, the section plane of the member, origin on the member axis):
//!
//! * `outer`: the closed boundary, `[[x,y],...]`, at least 3 distinct vertices.
//! * `holes`: optional (absent or `null` is none), at most [`MAX_HOLES`] rings of the same form.
//! * Either winding is accepted; the result is normalised to `outer` counter-clockwise and every
//!   hole clockwise. A ring whose last point repeats its first is accepted, the repeat is dropped.
//! * Every ring is simple (no repeated point, no spike, no self-touch or crossing) with non-zero
//!   area; each hole lies strictly inside `outer`; holes neither touch, overlap nor nest.
//! * All rings together carry at most [`MAX_POINTS`] vertices (bounds the pairwise edge tests), and
//!   every coordinate lies within [`MAX_COORDINATE`] of the member axis (keeps every intermediate
//!   product finite and the predicates meaningful).
//! * "Same point", "touching" and "no area" are judged to [`TOLERANCE`] millimetres, the IFC
//!   context precision (the same constant `ifc.write` writes), so what this accepts is what the file
//!   can represent.
//! * When the member declares `section.w` and `section.d`, the outline's bounding box must fit
//!   within `w` by `d`. It need not be centred: the origin is the member axis, not the box centre.

use serde_json::{Map, Value};

use super::geom::{Vec2, cross2, point_in_polygon, point_segment_distance, polygon_edges};

/// Holes per outline.
pub const MAX_HOLES: usize = 16;
/// Vertices across the outer ring and every hole together.
pub const MAX_POINTS: usize = 8192;
/// Largest |x| or |y| of a point, in millimetres (1 km from the member axis).
pub const MAX_COORDINATE: f64 = 1.0e6;
/// Length tolerance in millimetres for "same point", "touching" and "no area". It IS the precision of
/// the IFC geometric representation context (`ifc.write` writes this very constant), so validation
/// and the file can never disagree about whether two edges touch.
pub const TOLERANCE: f64 = 1.0e-5;

/// A validated, normalised outline: `outer` counter-clockwise, each hole clockwise, no closing repeat.
#[derive(Debug, Clone, PartialEq)]
pub struct OutlineSection {
    pub outer: Vec<Vec2>,
    pub holes: Vec<Vec<Vec2>>,
}

/// The `section.{w,d}` a member declares, when it declares both as positive finite numbers.
pub fn declared_envelope(section: Option<&Value>) -> Option<(f64, f64)> {
    let section = section?.as_object()?;
    let pick = |key: &str| {
        section
            .get(key)
            .and_then(Value::as_f64)
            .filter(|value| value.is_finite() && *value > 0.0)
    };
    Some((pick("w")?, pick("d")?))
}

/// How a ring winds when it is read. A member section accepts either and normalises; a scene
/// `extrusion` states its winding and an opposite one is reported, never reversed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winding {
    /// Either winding is accepted; the result is outer counter-clockwise, holes clockwise.
    Normalise,
    /// The outer ring must already be counter-clockwise and every hole clockwise.
    Require,
}

/// The limits and wording one consumer of the shared ring rules applies. The geometric rules
/// themselves (simple, non-zero area, holes strictly inside and apart, [`TOLERANCE`]) are the same
/// for every consumer; only these differ.
#[derive(Debug, Clone, Copy)]
pub struct RingLimits {
    /// Holes per ring set.
    pub max_holes: usize,
    /// Vertices across the outer ring and every hole together, when bounded that way.
    pub max_points_total: Option<usize>,
    /// Vertices in any one ring, when bounded that way.
    pub max_points_per_ring: Option<usize>,
    /// Largest |x| or |y| of a point, in millimetres from `origin_name`.
    pub max_coordinate: f64,
    /// What the coordinates are measured from, for the out-of-range message.
    pub origin_name: &'static str,
    /// What the ring set is called in a count message ("outline", "items[0]").
    pub subject: &'static str,
    pub winding: Winding,
}

/// The member section's limits (#797): 16 holes, 8192 points in all, either winding.
pub const OUTLINE_LIMITS: RingLimits = RingLimits {
    max_holes: MAX_HOLES,
    max_points_total: Some(MAX_POINTS),
    max_points_per_ring: None,
    max_coordinate: MAX_COORDINATE,
    origin_name: "the member axis",
    subject: "outline",
    winding: Winding::Normalise,
};

/// Validate an `xsection` object whose `shape` is `outline`. `Err` is the exact reason, ready to be
/// shown after the member's name.
pub fn parse_outline(
    xsection: &Map<String, Value>,
    declared: Option<(f64, f64)>,
) -> Result<OutlineSection, String> {
    let outer_value = xsection
        .get("outer")
        .ok_or_else(|| "outline needs `outer`, an array of [x, y] points".to_string())?;
    let section = parse_rings(
        outer_value,
        xsection.get("holes"),
        "outer",
        "holes",
        &OUTLINE_LIMITS,
    )?;
    if let Some((w, d)) = declared {
        let (min_x, max_x, min_y, max_y) = bounds(&section.outer);
        let (span_x, span_y) = (max_x - min_x, max_y - min_y);
        let tolerance = 1.0e-6_f64.max(1.0e-9 * w.max(d));
        if span_x > w + tolerance || span_y > d + tolerance {
            return Err(format!(
                "outline spans {} x {} mm, which does not fit the member section {} x {} mm",
                trim(span_x),
                trim(span_y),
                trim(w),
                trim(d)
            ));
        }
    }
    Ok(section)
}

/// The shared ring rules: one outer ring and optional holes, read, checked and (by `limits`)
/// normalised. `outer_label` and `holes_label` are the names the messages use for the rings
/// (`outer`/`holes` for a section, `items[0].outline`/`items[0].holes` for an extrusion). `Err` is
/// the exact rule that failed; nothing is coerced or repaired.
pub fn parse_rings(
    outer_value: &Value,
    holes_value: Option<&Value>,
    outer_label: &str,
    holes_label: &str,
    limits: &RingLimits,
) -> Result<OutlineSection, String> {
    let subject = limits.subject;
    let max_holes = limits.max_holes;
    let hole_values: &[Value] = match holes_value {
        None | Some(Value::Null) => &[],
        Some(Value::Array(values)) => values,
        Some(_) => return Err(format!("`{holes_label}` must be an array of rings")),
    };
    if hole_values.len() > max_holes {
        return Err(format!(
            "{subject} has {} holes, at most {max_holes} are supported",
            hole_values.len()
        ));
    }

    // Refuse an absurd array before reading it, then count the vertices that remain once each
    // ring's optional closing repeat is dropped: a ring of exactly the limit of distinct vertices
    // plus its repeated first point is within the limit.
    if let Some(max_ring) = limits.max_points_per_ring {
        let rings = std::iter::once((outer_label.to_string(), outer_value)).chain(
            hole_values
                .iter()
                .enumerate()
                .map(|(index, hole)| (format!("{holes_label}[{index}]"), hole)),
        );
        for (label, ring) in rings {
            if point_count(ring) > max_ring + 1 {
                return Err(format!(
                    "{label} has {} points, at most {max_ring} are supported in one ring",
                    point_count(ring)
                ));
            }
        }
    }
    let reject_total = |total: usize, max_total: usize| {
        Err(format!(
            "{subject} has {total} points in all its rings, at most {max_total} are supported"
        ))
    };
    if let Some(max_total) = limits.max_points_total {
        let raw_total = hole_values
            .iter()
            .fold(point_count(outer_value), |sum, hole| {
                sum + point_count(hole)
            });
        if raw_total > max_total + 1 + max_holes {
            return reject_total(raw_total, max_total);
        }
    }
    let mut outer = read_ring(outer_value, outer_label, limits)?;
    let mut holes = Vec::with_capacity(hole_values.len());
    for (index, hole) in hole_values.iter().enumerate() {
        holes.push(read_ring(hole, &format!("{holes_label}[{index}]"), limits)?);
    }
    if let Some(max_ring) = limits.max_points_per_ring {
        let over = std::iter::once((outer_label.to_string(), &outer))
            .chain(
                holes
                    .iter()
                    .enumerate()
                    .map(|(index, hole)| (format!("{holes_label}[{index}]"), hole)),
            )
            .find(|(_, ring)| ring.len() > max_ring);
        if let Some((label, ring)) = over {
            return Err(format!(
                "{label} has {} points, at most {max_ring} are supported in one ring",
                ring.len()
            ));
        }
    }
    if let Some(max_total) = limits.max_points_total {
        let total = holes.iter().fold(outer.len(), |sum, hole| sum + hole.len());
        if total > max_total {
            return reject_total(total, max_total);
        }
    }
    check_ring(&outer, outer_label)?;
    for (index, hole) in holes.iter().enumerate() {
        check_ring(hole, &format!("{holes_label}[{index}]"))?;
    }
    if limits.winding == Winding::Require {
        if signed_area2(&outer) < 0.0 {
            return Err(format!(
                "{outer_label} winds clockwise; it must be counter-clockwise"
            ));
        }
        for (index, hole) in holes.iter().enumerate() {
            if signed_area2(hole) > 0.0 {
                return Err(format!(
                    "{holes_label}[{index}] winds counter-clockwise; a hole must be clockwise"
                ));
            }
        }
    }

    // Holes strictly inside the outer ring, and apart from each other.
    for (index, hole) in holes.iter().enumerate() {
        let label = format!("{holes_label}[{index}]");
        if rings_touch(&outer, hole) {
            return Err(format!("{label} touches or crosses the outer boundary"));
        }
        if hole.iter().any(|&point| !strictly_inside(point, &outer)) {
            return Err(format!("{label} is not inside the outer boundary"));
        }
    }
    // Two holes whose boxes lie more than TOLERANCE apart can neither touch nor nest, so the
    // pairwise edge test runs only where it can find something. That keeps 64 holes of 512 points
    // affordable without changing any answer.
    let boxes: Vec<_> = holes.iter().map(|hole| bounds(hole)).collect();
    for first in 0..holes.len() {
        for second in (first + 1)..holes.len() {
            if boxes_apart(boxes[first], boxes[second]) {
                continue;
            }
            if rings_touch(&holes[first], &holes[second]) {
                return Err(format!(
                    "{holes_label}[{first}] and {holes_label}[{second}] touch or overlap"
                ));
            }
            if strictly_inside(holes[first][0], &holes[second])
                || strictly_inside(holes[second][0], &holes[first])
            {
                return Err(format!(
                    "{holes_label}[{first}] and {holes_label}[{second}] lie one inside the other"
                ));
            }
        }
    }

    // Normalise (a no-op under `Winding::Require`, which has already checked it): outer
    // counter-clockwise, holes clockwise.
    if signed_area2(&outer) < 0.0 {
        outer.reverse();
    }
    for hole in &mut holes {
        if signed_area2(hole) > 0.0 {
            hole.reverse();
        }
    }
    Ok(OutlineSection { outer, holes })
}

/// `(min_x, max_x, min_y, max_y)` of a ring.
fn bounds(ring: &[Vec2]) -> (f64, f64, f64, f64) {
    ring.iter().fold(
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ),
        |(lx, hx, ly, hy), p| (lx.min(p[0]), hx.max(p[0]), ly.min(p[1]), hy.max(p[1])),
    )
}

/// Whether two bounding boxes are more than [`TOLERANCE`] apart on some axis.
fn boxes_apart(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.1 + TOLERANCE < b.0 || b.1 + TOLERANCE < a.0 || a.3 + TOLERANCE < b.2 || b.3 + TOLERANCE < a.2
}

/// The area of a validated ring set in square millimetres: the outer ring less its holes.
pub fn net_area_mm2(section: &OutlineSection) -> f64 {
    let outer = signed_area2(&section.outer).abs() / 2.0;
    let holes: f64 = section
        .holes
        .iter()
        .map(|hole| signed_area2(hole).abs() / 2.0)
        .sum();
    outer - holes
}

fn trim(value: f64) -> String {
    let text = format!("{value:.3}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

fn point_count(ring: &Value) -> usize {
    ring.as_array().map_or(0, Vec::len)
}

/// Twice the signed area (positive counter-clockwise), taken from the ring's own first vertex so a
/// small ring far from the axis keeps its sign and its precision.
fn signed_area2(ring: &[Vec2]) -> f64 {
    let Some(&origin) = ring.first() else {
        return 0.0;
    };
    polygon_edges(ring)
        .map(|(a, b)| {
            let (a, b) = (
                [a[0] - origin[0], a[1] - origin[1]],
                [b[0] - origin[0], b[1] - origin[1]],
            );
            a[0] * b[1] - b[0] * a[1]
        })
        .sum::<f64>()
}

fn strictly_inside(point: Vec2, ring: &[Vec2]) -> bool {
    point_in_polygon(point, ring)
        && polygon_edges(ring).all(|(a, b)| point_segment_distance(point, a, b) > TOLERANCE)
}

/// Whether segments `a`-`b` and `c`-`d` cross, or come within [`TOLERANCE`] of each other.
fn segments_meet(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let (o1, o2, o3, o4) = (
        cross2(a, b, c),
        cross2(a, b, d),
        cross2(c, d, a),
        cross2(c, d, b),
    );
    let opposite = |p: f64, q: f64| (p > 0.0 && q < 0.0) || (p < 0.0 && q > 0.0);
    (opposite(o1, o2) && opposite(o3, o4))
        || point_segment_distance(c, a, b) <= TOLERANCE
        || point_segment_distance(d, a, b) <= TOLERANCE
        || point_segment_distance(a, c, d) <= TOLERANCE
        || point_segment_distance(b, c, d) <= TOLERANCE
}

/// Whether any edge of one ring touches or crosses any edge of the other.
fn rings_touch(a: &[Vec2], b: &[Vec2]) -> bool {
    polygon_edges(a).any(|(p, q)| polygon_edges(b).any(|(r, s)| segments_meet(p, q, r, s)))
}

/// Read one ring's points: shape, finiteness and range only, with the optional closing repeat dropped.
fn read_ring(value: &Value, label: &str, limits: &RingLimits) -> Result<Vec<Vec2>, String> {
    let max_coordinate = limits.max_coordinate;
    let origin_name = limits.origin_name;
    let items = value
        .as_array()
        .ok_or_else(|| format!("{label} must be an array of [x, y] points"))?;
    let mut ring: Vec<Vec2> = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        let pair = item
            .as_array()
            .filter(|pair| pair.len() == 2)
            .ok_or_else(|| format!("{label}[{index}] must be an [x, y] pair"))?;
        let coordinate = |axis: usize| {
            pair[axis]
                .as_f64()
                .filter(|number| number.is_finite())
                .ok_or_else(|| format!("{label}[{index}] must hold finite numbers"))
                .and_then(|number| {
                    if number.abs() > max_coordinate {
                        Err(format!(
                            "{label}[{index}] is more than {max_coordinate} mm from {origin_name}"
                        ))
                    } else {
                        Ok(number)
                    }
                })
        };
        ring.push([coordinate(0)?, coordinate(1)?]);
    }
    if ring.len() > 1 && ring.first() == ring.last() {
        ring.pop();
    }
    if ring.len() < 3 {
        return Err(format!(
            "{label} needs at least 3 distinct points (got {})",
            ring.len()
        ));
    }
    Ok(ring)
}

/// The geometric rules for one ring: no repeated point, no spike, simple, and enclosing an area.
fn check_ring(ring: &[Vec2], label: &str) -> Result<(), String> {
    let count = ring.len();
    for index in 0..count {
        let (a, b, c) = (
            ring[index],
            ring[(index + 1) % count],
            ring[(index + 2) % count],
        );
        if (a[0] - b[0]).hypot(a[1] - b[1]) <= TOLERANCE {
            return Err(format!(
                "{label}[{index}] and {label}[{}] are the same point",
                (index + 1) % count
            ));
        }
        // Two consecutive edges that double back on each other leave a zero-width spike.
        let (u, v) = ([b[0] - a[0], b[1] - a[1]], [c[0] - b[0], c[1] - b[1]]);
        let collinear = cross2(a, b, c).abs() <= 1.0e-9 * u[0].hypot(u[1]) * v[0].hypot(v[1]);
        if collinear && u[0] * v[0] + u[1] * v[1] < 0.0 {
            return Err(format!(
                "{label} doubles back on itself at point {}",
                (index + 1) % count
            ));
        }
    }
    for first in 0..count {
        for second in (first + 1)..count {
            if second == first + 1 || (second + 1) % count == first {
                continue;
            }
            if segments_meet(
                ring[first],
                ring[(first + 1) % count],
                ring[second],
                ring[(second + 1) % count],
            ) {
                return Err(format!(
                    "{label} crosses or touches itself (edges {first} and {second})"
                ));
            }
        }
    }
    // Area from the ring's own first vertex, against the ring's own size, so a small section far
    // from the axis is judged as it is, not by its distance from the origin.
    let origin = ring[0];
    let extent = ring.iter().fold(0.0_f64, |m, p| {
        m.max((p[0] - origin[0]).abs())
            .max((p[1] - origin[1]).abs())
    });
    if signed_area2(ring).abs() / 2.0 <= TOLERANCE * extent {
        return Err(format!("{label} encloses no area"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn parse(xsection: Value, declared: Option<(f64, f64)>) -> Result<OutlineSection, String> {
        parse_outline(xsection.as_object().unwrap(), declared)
    }

    fn square(half: f64) -> Value {
        json!([[-half, -half], [half, -half], [half, half], [-half, half]])
    }

    fn err(xsection: Value, declared: Option<(f64, f64)>) -> String {
        parse(xsection, declared).unwrap_err()
    }

    #[test]
    fn a_trapezoid_with_no_holes_is_accepted_and_kept_counter_clockwise() {
        let section = parse(
            json!({"shape":"outline","outer":[[-150,-100],[150,-100],[75,100],[-75,100]]}),
            Some((300.0, 200.0)),
        )
        .unwrap();
        assert_eq!(section.outer.len(), 4);
        assert!(section.holes.is_empty());
        assert!(signed_area2(&section.outer) > 0.0);
        assert_eq!(section.outer[0], [-150.0, -100.0]);
    }

    #[test]
    fn either_winding_is_normalised_outer_ccw_holes_cw() {
        let clockwise_outer = json!([[-100, -100], [-100, 100], [100, 100], [100, -100]]);
        let counter_clockwise_hole = json!([[-20, -20], [20, -20], [20, 20], [-20, 20]]);
        let section = parse(
            json!({"shape":"outline","outer":clockwise_outer,"holes":[counter_clockwise_hole]}),
            None,
        )
        .unwrap();
        assert!(signed_area2(&section.outer) > 0.0);
        assert!(signed_area2(&section.holes[0]) < 0.0);
    }

    #[test]
    fn a_repeated_closing_point_is_dropped_and_holes_null_means_none() {
        let section = parse(
            json!({"shape":"outline","outer":[[0,0],[10,0],[10,10],[0,0]],"holes":null}),
            None,
        )
        .unwrap();
        assert_eq!(section.outer.len(), 3);
        assert!(section.holes.is_empty());
    }

    #[test]
    fn fewer_than_three_points_is_refused_with_the_count() {
        let reason = err(json!({"shape":"outline","outer":[[0,0],[10,0]]}), None);
        assert_eq!(reason, "outer needs at least 3 distinct points (got 2)");
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,0],[0,0]]}),
            None,
        );
        assert!(reason.contains("at least 3"), "{reason}");
    }

    #[test]
    fn a_missing_or_malformed_outer_is_refused() {
        assert!(err(json!({"shape":"outline"}), None).contains("needs `outer`"));
        assert!(err(json!({"shape":"outline","outer":"abc"}), None).contains("array of [x, y]"));
        assert!(
            err(json!({"shape":"outline","outer":[[0,0],[1],[2,2]]}), None).contains("outer[1]")
        );
        assert!(
            err(
                json!({"shape":"outline","outer":[[0,0],[1,"2"],[2,2]]}),
                None
            )
            .contains("finite numbers")
        );
        assert!(
            err(
                json!({"shape":"outline","outer":[[0,0],[1,0],[1,1]],"holes":{}}),
                None
            )
            .contains("`holes` must be an array")
        );
    }

    #[test]
    fn a_repeated_point_a_spike_and_a_zero_area_ring_are_refused() {
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,0],[10,0],[10,10]]}),
            None,
        );
        assert!(reason.contains("are the same point"), "{reason}");
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,0],[5,0],[5,10]]}),
            None,
        );
        assert!(reason.contains("doubles back"), "{reason}");
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[0.000015,0],[0,0.000015]]}),
            None,
        );
        assert!(reason.contains("encloses no area"), "{reason}");
        // Collinear points fold the ring back on itself.
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,0],[20,0]]}),
            None,
        );
        assert!(reason.contains("doubles back"), "{reason}");
    }

    #[test]
    fn a_self_crossing_bow_tie_and_a_self_touching_ring_are_refused() {
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,10],[10,0],[0,10]]}),
            None,
        );
        assert!(reason.contains("crosses or touches itself"), "{reason}");
        // A vertex resting on another edge.
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,0],[10,10],[5,0],[0,10]]}),
            None,
        );
        assert!(reason.contains("crosses or touches itself"), "{reason}");
    }

    #[test]
    fn a_hole_must_be_strictly_inside_the_outer_ring() {
        let outer = square(100.0);
        // Outside entirely.
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[[[200,200],[220,200],[220,220]]]}),
            None,
        );
        assert!(reason.contains("holes[0] is not inside"), "{reason}");
        // Crossing the boundary.
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[[[90,0],[110,0],[110,20],[90,20]]]}),
            None,
        );
        assert!(reason.contains("holes[0] touches or crosses"), "{reason}");
        // Touching the boundary from inside.
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[[[-100,0],[-80,0],[-80,20],[-100,20]]]}),
            None,
        );
        assert!(reason.contains("holes[0] touches or crosses"), "{reason}");
    }

    #[test]
    fn holes_must_not_touch_overlap_or_nest() {
        let outer = square(100.0);
        let a = json!([[-50, -50], [-10, -50], [-10, -10], [-50, -10]]);
        let overlapping = json!([[-30, -30], [20, -30], [20, 20], [-30, 20]]);
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[a, overlapping]}),
            None,
        );
        assert!(
            reason.contains("holes[0] and holes[1] touch or overlap"),
            "{reason}"
        );
        let touching = json!([[-10, -50], [30, -50], [30, -10], [-10, -10]]);
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[a, touching]}),
            None,
        );
        assert!(reason.contains("touch or overlap"), "{reason}");
        let nested = json!([[-40, -40], [-20, -40], [-20, -20], [-40, -20]]);
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[a, nested]}),
            None,
        );
        assert!(reason.contains("one inside the other"), "{reason}");
        // Two apart holes are fine.
        let apart = json!([[10, 10], [40, 10], [40, 40], [10, 40]]);
        assert!(
            parse(
                json!({"shape":"outline","outer":outer,"holes":[a, apart]}),
                None
            )
            .is_ok()
        );
    }

    #[test]
    fn at_most_sixteen_holes() {
        let hole = |index: usize| {
            let x = -90.0 + 10.0 * index as f64;
            json!([[x, -5], [x + 5.0, -5], [x + 5.0, 5], [x, 5]])
        };
        let sixteen: Vec<Value> = (0..16).map(hole).collect();
        assert!(
            parse(
                json!({"shape":"outline","outer":square(100.0),"holes":sixteen}),
                None
            )
            .is_ok()
        );
        let mut seventeen: Vec<Value> = (0..16).map(hole).collect();
        seventeen.push(json!([[85, 50], [90, 50], [90, 60], [85, 60]]));
        let reason = err(
            json!({"shape":"outline","outer":square(100.0),"holes":seventeen}),
            None,
        );
        assert!(reason.contains("17 holes, at most 16"), "{reason}");
    }

    #[test]
    fn a_coordinate_far_from_the_axis_is_refused() {
        let reason = err(
            json!({"shape":"outline","outer":[[0,0],[10,0],[2000000,10]]}),
            None,
        );
        assert!(reason.contains("outer[2] is more than"), "{reason}");
    }

    #[test]
    fn predicates_hold_at_large_coordinates() {
        // A 10 m square with a 1 m hole, 900 km from the axis: the same rules classify them as at the origin.
        let at = 900000.0;
        let ring = |x0: f64, y0: f64, size: f64| {
            json!([
                [x0, y0],
                [x0 + size, y0],
                [x0 + size, y0 + size],
                [x0, y0 + size]
            ])
        };
        let outer = ring(at, at, 10000.0);
        let hole = ring(at + 1000.0, at + 1000.0, 1000.0);
        assert!(
            parse(
                json!({"shape":"outline","outer":outer,"holes":[hole]}),
                None
            )
            .is_ok()
        );
        let touching = ring(at, at + 1000.0, 1000.0);
        assert!(
            err(
                json!({"shape":"outline","outer":outer,"holes":[touching]}),
                None
            )
            .contains("touches or crosses")
        );
    }

    #[test]
    fn a_gap_finer_than_the_ifc_precision_is_rejected_as_touching() {
        // A hole 2e-6 mm from the outer boundary: an IFC consumer at 1e-5 mm reads it as touching.
        let outer = json!([[0, 0], [100, 0], [100, 100], [0, 100]]);
        let hole = json!([[0.000002, 10], [10, 10], [10, 20], [0.000002, 20]]);
        let reason = err(
            json!({"shape":"outline","outer":outer,"holes":[hole]}),
            None,
        );
        assert!(reason.contains("touches or crosses"), "{reason}");
        // The same hole a clear 2e-5 mm in is accepted.
        let hole = json!([[0.00002, 10], [10, 10], [10, 20], [0.00002, 20]]);
        assert!(
            parse(
                json!({"shape":"outline","outer":outer,"holes":[hole]}),
                None
            )
            .is_ok()
        );
    }

    #[test]
    fn a_small_ring_far_from_the_axis_still_encloses_its_area() {
        let at = 900000.0;
        let reason = parse(
            json!({"shape":"outline","outer":[[at, at],[at + 0.5, at],[at + 0.5, at + 0.5],[at, at + 0.5]]}),
            None,
        );
        assert!(reason.is_ok(), "{reason:?}");
    }

    #[test]
    fn winding_is_normalised_from_the_rings_own_vertex_far_from_the_axis() {
        // A clockwise 0.0001 mm square at (900000, 900000): its sign must not be lost to the distance
        // from the origin.
        let at = 900000.0;
        let d = 0.001;
        let section = parse(
            json!({"shape":"outline","outer":[[at, at],[at, at + d],[at + d, at + d],[at + d, at]]}),
            None,
        )
        .unwrap();
        assert!(signed_area2(&section.outer) > 0.0);
        assert_eq!(section.outer[0], [at + d, at]);
    }

    #[test]
    fn a_closing_repeat_does_not_count_against_the_point_budget() {
        // MAX_POINTS distinct vertices plus the repeated first point: within the limit, so the only
        // refusal is the geometry (these points are collinear), never the budget.
        let mut points: Vec<Value> = (0..MAX_POINTS).map(|i| json!([i, 0])).collect();
        points.push(json!([0, 0]));
        let reason = err(json!({"shape":"outline","outer":points}), None);
        assert!(!reason.contains("at most 8192"), "{reason}");
        // One more distinct vertex is over it.
        let over: Vec<Value> = (0..=MAX_POINTS).map(|i| json!([i, 0])).collect();
        assert!(err(json!({"shape":"outline","outer":over}), None).contains("at most 8192"));
    }

    #[test]
    fn the_total_point_budget_is_enforced() {
        let many: Vec<Value> = (0..=MAX_POINTS).map(|i| json!([i, 0])).collect();
        let reason = err(json!({"shape":"outline","outer":many}), None);
        assert!(reason.contains("at most 8192"), "{reason}");
    }

    #[test]
    fn the_outline_must_fit_a_declared_section_but_need_not_fill_or_centre_it() {
        let trapezoid = json!([[-150, -100], [150, -100], [75, 100], [-75, 100]]);
        // Exactly the envelope, and a larger one: both fit.
        assert!(
            parse(
                json!({"shape":"outline","outer":trapezoid}),
                Some((300.0, 200.0))
            )
            .is_ok()
        );
        assert!(
            parse(
                json!({"shape":"outline","outer":trapezoid}),
                Some((400.0, 300.0))
            )
            .is_ok()
        );
        // An eccentric outline (origin on its corner) fits a box of its own extent.
        assert!(
            parse(
                json!({"shape":"outline","outer":[[0,0],[100,0],[100,50],[0,50]]}),
                Some((100.0, 50.0))
            )
            .is_ok()
        );
        // Too wide, too deep.
        let reason = err(
            json!({"shape":"outline","outer":trapezoid}),
            Some((299.0, 200.0)),
        );
        assert!(
            reason
                .contains("spans 300 x 200 mm, which does not fit the member section 299 x 200 mm"),
            "{reason}"
        );
        assert!(
            err(
                json!({"shape":"outline","outer":trapezoid}),
                Some((300.0, 199.0))
            )
            .contains("does not fit")
        );
        // No declared section: nothing to check against.
        assert!(parse(json!({"shape":"outline","outer":trapezoid}), None).is_ok());
    }

    #[test]
    fn declared_envelope_needs_both_positive_finite_numbers() {
        assert_eq!(
            declared_envelope(Some(&json!({"w": 300, "d": 200}))),
            Some((300.0, 200.0))
        );
        assert_eq!(declared_envelope(Some(&json!({"w": 300}))), None);
        assert_eq!(declared_envelope(Some(&json!({"w": 300, "d": 0}))), None);
        assert_eq!(declared_envelope(Some(&json!({"w": "3", "d": 2}))), None);
        assert_eq!(declared_envelope(Some(&json!(7))), None);
        assert_eq!(declared_envelope(None), None);
    }
}
