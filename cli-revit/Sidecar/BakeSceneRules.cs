// ── AWARE Revit bake-scene — host-neutral rules ──────────────────────────────
// The half of `bake-scene` that has nothing to do with the Revit API: units,
// profile-name matching, id validation, receipt rows, and the retire-and-replace
// reconciliation decision.
//
// This file is compiled INTO aware-revit.exe (so the unit tests exercise it
// directly) AND embedded verbatim as source into the Roslyn script that runs
// inside Revit (see BakeSceneScript.Code). One text, two compilations — so the
// rules the tests pin are byte-for-byte the rules that mutate the model.
//
// CONSTRAINTS, because the same text must compile as a C# *script*:
//   * no `namespace` (scripts forbid them) and no `using` directives (a script
//     requires every using to precede all other code; the script's header owns
//     them).
//   * only BCL types the add-in's ScriptEngine puts on the reference list:
//     CoreLib, System.Linq, System.Text.Json. No Revit types. No
//     System.Security.Cryptography — the materialization hash is computed by
//     the sidecar, which has the whole framework.
#nullable disable

/// <summary>Pure decision rules shared by the sidecar and the in-Revit bake script.</summary>
internal static class AwareBakeRules
{
    // ── Ownership: Revit Extensible Storage ──────────────────────────────────
    // Tekla stamps ownership onto USER_FIELD UDAs. Revit's equivalent is an
    // Extensible Storage Entity on each created element. The GUID is STABLE
    // FOREVER: it is how a later bake recognises the elements a previous bake
    // owns, and therefore what makes bake-scene retire-and-replace instead of
    // duplicate. Changing it orphans every element AWARE has ever created.
    internal const string SchemaGuid = "9C4C7F1E-1B3A-4E7D-9A2F-6D5B8C0E4A11";
    internal const string SchemaName = "AwareBakeV1";
    internal const string SchemaVendorId = "AWRE"; // matches AwareRevit.addin <VendorId>
    internal const string FieldSourceId = "SourceId";
    internal const string FieldSceneHash = "SceneHash";
    internal const string FieldMaterializationHash = "MaterializationHash";
    internal const string FieldRecordId = "RecordId";

    /// <summary>Every Extensible Storage field this bake writes and reads back.</summary>
    internal static string[] SchemaFields()
    {
        return new[] { FieldSourceId, FieldSceneHash, FieldMaterializationHash, FieldRecordId };
    }

    // ── Units ────────────────────────────────────────────────────────────────
    // Revit's internal length unit is the decimal FOOT; the canonical scene is
    // millimetres. This is the ONLY place the conversion is written — every
    // coordinate in the bake goes through MmToFeet.
    internal const double MmPerFoot = 304.8;

    internal static double MmToFeet(double mm)
    {
        return mm / MmPerFoot;
    }

    internal static double FeetToMm(double feet)
    {
        return feet * MmPerFoot;
    }

    // ── Profile-name matching ────────────────────────────────────────────────
    // LOAD-BEARING: a live Revit document ships type names in inconsistent case
    // — `W16X26` next to `W16x50` in the same family — so a case-sensitive match
    // silently loses half the members to the DirectShape fallback. Every lookup
    // goes through the same normalisation: upper-case, all whitespace removed.
    internal static string NormalizeProfile(string name)
    {
        if (string.IsNullOrEmpty(name)) return "";
        var chars = new System.Text.StringBuilder(name.Length);
        foreach (var c in name)
        {
            if (char.IsWhiteSpace(c)) continue;
            chars.Append(char.ToUpperInvariant(c));
        }
        return chars.ToString();
    }

    /// <summary>Index candidate names by their normalised form. First occurrence
    /// wins, so the result is stable for a given enumeration order.</summary>
    internal static Dictionary<string, T> IndexByNormalizedName<T>(IEnumerable<KeyValuePair<string, T>> named)
    {
        var index = new Dictionary<string, T>(StringComparer.Ordinal);
        if (named == null) return index;
        foreach (var pair in named)
        {
            var key = NormalizeProfile(pair.Key);
            if (key.Length == 0) continue;
            if (index.ContainsKey(key)) continue;
            index[key] = pair.Value;
        }
        return index;
    }

