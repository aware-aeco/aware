// Canonical scene-to-Revit materializer. It deliberately lives as a Roslyn
// script — the same path the `exec` verb uses — so the sidecar stays version-
// neutral across Revit 2025 and 2026 and the bake runs inside the add-in's
// Revit API context (the only place the API may be touched).
//
// The script is assembled from three parts:
//   1. Header      — the `using` directives a script must declare first.
//   2. RulesSource — BakeSceneRules.cs, embedded verbatim as a resource. That
//                    file is ALSO compiled into this assembly, so the unit tests
//                    exercise exactly the text that runs inside Revit.
//   3. Body        — the Revit-specific half: symbols, levels, placement,
//                    Extensible Storage ownership, and the single Transaction.

using System.Reflection;

namespace AwareRevit.Sidecar;

internal static class BakeSceneScript
{
    const string RulesResourceName = "BakeSceneRules.cs";

    static string? _code;

    /// <summary>The complete C# script shipped to the in-Revit add-in.</summary>
    internal static string Code => _code ??= Header + LoadRulesSource() + Body;

    /// <summary>The host-neutral rules, read back out of this assembly's
    /// resources. Missing = a broken build, and a broken build must not ship a
    /// half-script that would compile-error inside the user's model.</summary>
    internal static string LoadRulesSource()
    {
        var asm = typeof(BakeSceneScript).GetTypeInfo().Assembly;
        using var stream = asm.GetManifestResourceStream(RulesResourceName);
        if (stream is null)
        {
            throw new InvalidOperationException(
                $"aware-revit is built without the embedded bake-scene rules source '{RulesResourceName}'");
        }
        using var reader = new StreamReader(stream);
        return reader.ReadToEnd();
    }

    /// <summary>Script-leading `using` directives. Internal so the compile-check
    /// test can splice stub globals in between the usings and the rest.</summary>
    internal const string Header = """
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Autodesk.Revit.DB;
using Autodesk.Revit.DB.ExtensibleStorage;
using Autodesk.Revit.DB.Structure;
using Autodesk.Revit.UI;

""";

