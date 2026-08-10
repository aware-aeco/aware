//! The plain 2D/3D vector arithmetic the render nodes share.
//!
//! `render::ifc` and `render::viewer_3d` consume the *same* scene JSON and each
//! grew its own copy of the same eleven primitives — dot/cross/length/distance,
//! the polygon walk, the segment-intersection test, the simple-polygon guard.
//! The two copies were the same maths written against two different spellings of
//! a vector: `ifc` used tuples (`(f64, f64, f64)`), `viewer_3d` used fixed arrays
//! (`[f64; 3]`). Nothing but the spelling differed, so a fix to one (an epsilon,
//! a degenerate-edge case) silently missed the other.
//!
//! Arrays win as the single spelling: they index (`v[i]`), they `map`, and they
//! iterate, so the bodies below stay short. Both callers now use these, and the
//! substrate stays generic — this is scene geometry, not any one host's.

/// A point or direction in the XY plane.
pub type Vec2 = [f64; 2];

/// A point or direction in space.
pub type Vec3 = [f64; 3];

/// The tolerance shared by the degenerate-geometry guards below. Lifted from the
/// two copies, which already agreed on it.
pub const EPS: f64 = 1.0e-9;

// ─────────────────────────────── 3D ───────────────────────────────

pub fn add3(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn scale3(a: Vec3, s: f64) -> Vec3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

pub fn dot3(a: Vec3, b: Vec3) -> f64 {
    (0..3).map(|index| a[index] * b[index]).sum()
}

pub fn cross3(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn length3(value: Vec3) -> f64 {
    dot3(value, value).sqrt()
}

pub fn distance3(a: Vec3, b: Vec3) -> f64 {
    length3([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
}

/// The unit vector along `value`, or `None` when it is degenerate or non-finite.
///
/// Both former copies rejected a length at or below [`EPS`]; the `is_finite`
/// check comes from the `ifc` copy and is kept, because a NaN component would
/// otherwise propagate a NaN direction into the emitted geometry.
pub fn normalized3(value: Vec3) -> Option<Vec3> {
    let length = length3(value);
    (length.is_finite() && length > EPS).then(|| value.map(|component| component / length))
}

// ─────────────────────────────── 2D ───────────────────────────────

/// The closed polygon's edges, wrapping the last vertex back to the first.
pub fn polygon_edges(polygon: &[Vec2]) -> impl Iterator<Item = (Vec2, Vec2)> + '_ {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied().cycle().skip(1))
        .take(polygon.len())
}

/// Even-odd (ray casting) containment test.
pub fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let mut inside = false;
    for (a, b) in polygon_edges(polygon) {
        if (a[1] > point[1]) != (b[1] > point[1])
            && point[0] < (b[0] - a[0]) * (point[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            inside = !inside;
        }
    }
    inside
}

/// Distance from `point` to the segment `a`–`b` (to the nearer endpoint when the
/// segment is degenerate).
pub fn point_segment_distance(point: Vec2, a: Vec2, b: Vec2) -> f64 {
    let delta = [b[0] - a[0], b[1] - a[1]];
    let length_sq = delta[0] * delta[0] + delta[1] * delta[1];
    if length_sq <= EPS * EPS {
        return (point[0] - a[0]).hypot(point[1] - a[1]);
    }
    let t =
        (((point[0] - a[0]) * delta[0] + (point[1] - a[1]) * delta[1]) / length_sq).clamp(0.0, 1.0);
    let nearest = [a[0] + t * delta[0], a[1] + t * delta[1]];
    (point[0] - nearest[0]).hypot(point[1] - nearest[1])
}

/// Twice the signed area of triangle `a`,`b`,`c` — positive when `c` is left of
/// `a`→`b`. The orientation test the segment code below is built on.
pub fn cross2(a: Vec2, b: Vec2, c: Vec2) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

/// Whether segments `a`–`b` and `c`–`d` touch or cross, collinear cases included.
pub fn segments_intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let on_segment = |p: Vec2, q: Vec2, r: Vec2| {
        cross2(p, q, r).abs() <= EPS
            && r[0] >= p[0].min(q[0]) - EPS
            && r[0] <= p[0].max(q[0]) + EPS
            && r[1] >= p[1].min(q[1]) - EPS
            && r[1] <= p[1].max(q[1]) + EPS
    };
    let (ab_c, ab_d, cd_a, cd_b) = (
        cross2(a, b, c),
        cross2(a, b, d),
        cross2(c, d, a),
        cross2(c, d, b),
    );
    (ab_c.signum() != ab_d.signum() && cd_a.signum() != cd_b.signum())
        || on_segment(a, b, c)
        || on_segment(a, b, d)
        || on_segment(c, d, a)
        || on_segment(c, d, b)
}

/// Whether the polygon is a simple ring with non-zero area: at least three
/// vertices, no zero-length edge, non-degenerate area, and no self-intersection
/// between non-adjacent edges.
pub fn polygon_is_simple_nonzero(polygon: &[Vec2]) -> bool {
    if polygon.len() < 3
        || polygon_edges(polygon).any(|(a, b)| (a[0] - b[0]).hypot(a[1] - b[1]) <= EPS)
    {
        return false;
    }
    let twice_area = polygon_edges(polygon)
        .map(|(a, b)| a[0] * b[1] - b[0] * a[1])
        .sum::<f64>();
    if twice_area.abs() <= EPS {
        return false;
    }
    let n = polygon.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if j == (i + 1) % n || (j + 1) % n == i {
                continue;
            }
            if segments_intersect(
                polygon[i],
                polygon[(i + 1) % n],
                polygon[j],
                polygon[(j + 1) % n],
            ) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross3_is_right_handed_and_orthogonal() {
        let x = [1.0, 0.0, 0.0];
        let y = [0.0, 1.0, 0.0];
        assert_eq!(cross3(x, y), [0.0, 0.0, 1.0]);
        assert_eq!(dot3(cross3(x, y), x), 0.0);
    }

    #[test]
    fn length_and_distance_agree_with_pythagoras() {
        assert_eq!(length3([3.0, 4.0, 0.0]), 5.0);
        assert_eq!(distance3([1.0, 1.0, 1.0], [4.0, 5.0, 1.0]), 5.0);
    }

    #[test]
    fn add_and_scale_are_componentwise() {
        assert_eq!(
            add3([1.0, 2.0, 3.0], [10.0, 20.0, 30.0]),
            [11.0, 22.0, 33.0]
        );
        assert_eq!(scale3([1.0, 2.0, 3.0], 2.0), [2.0, 4.0, 6.0]);
    }

    #[test]
    fn normalized3_rejects_degenerate_and_non_finite_input() {
        assert_eq!(normalized3([0.0, 5.0, 0.0]), Some([0.0, 1.0, 0.0]));
        assert_eq!(normalized3([0.0, 0.0, 0.0]), None);
        assert_eq!(normalized3([EPS / 2.0, 0.0, 0.0]), None);
        // A NaN component makes the length NaN, and a NaN direction must not reach
        // the emitters — `> EPS` alone is false for NaN, but the explicit
        // `is_finite` also covers an infinite component.
        assert_eq!(normalized3([f64::INFINITY, 0.0, 0.0]), None);
        assert_eq!(normalized3([f64::NAN, 0.0, 0.0]), None);
    }

    #[test]
    fn polygon_edges_closes_the_ring() {
        let square = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let edges: Vec<_> = polygon_edges(&square).collect();
        assert_eq!(edges.len(), 4);
        assert_eq!(edges[3], ([0.0, 1.0], [0.0, 0.0]));
    }

    #[test]
    fn point_in_polygon_separates_inside_from_outside() {
        let square = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
        assert!(point_in_polygon([1.0, 1.0], &square));
        assert!(!point_in_polygon([3.0, 1.0], &square));
    }

    #[test]
    fn point_segment_distance_clamps_to_the_endpoints() {
        assert_eq!(
            point_segment_distance([1.0, 1.0], [0.0, 0.0], [2.0, 0.0]),
            1.0
        );
        assert_eq!(
            point_segment_distance([5.0, 0.0], [0.0, 0.0], [2.0, 0.0]),
            3.0
        );
        // Degenerate segment: fall back to the distance to the collapsed point.
        assert_eq!(
            point_segment_distance([3.0, 4.0], [0.0, 0.0], [0.0, 0.0]),
            5.0
        );
    }

    #[test]
    fn segments_intersect_covers_crossing_touching_and_disjoint() {
        assert!(segments_intersect(
            [0.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
            [2.0, 0.0]
        ));
        // Collinear overlap counts as touching.
        assert!(segments_intersect(
            [0.0, 0.0],
            [2.0, 0.0],
            [1.0, 0.0],
            [3.0, 0.0]
        ));
        assert!(!segments_intersect(
            [0.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0]
        ));
    }

    #[test]
    fn polygon_is_simple_nonzero_rejects_the_degenerate_rings() {
        let square = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
        assert!(polygon_is_simple_nonzero(&square));
        // Bow-tie: non-adjacent edges cross.
        let bowtie = [[0.0, 0.0], [2.0, 2.0], [2.0, 0.0], [0.0, 2.0]];
        assert!(!polygon_is_simple_nonzero(&bowtie));
        // Fewer than three vertices, a repeated vertex, and zero area.
        assert!(!polygon_is_simple_nonzero(&[[0.0, 0.0], [1.0, 0.0]]));
        assert!(!polygon_is_simple_nonzero(&[
            [0.0, 0.0],
            [0.0, 0.0],
            [1.0, 1.0]
        ]));
        assert!(!polygon_is_simple_nonzero(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [2.0, 0.0]
        ]));
    }
}