    internal static bool TryResolveProfile<T>(Dictionary<string, T> index, string profile, out T match)
    {
        match = default(T);
        if (index == null) return false;
        var key = NormalizeProfile(profile);
        if (key.Length == 0) return false;
        return index.TryGetValue(key, out match);
    }

    // ── Record ids ───────────────────────────────────────────────────────────
    // Same rule as the Tekla sink: trimmed, 1-256 UTF-8 bytes, no control
    // characters. Uniqueness is enforced by the caller's `seen` set.
    internal static bool IsAcceptableId(string id)
    {
        if (string.IsNullOrWhiteSpace(id)) return false;
        if (id != id.Trim()) return false;
        if (System.Text.Encoding.UTF8.GetByteCount(id) > 256) return false;
        foreach (var c in id)
        {
            if (c < 32 || c == 127) return false;
        }
        return true;
    }

    // ── Receipt rows ─────────────────────────────────────────────────────────
    // Same shape as the Tekla sink's `row(...)`: {id, kind, status} plus optional
    // code/message. Returned as a plain dictionary so the add-in's ResultSerializer
    // walks it as JSON rather than reflecting over an anonymous type.
    internal static Dictionary<string, object> Row(string id, string kind, string status, string code, string message)
    {
        var r = new Dictionary<string, object>(StringComparer.Ordinal);
        r["id"] = id;
        r["kind"] = kind;
        r["status"] = status;
        if (!string.IsNullOrEmpty(code)) r["code"] = code;
        if (!string.IsNullOrEmpty(message)) r["message"] = message;
        return r;
    }

    // ── Column vs beam ───────────────────────────────────────────────────────
    // A member whose axis is within VerticalToleranceDegrees of global Z is a
    // column (OST_StructuralColumns / StructuralType.Column); everything else is
    // framing (OST_StructuralFraming / StructuralType.Beam).
    internal const double VerticalToleranceDegrees = 1.0;

    internal static bool IsVerticalAxis(double dx, double dy, double dz)
    {
        var length = Math.Sqrt(dx * dx + dy * dy + dz * dz);
        if (!(length > 0)) return false;
        var cosine = Math.Abs(dz) / length;
        return cosine >= Math.Cos(VerticalToleranceDegrees * Math.PI / 180.0);
    }

    // ── Level choice ─────────────────────────────────────────────────────────
    /// <summary>Index of the nearest level at or below <paramref name="zFeet"/>.
    /// Returns the LOWEST level when the member starts below every level (the
    /// caller reports that as a warning), and -1 when the document has none —
    /// which the caller turns into a clean failure, since Revit cannot place a
    /// structural family instance without a level.</summary>
    internal static int PickLevelIndex(IList<double> ascendingElevationsFeet, double zFeet)
    {
        if (ascendingElevationsFeet == null || ascendingElevationsFeet.Count == 0) return -1;
        var chosen = -1;
        for (var i = 0; i < ascendingElevationsFeet.Count; i++)
        {
            if (ascendingElevationsFeet[i] <= zFeet + 1e-9) chosen = i;
        }
        return chosen < 0 ? 0 : chosen;
    }

    // ── Fallback solid section ───────────────────────────────────────────────
    // When no loaded family type matches the designation the member becomes a
    // DirectShape carrying an extruded prismatic solid. Its cross-section is a
    // PLACEHOLDER: the nominal depth read off the designation, with a width
    // taken from the designation only where the designation actually carries one
    // (boxes, angles, and the British UB/UC families). It is a marker that keeps
    // the member visible and selectable, never a section model.
    internal const double FallbackSectionDepthMm = 200.0;
    internal const double FallbackSectionWidthMm = 100.0;