    internal const string Body = """

// ── bake-scene body (runs on Revit's main thread, inside the add-in) ─────────
// Materializes the canonical millimetre scene as source-owned native Revit
// elements and retires whatever the same source owned before.
//
// DELIBERATELY NO TaskDialog / no status announce anywhere in this script.
// Tekla's bake-scene takes a `label` and calls Operation.DisplayPrompt because
// Tekla has a non-blocking status bar. Revit has no status-bar equivalent, and
// TaskDialog is MODAL: announcing "adding N objects..." would block the very
// operation it is announcing until a human clicks OK, inside an open
// Transaction, on the API thread. Do not "fix" this by adding one.

var attemptId = Guid.NewGuid().ToString("N");
var noRows = new List<Dictionary<string, object>>();
var emitted = new List<Dictionary<string, object>>();
var warnings = new List<Dictionary<string, object>>();
var retired = new List<Dictionary<string, object>>();
var materializationHash = "";
var modelName = "";
var sourceId = "";
var sceneHash = "";
var sceneName = "";

Dictionary<string, object> Envelope(
    bool ok,
    List<Dictionary<string, object>> emittedRows,
    List<Dictionary<string, object>> failedRows,
    List<Dictionary<string, object>> unsupportedRows,
    List<Dictionary<string, object>> warningRows)
{
    var r = new Dictionary<string, object>(StringComparer.Ordinal);
    r["ok"] = ok;
    r["host"] = "revit";
    r["verb"] = "bake-scene";
    r["attemptId"] = attemptId;
    r["sourceId"] = sourceId;
    r["sceneHash"] = sceneHash;
    r["materializationHash"] = materializationHash;
    r["scene_name"] = sceneName;
    r["model"] = modelName;
    r["created"] = emittedRows.Count;
    r["retired_count"] = retired.Count;
    r["failed_count"] = failedRows.Count;
    r["skipped"] = unsupportedRows.Count;
    r["emitted"] = emittedRows;
    r["failed"] = failedRows;
    r["unsupported"] = unsupportedRows;
    r["warnings"] = warningRows;
    r["retired"] = retired;
    return r;
}

// ── host ────────────────────────────────────────────────────────────────────
UIDocument uidoc = uiapp == null ? null : uiapp.ActiveUIDocument;
Document doc = uidoc == null ? null : uidoc.Document;
if (doc == null)
{
    return Envelope(false, noRows,
        new List<Dictionary<string, object>> {
            AwareBakeRules.Row("scene", "scene", "failed", "host-unavailable", "No Revit document is open.") },
        noRows, noRows);
}
if (doc.IsFamilyDocument)
{
    return Envelope(false, noRows,
        new List<Dictionary<string, object>> {
            AwareBakeRules.Row("scene", "scene", "failed", "host-unavailable",
                "The active Revit document is a family document; bake-scene needs a project model.") },
        noRows, noRows);
}
modelName = string.IsNullOrEmpty(doc.Title) ? "Revit model" : doc.Title;

// ── request ─────────────────────────────────────────────────────────────────
object sceneRaw = null;
if (args == null || !args.TryGetValue("scene", out sceneRaw) || sceneRaw == null)
{
    return Envelope(false, noRows,
        new List<Dictionary<string, object>> {
            AwareBakeRules.Row("scene", "scene", "failed", "invalid-scene", "scene must be an object") },
        noRows, noRows);
}
// The pipe hands `args` values across as JsonElement; keep the in-process shape
// working too by round-tripping anything else through System.Text.Json.
JsonElement sceneEl = sceneRaw is JsonElement
    ? (JsonElement)sceneRaw
    : JsonSerializer.SerializeToElement(sceneRaw);

object hashRaw = null;
if (args.TryGetValue("materializationHash", out hashRaw) && hashRaw != null)
{
    materializationHash = hashRaw is JsonElement
        ? (((JsonElement)hashRaw).ValueKind == JsonValueKind.String
            ? ((JsonElement)hashRaw).GetString()
            : ((JsonElement)hashRaw).ToString())
        : hashRaw.ToString();
}

// ── preflight: complete validation before the first model write ─────────────
var parsed = AwareBakeRules.ParseScene(sceneEl);
sourceId = parsed.SourceId;
sceneHash = parsed.SceneHash;
sceneName = parsed.Name;
warnings = parsed.Warnings;
if (!parsed.Ok)
{
    AwareBakeRules.AppendBatchAborted(parsed,
        "Batch was aborted because scene identity or envelope validation failed.");
    return Envelope(false, noRows, parsed.Failed, parsed.Unsupported, parsed.Warnings);
}

// ── levels ──────────────────────────────────────────────────────────────────
// Revit cannot place a structural family instance without a level, so an empty
// document fails cleanly here instead of throwing halfway through the batch.
var levels = new FilteredElementCollector(doc)
    .OfClass(typeof(Level))
    .Cast<Level>()
    .OrderBy(l => l.Elevation)
    .ToList();
if (levels.Count == 0)
{
    parsed.Failed.Add(AwareBakeRules.Row("scene", "scene", "failed", "no-levels",
        "The Revit document has no Level. bake-scene places structural members against a level; add at least one level and re-run."));
    AwareBakeRules.AppendBatchAborted(parsed, "Batch was aborted because the document has no level to place members against.");
    return Envelope(false, noRows, parsed.Failed, parsed.Unsupported, parsed.Warnings);
}
var levelElevations = levels.Select(l => l.Elevation).ToList();

// ── family symbols, indexed by NORMALISED name ──────────────────────────────
// A live document ships `W16X26` alongside `W16x50`; matching case-sensitively
// would silently push half the members into the DirectShape fallback.
var framingPairs = new List<KeyValuePair<string, FamilySymbol>>();
var columnPairs = new List<KeyValuePair<string, FamilySymbol>>();
foreach (var symbol in new FilteredElementCollector(doc).OfClass(typeof(FamilySymbol)).Cast<FamilySymbol>())
{
    var category = symbol.Category;
    if (category == null) continue;
    var builtIn = (BuiltInCategory)category.Id.Value;
    if (builtIn == BuiltInCategory.OST_StructuralFraming)
        framingPairs.Add(new KeyValuePair<string, FamilySymbol>(symbol.Name, symbol));
    else if (builtIn == BuiltInCategory.OST_StructuralColumns)
        columnPairs.Add(new KeyValuePair<string, FamilySymbol>(symbol.Name, symbol));
}
var framingIndex = AwareBakeRules.IndexByNormalizedName(framingPairs);
var columnIndex = AwareBakeRules.IndexByNormalizedName(columnPairs);

// ── ownership schema ────────────────────────────────────────────────────────
Schema EnsureSchema(List<Dictionary<string, object>> failures)
{
    var guid = new Guid(AwareBakeRules.SchemaGuid);
    var existing = Schema.Lookup(guid);
    if (existing != null)
    {
        foreach (var fieldName in AwareBakeRules.SchemaFields())
        {
            if (existing.GetField(fieldName) == null)
            {
                failures.Add(AwareBakeRules.Row("scene", "scene", "failed", "schema-conflict",
                    "An incompatible `" + AwareBakeRules.SchemaName + "` Extensible Storage schema is already loaded (missing field `"
                    + fieldName + "`). Restart Revit and re-run."));
                return null;
            }
        }
        return existing;
    }
    var builder = new SchemaBuilder(guid);
    builder.SetSchemaName(AwareBakeRules.SchemaName);
    builder.SetVendorId(AwareBakeRules.SchemaVendorId);
    builder.SetReadAccessLevel(AccessLevel.Public);
    builder.SetWriteAccessLevel(AccessLevel.Public);
    foreach (var fieldName in AwareBakeRules.SchemaFields())
        builder.AddSimpleField(fieldName, typeof(string));
    return builder.Finish();
}

var schemaFailures = new List<Dictionary<string, object>>();
var schema = EnsureSchema(schemaFailures);
if (schema == null)
{
    foreach (var row in schemaFailures) parsed.Failed.Add(row);
    AwareBakeRules.AppendBatchAborted(parsed, "Batch was aborted because ownership storage could not be prepared.");
    return Envelope(false, noRows, parsed.Failed, parsed.Unsupported, parsed.Warnings);
}
var fieldSourceId = schema.GetField(AwareBakeRules.FieldSourceId);
var fieldSceneHash = schema.GetField(AwareBakeRules.FieldSceneHash);
var fieldMaterializationHash = schema.GetField(AwareBakeRules.FieldMaterializationHash);
var fieldRecordId = schema.GetField(AwareBakeRules.FieldRecordId);

// The only categories this bake ever creates, and therefore the only ones that
// can carry its ownership entity.
var ownedCategories = new List<BuiltInCategory> {
    BuiltInCategory.OST_StructuralFraming,
    BuiltInCategory.OST_StructuralColumns,
};

// ── geometry helpers ────────────────────────────────────────────────────────
XYZ ToFeet(double[] mm)
{
    return new XYZ(AwareBakeRules.MmToFeet(mm[0]), AwareBakeRules.MmToFeet(mm[1]), AwareBakeRules.MmToFeet(mm[2]));
}

Solid PlaceholderSolid(XYZ start, XYZ end, double depthMm, double widthMm)
{
    var axis = end - start;
    var length = axis.GetLength();
    var direction = axis.Normalize();
    var seed = Math.Abs(direction.Z) > 0.9 ? XYZ.BasisX : XYZ.BasisZ;
    var side = direction.CrossProduct(seed).Normalize();
    var up = side.CrossProduct(direction).Normalize();
    var halfWidth = AwareBakeRules.MmToFeet(widthMm) / 2.0;
    var halfDepth = AwareBakeRules.MmToFeet(depthMm) / 2.0;
    var a = start + side * halfWidth + up * halfDepth;
    var b = start - side * halfWidth + up * halfDepth;
    var c = start - side * halfWidth - up * halfDepth;
    var d = start + side * halfWidth - up * halfDepth;
    var loop = CurveLoop.Create(new List<Curve> {
        Line.CreateBound(a, b), Line.CreateBound(b, c), Line.CreateBound(c, d), Line.CreateBound(d, a) });
    return GeometryCreationUtilities.CreateExtrusionGeometry(new List<CurveLoop> { loop }, direction, length);
}

// ── the single transaction ──────────────────────────────────────────────────
// One Transaction for the whole bake, so the user's Undo takes the entire thing
// back in one step and a mid-batch failure rolls back completely (unlike Tekla,
// Revit really does have rollback — the receipt can say so honestly).
var activeId = parsed.SupportedOrder.Count > 0 ? parsed.SupportedOrder[0].Id : "scene";
using (var tx = new Transaction(doc, "AWARE bake-scene " + sceneName))
{
    tx.Start();
    // Never force a modal failure dialog: this runs unattended from the CLI and
    // a modal would hang the API thread inside an open transaction.
    var failureOptions = tx.GetFailureHandlingOptions();
    failureOptions.SetForcedModalHandling(false);
    failureOptions.SetClearAfterRollback(true);
    failureOptions.SetDelayedMiniWarnings(true);
    tx.SetFailureHandlingOptions(failureOptions);

    try
    {
        // Read the prior owned set BEFORE creating anything, so nothing this
        // bake stages can possibly land in the retirement candidates.
        var owned = new List<AwareBakeRules.OwnedRecord>();
        var ownedFilter = new ElementMulticategoryFilter(ownedCategories);
        foreach (var element in new FilteredElementCollector(doc).WherePasses(ownedFilter).WhereElementIsNotElementType())
        {
            var entity = element.GetEntity(schema);
            if (entity == null || !entity.IsValid()) continue;
            owned.Add(new AwareBakeRules.OwnedRecord {
                ElementId = element.Id.Value,
                SourceId = entity.Get<string>(fieldSourceId) ?? "",
                RecordId = entity.Get<string>(fieldRecordId) ?? "",
            });
        }

        var stagedIds = new List<long>();
        foreach (var member in parsed.Members)
        {
            activeId = member.Id;
            var start = ToFeet(member.FromMm);
            var end = ToFeet(member.ToMm);
            var vertical = AwareBakeRules.IsVerticalAxis(
                member.ToMm[0] - member.FromMm[0],
                member.ToMm[1] - member.FromMm[1],
                member.ToMm[2] - member.FromMm[2]);

            var levelIndex = AwareBakeRules.PickLevelIndex(levelElevations, start.Z);
            var level = levels[levelIndex];
            if (start.Z + 1e-9 < levelElevations[levelIndex])
            {
                warnings.Add(AwareBakeRules.Row(member.Id, member.Kind, "warning", "level-below-lowest",
                    "The member starts below every level in the model; it was hosted on the lowest level `" + level.Name + "`."));
            }

            FamilySymbol symbol = null;
            var index = vertical ? columnIndex : framingIndex;
            var matched = AwareBakeRules.TryResolveProfile(index, member.Profile, out symbol);

            Element created;
            var usedFallback = false;
            if (matched)
            {
                if (!symbol.IsActive)
                {
                    symbol.Activate();
                    doc.Regenerate();
                }
                created = doc.Create.NewFamilyInstance(
                    Line.CreateBound(start, end),
                    symbol,
                    level,
                    vertical ? StructuralType.Column : StructuralType.Beam);
            }
            else
            {
                // Never a silent drop: with no loaded family for this designation
                // the member still lands, as a DirectShape carrying an extruded
                // placeholder solid, and says so in the receipt.
                usedFallback = true;
                double depthMm;
                double widthMm;
                var recognised = AwareBakeRules.TryParseNominalSectionMm(member.Profile, out depthMm, out widthMm);
                var directShape = DirectShape.CreateElement(doc, new ElementId(BuiltInCategory.OST_StructuralFraming));
                directShape.SetShape(new List<GeometryObject> { PlaceholderSolid(start, end, depthMm, widthMm) });
                created = directShape;
                warnings.Add(AwareBakeRules.Row(member.Id, member.Kind, "warning", "profile-family-not-loaded",
                    "No loaded Revit family type matches profile `" + member.Profile + "`"
                    + (vertical ? " in Structural Columns" : " in Structural Framing")
                    + ". A placeholder DirectShape solid was created instead ("
                    + (recognised ? "nominal" : "default") + " section "
                    + depthMm.ToString("0.#", System.Globalization.CultureInfo.InvariantCulture) + " x "
                    + widthMm.ToString("0.#", System.Globalization.CultureInfo.InvariantCulture)
                    + " mm). Load the family to get the real section."));
            }

            if (created == null)
                throw new Exception(member.Id + ": Revit returned no element for the member");

            var stamp = new Entity(schema);
            stamp.Set<string>(fieldSourceId, sourceId);
            stamp.Set<string>(fieldSceneHash, sceneHash);
            stamp.Set<string>(fieldMaterializationHash, materializationHash);
            stamp.Set<string>(fieldRecordId, member.Id);
            created.SetEntity(stamp);
            stagedIds.Add(created.Id.Value);

            var row = AwareBakeRules.Row(member.Id, member.Kind, "emitted", "", "");
            row["elementId"] = created.Id.Value;
            row["profile"] = member.Profile;
            row["category"] = usedFallback
                ? "OST_StructuralFraming"
                : (vertical ? "OST_StructuralColumns" : "OST_StructuralFraming");
            row["fallback"] = usedFallback;
            if (matched) row["symbol"] = symbol.Name;
            row["level"] = level.Name;
            emitted.Add(row);
        }

        // Retire-and-replace: every element this source owned that is not one of
        // the elements just created goes. A foreign source is never touched.
        activeId = "scene";
        var incomingIds = parsed.Members.Select(m => m.Id).ToList();
        var plan = AwareBakeRules.PlanRetirement(owned, sourceId, incomingIds, stagedIds);
        if (plan.Delete.Count > 0)
        {
            var doomed = plan.Delete.Select(r => new ElementId(r.ElementId)).ToList();
            doc.Delete(doomed);
        }
        foreach (var retirement in plan.Delete)
        {
            var row = AwareBakeRules.Row(retirement.RecordId, "retired", "retired", retirement.Reason, "");
            row["elementId"] = retirement.ElementId;
            retired.Add(row);
        }

        // Commit REPORTS its outcome rather than always throwing. When Revit's failure processing
        // rolls the transaction back — or leaves it pending — Commit() returns that status quietly,
        // and treating the call as fire-and-forget would hand back ok:true with a list of element
        // ids for geometry the model does not have. Same failure this bake already guards at the
        // envelope: a write that did not land must never read as success.
        var commitStatus = tx.Commit();
        if (commitStatus != TransactionStatus.Committed)
        {
            throw new Exception(
                "Revit did not commit the bake (transaction status: " + commitStatus + "). "
                + "Nothing was written; this is usually Revit's failure processing rejecting the "
                + "geometry or a warning dialog resolving to a rollback.");
        }
    }
    catch (Exception ex)
    {
        if (tx.HasStarted() && !tx.HasEnded()) tx.RollBack();
        retired.Clear();
        var failures = new List<Dictionary<string, object>>();
        var causeAssigned = false;
        foreach (var record in parsed.SupportedOrder)
        {
            var cause = !causeAssigned && record.Id == activeId;
            var row = AwareBakeRules.Row(record.Id, record.Kind, "failed",
                cause ? "materialization-failed" : "batch-aborted",
                cause ? ex.Message : "Batch was aborted after another supported record failed.");
            row["rolledBack"] = true;
            failures.Add(row);
            if (cause) causeAssigned = true;
        }
        if (!causeAssigned)
        {
            var row = AwareBakeRules.Row(activeId, "scene", "failed", "materialization-failed", ex.Message);
            row["rolledBack"] = true;
            failures.Insert(0, row);
        }
        return Envelope(false, noRows, failures, parsed.Unsupported, warnings);
    }
}

return Envelope(true, emitted, noRows, parsed.Unsupported, warnings);
""";
}
