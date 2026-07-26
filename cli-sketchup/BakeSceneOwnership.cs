// Ownership vocabulary for bake-scene: which AttributeDictionary the sidecar
// stamps onto every group it materializes, and the rule that decides which
// already-present groups a new bake retires.
//
// This is SketchUp's equivalent of the Tekla sink's ownership UDAs
// (USER_FIELD_1..4). It lives in C# — not in the Ruby — so the rule can be unit
// tested without a live SketchUp, and so there is exactly one definition of it:
// BakeSceneScript.rb reads every key name and value out of `args['ownership']`
// and hardcodes none of them, so the two cannot drift apart.

using System.Text.RegularExpressions;

namespace AwareSketchup;

/// <summary>
/// What a bake writes into each group it owns, and what it will erase.
/// </summary>
internal static class BakeSceneOwnership
{
    /// <summary>AttributeDictionary the ownership keys live in. Matches the `AWARE` dictionary the curated verbs already use.</summary>
    public const string DictionaryName = "AWARE";

    /// <summary>Tag (layer) every baked group is placed on.</summary>
    public const string TagName = "AWARE";

    public const string SourceIdKey  = "bake_source_id";
    public const string RecordIdKey  = "bake_record_id";
    public const string SceneHashKey = "bake_scene_hash";
    public const string MarkerKey    = "bake_marker";

    /// <summary>Namespaced prefix so a marker cannot be confused with any other extension's attribute.</summary>
    public const string MarkerPrefix = "AWARE_BAKE_V1:";

    /// <summary>Ruby-compatible anchored pattern for a well-formed marker (prefix + 64 lowercase hex).</summary>
    public const string MarkerPattern = @"\AAWARE_BAKE_V1:[0-9a-f]{64}\z";

    static readonly Regex MarkerRegex = new("^AWARE_BAKE_V1:[0-9a-f]{64}$", RegexOptions.CultureInvariant);

    public static string BuildMarker(string materializationHash) => MarkerPrefix + materializationHash;

    public static bool IsOwnershipMarker(string? marker) => marker is not null && MarkerRegex.IsMatch(marker);

    /// <summary>
    /// One already-present entity as the ownership scan reads it. Absent
    /// attributes arrive as null / empty, exactly as SketchUp's
    /// <c>get_attribute</c> returns them.
    /// </summary>
    internal readonly record struct OwnedCandidate(
        string? SourceId,
        string? RecordId,
        string? SceneHash,
        string? Marker);

    internal enum Retirement
    {
        /// <summary>Not stamped by any bake — user geometry. Never touched.</summary>
        KeepUnowned,

        /// <summary>Stamped, but by a different source id. Never touched: this is the property that keeps a bake from erasing someone else's model.</summary>
        KeepForeign,

        /// <summary>Owned by this source. Retired, because every record the incoming scene still carries is re-materialized fresh in the same operation.</summary>
        Retire,
    }

    /// <summary>
    /// The retire-and-replace decision, mirrored verbatim by
    /// BakeSceneScript.rb's <c>owned</c> lambda.
    ///
    /// bake-scene is a REPLACEMENT, not an append: every record the incoming
    /// scene carries is created fresh, so the whole prior set owned by this
    /// source id is retired. The observable effect is that a record the new
    /// scene no longer carries disappears, and one it still carries is replaced.
    /// A candidate is only ever retired when it carries all four attributes, its
    /// source id equals this bake's, and its marker is well formed.
    /// </summary>
    /// <param name="incomingSourceId">
    /// <c>scene.meta.sourceId</c> for the bake now running. An empty value would
    /// match every never-stamped entity, so it always decides KeepUnowned.
    /// </param>
    internal static Retirement Decide(OwnedCandidate candidate, string? incomingSourceId)
    {
        if (string.IsNullOrWhiteSpace(incomingSourceId)) return Retirement.KeepUnowned;

        bool stamped = !string.IsNullOrWhiteSpace(candidate.SourceId)
                       && !string.IsNullOrWhiteSpace(candidate.RecordId)
                       && !string.IsNullOrWhiteSpace(candidate.SceneHash)
                       && IsOwnershipMarker(candidate.Marker);
        if (!stamped) return Retirement.KeepUnowned;

        return string.Equals(candidate.SourceId, incomingSourceId, StringComparison.Ordinal)
            ? Retirement.Retire
            : Retirement.KeepForeign;
    }
}