    static readonly string[] ImperialPrefixes = { "W", "M", "S", "HP", "C", "MC", "WT", "MT", "ST", "L", "HSS", "PIPE" };
    static readonly string[] MetricPrefixes = { "UB", "UC", "UKB", "UKC", "UBP", "IPE", "IPN", "HE", "HEA", "HEB", "HEM", "HD", "UPN", "PFC", "RHS", "SHS", "CHS" };
    // Designations whose SECOND `X`-separated token is a width rather than a
    // mass-per-length or a wall thickness.
    static readonly string[] SecondTokenIsWidthPrefixes = { "HSS", "RHS", "SHS", "L", "UB", "UC", "UKB", "UKC", "UBP" };

    /// <summary>Nominal placeholder section for a designation. The unit is decided
    /// by the letter PREFIX (imperial AISC families are inches, European families
    /// are millimetres) — never by the magnitude of the number, because IPE80 and
    /// W44 would both be misread that way.</summary>
    internal static bool TryParseNominalSectionMm(string designation, out double depthMm, out double widthMm)
    {
        depthMm = FallbackSectionDepthMm;
        widthMm = FallbackSectionWidthMm;

        var norm = NormalizeProfile(designation);
        if (norm.Length == 0) return false;

        var split = 0;
        while (split < norm.Length && !char.IsDigit(norm[split])) split++;
        if (split == 0 || split >= norm.Length) return false;

        var prefix = norm.Substring(0, split);
        var imperial = ImperialPrefixes.Contains(prefix);
        var metric = MetricPrefixes.Contains(prefix);
        if (!imperial && !metric) return false;

        var tokens = norm.Substring(split).Split('X');
        double first;
        if (!TryLeadingNumber(tokens[0], out first) || !(first > 0)) return false;

        var scale = imperial ? 25.4 : 1.0;
        depthMm = first * scale;

        double second;
        if (tokens.Length > 1
            && SecondTokenIsWidthPrefixes.Contains(prefix)
            && TryLeadingNumber(tokens[1], out second)
            && second > 0)
        {
            widthMm = second * scale;
        }
        else
        {
            widthMm = depthMm / 2.0;
        }
        return true;
    }

    internal static bool TryLeadingNumber(string token, out double value)
    {
        value = 0;
        if (string.IsNullOrEmpty(token)) return false;
        var end = 0;
        while (end < token.Length && (char.IsDigit(token[end]) || token[end] == '.')) end++;
        if (end == 0) return false;
        return double.TryParse(
            token.Substring(0, end),
            System.Globalization.NumberStyles.Float,
            System.Globalization.CultureInfo.InvariantCulture,
            out value);
    }

    // ── Retire-and-replace ───────────────────────────────────────────────────
    // bake-scene is a REPLACEMENT, not an append: floless bakes each drag-drop
    // under its own derived source id precisely so that re-baking the same source
    // supersedes what it produced last time instead of piling duplicates into the
    // model. Get this wrong and a bake deletes the user's model.
    //
    // Two safety properties, both pinned by tests:
    //   1. an element owned by a DIFFERENT sourceId is never touched;
    //   2. an element this bake just created is never deleted, even if the
    //      collector happened to see it (belt and braces — the caller also reads
    //      the owned set BEFORE creating anything).
    // Everything else that this source owns retires, because every incoming
    // record was just re-created from scratch: a still-present record id is
    // `superseded`, an absent one is `removed`.
    internal sealed class OwnedRecord
    {
        public long ElementId;
        public string SourceId = "";
        public string RecordId = "";
    }

    internal sealed class RetirementRow
    {
        public long ElementId;
        public string RecordId = "";
        public string Reason = "";
    }

    internal sealed class RetirementPlan
    {
        public List<RetirementRow> Delete = new List<RetirementRow>();
        public List<RetirementRow> Keep = new List<RetirementRow>();
    }

    internal const string ReasonSuperseded = "superseded";
    internal const string ReasonRemoved = "removed";
    internal const string ReasonForeignSource = "foreign-source";
    internal const string ReasonJustCreated = "just-created";

