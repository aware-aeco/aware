using System;
using System.Collections.Generic;

namespace AwareTekla;

/// <summary>A member's validated labels (#818), opaque to the bake script.</summary>
public sealed class TeklaMemberLabels
{
    internal TeklaMemberLabels(AwareMemberLabelSet set) => Set = set;

    internal AwareMemberLabelSet Set { get; }

    /// <summary>True when the member sent neither a description nor a property set.</summary>
    public bool IsEmpty => Set.IsEmpty;
}

/// <summary>What the bake provably wrote of one member's labels.</summary>
public sealed class TeklaMemberLabelReport
{
    internal TeklaMemberLabelReport(AwareMemberLabelOutcome outcome) => Outcome = outcome;

    internal AwareMemberLabelOutcome Outcome { get; }

    /// <summary>Adds `description: true`, `propertySet` and `labelsNotWritten` to the member's receipt row.</summary>
    public void ApplyTo(IDictionary<string, object> row) => AwareMemberLabels.Report(row, Outcome);

    /// <summary>One `member-label-not-written` warning row per label that was sent but not written.</summary>
    public List<Dictionary<string, object>> Warnings(string id, string kind)
    {
        var rows = new List<Dictionary<string, object>>();
        foreach (var miss in Outcome.NotWritten)
            rows.Add(AwareMemberLabels.NotWrittenWarning(id, kind, miss.Key, miss.Value));
        return rows;
    }
}

/// <summary>
/// Tekla's carrier for a scene member's own labels (#818), the public face the Roslyn
/// materializer calls. The rules are the shared <c>cli-shared/AwareMemberLabels.cs</c> ones
/// (the IFC writer's, mirrored); what is Tekla's is where each label goes:
///
///   meta.description  -> the part's Comment, which is Tekla's standard `comment` UDA;
///   meta.properties   -> one UDA per property, named by the property's own key.
///
/// UDAs are how this sink already stamps its own facts onto a part (USER_FIELD_1..4 for
/// ownership, AWARE_REQUESTED_PROFILE for a profile-less line), through the same
/// SetUserProperty/GetUserProperty pair, so properties take that established route. Tekla's
/// UDA namespace is flat, so the set's NAME has no place in the model: the keys go in as they
/// are, which is also what lets a firm's own predefined UDA (say `PRELIM_MARK`) receive a value.
/// A key that is one of the UDAs this sink owns is never written over.
/// </summary>
public static class TeklaMemberLabelContract
{
    /// <summary>Tekla's standard part attribute behind the Comment field.</summary>
    public const string CommentUda = "comment";

    public const string RefusalCode = AwareMemberLabels.RefusalCode;

    static readonly Dictionary<string, string> OwnedUdas = new(StringComparer.OrdinalIgnoreCase)
    {
        ["USER_FIELD_1"] = "it is the UDA bake-scene stamps the part's source ownership into",
        ["USER_FIELD_2"] = "it is the UDA bake-scene stamps the part's record id into",
        ["USER_FIELD_3"] = "it is the UDA bake-scene stamps the part's scene hash into",
        ["USER_FIELD_4"] = "it is the UDA bake-scene stamps the part's ownership marker into",
        [CommentUda] = "it is the part's Comment, which carries `meta.description`",
        ["AWARE_REQUESTED_PROFILE"] = "it is the UDA bake-scene records a profile-less line's requested profile in",
    };

    /// <summary>Null when the member's `meta` labels are well formed, else the refusal message.</summary>
    /// <param name="meta">The member's `meta` as the script receives it (an
    /// <c>IDictionary&lt;string, object&gt;</c>), or null.</param>
    public static string? Validate(string id, object? meta, out TeklaMemberLabels labels)
    {
        var error = AwareMemberLabels.Validate(id, meta, out var set);
        labels = new TeklaMemberLabels(set);
        return error;
    }

    /// <summary>
    /// Write the labels onto the member's native part(s) and report what landed.
    /// </summary>
    /// <param name="setUda">(name, value) -> whether every native part of the member accepted it.</param>
    /// <param name="readUda">name -> the value every native part now holds (read fresh), or null when
    /// any part lacks it or the parts disagree.</param>
    public static TeklaMemberLabelReport Write(
        TeklaMemberLabels labels, Func<string, string, bool> setUda, Func<string, string?> readUda)
    {
        var carrier = new AwareMemberLabelCarrier
        {
            DescriptionKey = CommentUda,
            PropertyKey = (set, key) => key,
            Reserved = (set, uda) => OwnedUdas.TryGetValue(uda, out var why) ? why : null,
            Write = setUda,
            ReadBack = name => readUda(name),
            // Tekla has no "remove UDA" call; an empty string is how a UDA is cleared.
            Clear = name => setUda(name, ""),
        };
        return new TeklaMemberLabelReport(AwareMemberLabels.Write(labels.Set, carrier));
    }
}
