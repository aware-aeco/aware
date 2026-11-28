use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer, Serialize, de};

pub const LIMIT: usize = 32 * 1024 * 1024;
type Vec3 = [f64; 3];

fn decimal<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
    let value = String::deserialize(deserializer)?;
    let unsigned = value.strip_prefix(['-', '+']).unwrap_or(&value);
    let (whole, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let digits = whole.len() + fraction.len();
    if digits == 0
        || digits > 32
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err(de::Error::custom(
            "expected a decimal with at most 32 digits",
        ));
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Decimal(#[serde(deserialize_with = "decimal")] ());

fn nullable_base64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let value = Option::<String>::deserialize(deserializer)?;
    if let Some(encoded) = &value {
        STANDARD
            .decode(encoded)
            .map_err(|_| de::Error::custom("brep_b64 requires standard padded base64"))?;
    }
    Ok(value)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    units: String,
    origin_mm: Vec3,
    axes: [Vec3; 3],
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

impl Frame {
    fn validate(&self) -> Result<(), &'static str> {
        if self.units != "mm" {
            return Err("frame units must be mm");
        }
        let [a, b, c] = self.axes;
        let cross = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];
        let tolerance = -1e-12..=1e-12;
        if self.axes.iter().enumerate().any(|(i, &a)| {
            self.axes
                .iter()
                .enumerate()
                .any(|(j, &b)| !tolerance.contains(&(dot(a, b) - f64::from(i == j))))
        }) || !tolerance.contains(&(dot(cross, c) - 1.))
        {
            return Err("frame must have a right-handed orthonormal basis");
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Mesh {
    positions_mm: Vec<Vec3>,
    triangles: Vec<[u64; 3]>,
}

impl Mesh {
    fn validate(&self) -> Result<(), &'static str> {
        if self.triangles.len() > 100_000 {
            return Err("mesh exceeds 100,000 triangles");
        }
        if self
            .triangles
            .iter()
            .flatten()
            .any(|&index| index > (1 << 53) - 1 || index >= self.positions_mm.len() as u64)
        {
            return Err("mesh contains an invalid index");
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    xy_mm: Vec<[Decimal; 2]>,
    #[serde(rename = "z_mm")]
    _z_mm: [Decimal; 2],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operand {
    pub id: String,
    #[serde(rename = "offset_mm")]
    _offset_mm: Vec3,
    profile: Profile,
    #[serde(deserialize_with = "nullable_base64")]
    pub brep_b64: Option<String>,
    mesh: Mesh,
}

impl Operand {
    fn validate(&self) -> Result<(), &'static str> {
        if self.id.is_empty() {
            return Err("operand id must be nonempty");
        }
        if self.profile.xy_mm.len() < 3 {
            return Err("profile needs at least three vertices");
        }
        self.mesh.validate()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    schema: String,
    op: String,
    kind: String,
    pub frame: Frame,
    chord_height_mm: f64,
    pub a: Operand,
    pub b: Operand,
}

impl Request {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema != "aware.geometry.exec.request/v1"
            || self.op != "boolean"
            || self.kind != "difference"
        {
            return Err("expected aware.geometry.exec.request/v1 boolean difference");
        }
        if self.chord_height_mm <= 0. {
            return Err("chord_height_mm must be positive");
        }
        self.frame.validate()?;
        self.a.validate()?;
        self.b.validate()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cut {
    offset_mm: Vec3,
    #[serde(deserialize_with = "nullable_base64")]
    pub brep_b64: Option<String>,
    mesh: Mesh,
    volume_mm3: f64,
}

impl Cut {
    pub fn validate(&self, exact: bool) -> bool {
        if self.mesh.validate().is_err() {
            return false;
        }
        if self.volume_mm3 == 0. {
            self.brep_b64.is_none()
                && self.mesh.positions_mm.is_empty()
                && self.mesh.triangles.is_empty()
        } else {
            self.volume_mm3 > 0.
                && !self.mesh.positions_mm.is_empty()
                && !self.mesh.triangles.is_empty()
                && if exact {
                    self.brep_b64.as_ref().is_some_and(|data| !data.is_empty())
                } else {
                    self.brep_b64.is_none()
                }
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WarningCode {
    OcctRuntimeMissing,
    OcctImportFailed,
    OcctTimeout,
    OcctWorkerFailed,
    BrepValidationFailed,
    BrepInputMissing,
    MeshInvalid,
    MeshWorkerFailed,
    MeshTimeout,
    BudgetExceeded,
    ChordHeightUnverified,
    CoincidenceMismatch,
}

impl WarningCode {
    pub fn message(self) -> &'static str {
        match self {
            Self::OcctRuntimeMissing => "The verified OCCT runtime is unavailable.",
            Self::OcctImportFailed => "The OCCT worker could not import its pinned runtime.",
            Self::OcctTimeout => "The OCCT worker exceeded its deadline.",
            Self::OcctWorkerFailed => "The OCCT worker did not complete with a valid reply.",
            Self::BrepValidationFailed => "The exact candidate failed validation.",
            Self::BrepInputMissing => "At least one operand has no BRep.",
            Self::MeshInvalid => "The mesh failed geometry validation.",
            Self::MeshWorkerFailed => "The staged mesh worker did not complete with a valid reply.",
            Self::MeshTimeout => "The staged mesh worker exceeded its deadline.",
            Self::BudgetExceeded => "The pair exceeded the geometry precision or resource budget.",
            Self::ChordHeightUnverified => "The mesh path did not verify chord height.",
            Self::CoincidenceMismatch => "A declared coincidence changed in kernel coordinates.",
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    BaseCap,
    TopCap,
    SideIncidence,
    VertexAlignment,
}

impl Relation {
    pub fn name(self) -> &'static str {
        match self {
            Self::BaseCap => "base_cap",
            Self::TopCap => "top_cap",
            Self::SideIncidence => "side_incidence",
            Self::VertexAlignment => "vertex_alignment",
        }
    }
}

fn required_nullable<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    Option::deserialize(deserializer)
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReplyStatus {
    Ok,
    Rejected,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChildReply {
    status: ReplyStatus,
    #[serde(deserialize_with = "required_nullable")]
    code: Option<WarningCode>,
    #[serde(deserialize_with = "required_nullable")]
    relation: Option<Relation>,
    #[serde(deserialize_with = "required_nullable")]
    result: Option<Cut>,
}

pub type Rejection = (WarningCode, Option<Relation>);

pub fn parse_child(bytes: &[u8], exact: bool) -> Result<Cut, Rejection> {
    use WarningCode::*;
    let invalid = (
        if exact {
            OcctWorkerFailed
        } else {
            MeshWorkerFailed
        },
        None,
    );
    let reply: ChildReply = serde_json::from_slice(bytes).map_err(|_| invalid)?;
    match (reply.status, reply.code, reply.relation, reply.result) {
        (ReplyStatus::Ok, None, None, Some(result)) if result.validate(exact) => Ok(result),
        (ReplyStatus::Rejected, Some(CoincidenceMismatch), Some(relation), None) => {
            Err((CoincidenceMismatch, Some(relation)))
        }
        (ReplyStatus::Rejected, Some(code), None, None)
            if code == BudgetExceeded
                || exact && matches!(code, BrepValidationFailed | OcctImportFailed)
                || !exact && matches!(code, MeshInvalid | MeshWorkerFailed) =>
        {
            Err((code, None))
        }
        _ => Err(invalid),
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeCandidate {
    status: NativeStatus,
    code: (),
    relation: (),
    result: Cut,
    native_boundary: Vec<Vec<Vec<Vec3>>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum NativeStatus {
    Ok,
}

impl NativeCandidate {
    pub fn verification_input(self, request: &[u8]) -> Result<Vec<u8>, Rejection> {
        if !self.result.validate(true)
            || self.native_boundary.is_empty() != self.result.mesh.triangles.is_empty()
            || self.native_boundary.iter().any(|face| {
                face.is_empty()
                    || face
                        .iter()
                        .any(|wire| wire.len() < 3 || wire.iter().flatten().any(|v| !v.is_finite()))
            })
            || self
                .native_boundary
                .iter()
                .flatten()
                .map(Vec::len)
                .sum::<usize>()
                > 300_000
        {
            return Err((WarningCode::OcctWorkerFailed, None));
        }
        let mut bytes = Vec::with_capacity(request.len());
        bytes.extend_from_slice(b"{\"request\":");
        bytes.extend_from_slice(request);
        bytes.extend_from_slice(b",\"result\":");
        serde_json::to_writer(&mut bytes, &self.result)
            .map_err(|_| (WarningCode::OcctWorkerFailed, None))?;
        bytes.extend_from_slice(b",\"native_boundary\":");
        serde_json::to_writer(&mut bytes, &self.native_boundary)
            .map_err(|_| (WarningCode::OcctWorkerFailed, None))?;
        bytes.push(b'}');
        if bytes.len() > LIMIT {
            return Err((WarningCode::BudgetExceeded, None));
        }
        Ok(bytes)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    #[serde(rename = "status")]
    _status: Admitted,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Admitted {
    Admitted,
}