    internal static RetirementPlan PlanRetirement(
        IEnumerable<OwnedRecord> owned,
        string sourceId,
        IEnumerable<string> incomingRecordIds,
        IEnumerable<long> stagedElementIds)
    {
        var plan = new RetirementPlan();
        if (owned == null) return plan;

        var incoming = new HashSet<string>(StringComparer.Ordinal);
        if (incomingRecordIds != null)
        {
            foreach (var id in incomingRecordIds)
            {
                if (!string.IsNullOrEmpty(id)) incoming.Add(id);
            }
        }
        var staged = new HashSet<long>();
        if (stagedElementIds != null)
        {
            foreach (var id in stagedElementIds) staged.Add(id);
        }

        foreach (var record in owned)
        {
            if (record == null) continue;
            var row = new RetirementRow
            {
                ElementId = record.ElementId,
                RecordId = record.RecordId ?? "",
            };

            if (!string.Equals(record.SourceId ?? "", sourceId ?? "", StringComparison.Ordinal))
            {
                row.Reason = ReasonForeignSource;
                plan.Keep.Add(row);
                continue;
            }
            if (staged.Contains(record.ElementId))
            {
                row.Reason = ReasonJustCreated;
                plan.Keep.Add(row);
                continue;
            }
            row.Reason = incoming.Contains(row.RecordId) ? ReasonSuperseded : ReasonRemoved;
            plan.Delete.Add(row);
        }
        return plan;
    }

    // ── Scene parsing ────────────────────────────────────────────────────────
    // Mirrors the Tekla sink's preflight: complete trust-boundary validation
    // BEFORE the first model write, so malformed geometry or identity never
    // starts a batch. Kinds the Revit sink cannot materialize yet become
    // `unsupported` rows — the receipt always accounts for them, they are never
    // silently dropped.
    internal sealed class MemberRecord
    {
        public string Id = "";
        public string Kind = "";
        public double[] FromMm = new double[3];
        public double[] ToMm = new double[3];
        public string Profile = "";
        public string Name = "";
    }

    internal sealed class RecordRef
    {
        public string Id = "";
        public string Kind = "";
    }

    internal sealed class ParsedScene
    {
        public string Name = "";
        public string Units = "";
        public string SourceId = "";
        public string SceneHash = "";
        public List<MemberRecord> Members = new List<MemberRecord>();
        public List<RecordRef> SupportedOrder = new List<RecordRef>();
        public List<Dictionary<string, object>> Failed = new List<Dictionary<string, object>>();
        public List<Dictionary<string, object>> Unsupported = new List<Dictionary<string, object>>();
        public List<Dictionary<string, object>> Warnings = new List<Dictionary<string, object>>();
        public bool Ok { get { return Failed.Count == 0; } }
    }

    /// <summary>Kinds the Revit sink materializes today. Everything else the
    /// canonical scene can carry (plate, rod, bolt-shank, washer, nut, bolt-head,
    /// and every operation/reference system) is reported as `unsupported`.</summary>
    internal static bool IsSupportedElementKind(string kind)
    {
        return kind == "member" || kind == "line" || kind == "box";
    }

    internal static ParsedScene ParseScene(System.Text.Json.JsonElement scene)
    {
        var parsed = new ParsedScene();
        if (scene.ValueKind != System.Text.Json.JsonValueKind.Object)
        {
            parsed.Failed.Add(Row("scene", "scene", "failed", "invalid-scene", "scene must be an object"));
            return parsed;
        }

        System.Text.Json.JsonElement meta;
        var hasMeta = scene.TryGetProperty("meta", out meta)
                      && meta.ValueKind == System.Text.Json.JsonValueKind.Object;
        parsed.Name = hasMeta ? Str(meta, "name") : "";
        if (string.IsNullOrWhiteSpace(parsed.Name)) parsed.Name = "Steel from Drawings";
        parsed.Units = hasMeta ? Str(meta, "units") : "";
        parsed.SourceId = (hasMeta ? Str(meta, "sourceId") : "").Trim();
        parsed.SceneHash = (hasMeta ? Str(meta, "sceneHash") : "").Trim();

        // An absent `units` means legacy millimetres (same as the Tekla sink);
        // an EXPLICIT non-mm unit is refused before any model mutation.
        if (!string.IsNullOrEmpty(parsed.Units)
            && !string.Equals(parsed.Units, "mm", StringComparison.OrdinalIgnoreCase))
        {
            parsed.Failed.Add(Row("scene", "scene", "failed", "unsupported-units",
                "Revit bake-scene accepts only millimetres"));
        }
        if (string.IsNullOrWhiteSpace(parsed.SourceId))
            parsed.Failed.Add(Row("scene", "scene", "failed", "missing-source-id", "scene.meta.sourceId is required"));
        if (string.IsNullOrWhiteSpace(parsed.SceneHash))
            parsed.Failed.Add(Row("scene", "scene", "failed", "missing-scene-hash", "scene.meta.sceneHash is required"));

        var seen = new HashSet<string>(StringComparer.Ordinal);
        Func<string, string, bool> acceptId = (id, kind) =>
        {
            if (!IsAcceptableId(id))
            {
                parsed.Failed.Add(Row(string.IsNullOrEmpty(id) ? "scene" : id, kind, "failed", "invalid-id",
                    "id must be unique, trimmed, 1-256 UTF-8 bytes, and contain no control characters"));
                return false;
            }
            if (!seen.Add(id))
            {
                parsed.Failed.Add(Row(id, kind, "failed", "duplicate-id", "id is duplicated in the scene"));
                return false;
            }
            return true;
        };

        ParseElements(scene, parsed, acceptId);
        // Which nested collections each family of records carries is fixed by the canonical scene,
        // and is the SAME set the Tekla sink walks: a bolt array owns instances, and each instance
        // owns the hole effects it cuts into its participants; a reference system owns axes and
        // levels. Every one of those is an addressable record with its own id, so the receipt has to
        // carry a row for it — a receipt that skips records cannot prove WHICH scene was applied.
        ParseUnsupportedCollection(scene, parsed, acceptId, "operations", "operation", "unsupported-operation",
            "operation kind is not supported by the Revit sink",
            (record, kind) =>
            {
                if (kind == "bolt-array")
                    ParseUnsupportedChildren(record, parsed, acceptId, "instances", "bolt-instance", "holeEffects", "opening");
            });
        ParseUnsupportedCollection(scene, parsed, acceptId, "referenceSystems", "reference-system",
            "unsupported-reference-system", "reference system kind is not supported by the Revit sink",
            (record, kind) =>
            {
                ParseUnsupportedChildren(record, parsed, acceptId, "axes", "grid-axis", null, null);
                ParseUnsupportedChildren(record, parsed, acceptId, "levels", "grid-level", null, null);
            });
        return parsed;
    }

    static void ParseElements(
        System.Text.Json.JsonElement scene,
        ParsedScene parsed,
        Func<string, string, bool> acceptId)
    {
        System.Text.Json.JsonElement elements;
        if (!scene.TryGetProperty("elements", out elements)) return;
        if (elements.ValueKind != System.Text.Json.JsonValueKind.Array)
        {
            parsed.Failed.Add(Row("scene", "scene", "failed", "invalid-collection", "scene.elements must be an array"));
            return;
        }

        foreach (var el in elements.EnumerateArray())
        {
            if (el.ValueKind != System.Text.Json.JsonValueKind.Object)
            {
                parsed.Failed.Add(Row("scene", "element", "failed", "invalid-record", "element must be an object"));
                continue;
            }
            var id = Str(el, "id");
            var kind = Str(el, "kind").ToLowerInvariant();
            if (string.IsNullOrEmpty(kind)) kind = "member";
            if (!acceptId(id, kind)) continue;

            if (!IsSupportedElementKind(kind))
            {
                parsed.Unsupported.Add(Row(id, kind, "unsupported", "unsupported-kind",
                    "element kind is not supported by the Revit sink"));
                // Children still get a row apiece so nothing vanishes unaccounted for. Only a plate
                // carries `holes` in the canonical scene, so only a plate's are walked — an id the
                // scene never declared would be as unreconcilable as one it skipped.
                if (kind == "plate")
                    ParseUnsupportedChildren(el, parsed, acceptId, "holes", "opening", null, null);
                continue;
            }

            var member = ReadMember(el, id, kind, parsed);
            if (member == null) continue;
            parsed.Members.Add(member);
            parsed.SupportedOrder.Add(new RecordRef { Id = id, Kind = kind });
        }
    }

    static MemberRecord ReadMember(
        System.Text.Json.JsonElement el,
        string id,
        string kind,
        ParsedScene parsed)
    {
        System.Text.Json.JsonElement metaEl;
        var hasMeta = el.TryGetProperty("meta", out metaEl)
                      && metaEl.ValueKind == System.Text.Json.JsonValueKind.Object;

        var from = Vec3(el, "from");
        var to = Vec3(el, "to");
        var profile = Str(el, "profile");
        if (string.IsNullOrWhiteSpace(profile) && hasMeta) profile = Str(metaEl, "profile");
        if (string.IsNullOrWhiteSpace(profile) && hasMeta) profile = Str(metaEl, "name");

        if (from == null || to == null || !(Distance(from, to) > 1e-6) || string.IsNullOrWhiteSpace(profile))
        {
            parsed.Failed.Add(Row(id, kind, "failed", "invalid-geometry",
                "member requires a nonzero finite axis and an exact profile"));
            return null;
        }

        var name = Str(el, "name");
        if (string.IsNullOrWhiteSpace(name) && hasMeta) name = Str(metaEl, "name");

        return new MemberRecord
        {
            Id = id,
            Kind = kind,
            FromMm = from,
            ToMm = to,
            Profile = profile.Trim(),
            Name = name ?? "",
        };
    }

    /// <param name="parseChildren">Walks the nested records THIS collection's parents carry, given
    /// the parent element and its kind. Passed in rather than hardcoded because the nested
    /// collections are per-family: an operation owns `instances`, a reference system owns
    /// `axes`/`levels`, and walking one family's collections on the other would invent rows for ids
    /// the scene never declared.</param>
    static void ParseUnsupportedCollection(
        System.Text.Json.JsonElement scene,
        ParsedScene parsed,
        Func<string, string, bool> acceptId,
        string property,
        string recordLabel,
        string code,
        string message,
        Action<System.Text.Json.JsonElement, string> parseChildren)
    {
        System.Text.Json.JsonElement collection;
        if (!scene.TryGetProperty(property, out collection)) return;
        if (collection.ValueKind != System.Text.Json.JsonValueKind.Array)
        {
            parsed.Failed.Add(Row("scene", "scene", "failed", "invalid-collection",
                "scene." + property + " must be an array"));
            return;
        }
        foreach (var record in collection.EnumerateArray())
        {
            if (record.ValueKind != System.Text.Json.JsonValueKind.Object)
            {
                parsed.Failed.Add(Row("scene", recordLabel, "failed", "invalid-record",
                    recordLabel + " must be an object"));
                continue;
            }
            var id = Str(record, "id");
            var kind = Str(record, "kind").ToLowerInvariant();
            if (string.IsNullOrEmpty(kind)) kind = recordLabel;
            if (!acceptId(id, kind)) continue;
            parsed.Unsupported.Add(Row(id, kind, "unsupported", code, message));
            parseChildren(record, kind);
        }
    }

    /// <param name="grandchildProperty">A collection nested one level deeper inside each child —
    /// a bolt instance's `holeEffects`. Null when the child carries none. Bolt holes live TWO levels
    /// below the operation, which is exactly why they were the records this receipt used to drop.</param>
    static void ParseUnsupportedChildren(
        System.Text.Json.JsonElement parent,
        ParsedScene parsed,
        Func<string, string, bool> acceptId,
        string property,
        string childKind,
        string grandchildProperty,
        string grandchildKind)
    {
        System.Text.Json.JsonElement children;
        if (!parent.TryGetProperty(property, out children)) return;
        if (children.ValueKind != System.Text.Json.JsonValueKind.Array)
        {
            parsed.Failed.Add(Row(Str(parent, "id"), childKind, "failed", "invalid-collection",
                property + " must be an array"));
            return;
        }
        foreach (var child in children.EnumerateArray())
        {
            if (child.ValueKind != System.Text.Json.JsonValueKind.Object)
            {
                parsed.Failed.Add(Row(Str(parent, "id"), childKind, "failed", "invalid-record",
                    childKind + " must be an object"));
                continue;
            }
            var id = Str(child, "id");
            if (acceptId(id, childKind))
                parsed.Unsupported.Add(Row(id, childKind, "unsupported", "unsupported-parent",
                    "the parent record is not supported by the Revit sink"));
            // A rejected child id must NOT cost its grandchildren their rows. The hole effects under
            // a bolt instance are records in their own right: skipping them would leave valid ids
            // unclassified and — worse — let a duplicate among THEM go undetected. Both reference
            // sinks (cli-tekla, cli-rhino) walk them whatever the instance id did.
            if (grandchildProperty != null)
                ParseUnsupportedChildren(child, parsed, acceptId, grandchildProperty, grandchildKind, null, null);
        }
    }

    /// <summary>Give every supported record that never got its own verdict a
    /// `batch-aborted` row, so a refused bake still accounts for the whole scene.</summary>
    internal static void AppendBatchAborted(ParsedScene parsed, string message)
    {
        var already = new HashSet<string>(StringComparer.Ordinal);
        foreach (var row in parsed.Failed)
        {
            object id;
            if (row.TryGetValue("id", out id) && id != null) already.Add(id.ToString());
        }
        foreach (var record in parsed.SupportedOrder)
        {
            if (already.Add(record.Id))
                parsed.Failed.Add(Row(record.Id, record.Kind, "failed", "batch-aborted", message));
        }
    }

    // ── Small JSON helpers ───────────────────────────────────────────────────
    internal static string Str(System.Text.Json.JsonElement obj, string property)
    {
        System.Text.Json.JsonElement value;
        if (obj.ValueKind != System.Text.Json.JsonValueKind.Object) return "";
        if (!obj.TryGetProperty(property, out value)) return "";
        if (value.ValueKind == System.Text.Json.JsonValueKind.String) return value.GetString() ?? "";
        if (value.ValueKind == System.Text.Json.JsonValueKind.Number) return value.ToString();
        return "";
    }

    /// <summary>A finite 3-vector, or null when the property is missing, short, or
    /// carries a non-finite number.</summary>
    internal static double[] Vec3(System.Text.Json.JsonElement obj, string property)
    {
        System.Text.Json.JsonElement value;
        if (obj.ValueKind != System.Text.Json.JsonValueKind.Object) return null;
        if (!obj.TryGetProperty(property, out value)) return null;
        if (value.ValueKind != System.Text.Json.JsonValueKind.Array) return null;
        if (value.GetArrayLength() < 3) return null;
        var vec = new double[3];
        var i = 0;
        foreach (var item in value.EnumerateArray())
        {
            if (i > 2) break;
            double component;
            if (!TryNumber(item, out component)) return null;
            vec[i] = component;
            i++;
        }
        return i >= 3 ? vec : null;
    }

    internal static bool TryNumber(System.Text.Json.JsonElement value, out double number)
    {
        number = 0;
        if (value.ValueKind == System.Text.Json.JsonValueKind.Number)
        {
            if (!value.TryGetDouble(out number)) return false;
        }
        else if (value.ValueKind == System.Text.Json.JsonValueKind.String)
        {
            if (!double.TryParse(value.GetString(), System.Globalization.NumberStyles.Float,
                    System.Globalization.CultureInfo.InvariantCulture, out number)) return false;
        }
        else
        {
            return false;
        }
        return !double.IsNaN(number) && !double.IsInfinity(number);
    }

    internal static double Distance(double[] a, double[] b)
    {
        var dx = a[0] - b[0];
        var dy = a[1] - b[1];
        var dz = a[2] - b[2];
        return Math.Sqrt(dx * dx + dy * dy + dz * dz);
    }
}
