// ── AWARE bake-scene — a member's own labels (#818) ─────────────────────────
// A scene member (`member`, `line` or `box`) may carry two optional labels in its
// `meta`, beside its catalogue `meta.profile`:
//
//   meta.description   one line of text, e.g. "HOLD: size pending from others"
//   meta.properties    { name, properties } — one named set of string values
//
// `ifc write` (cli/src/render/ifc.rs) defined them first. This file is the C#
// mirror of its rules, shared by every CAD-host sink (Tekla, Revit, Rhino,
// SketchUp) so a scene is accepted or refused identically by all five. The
// messages are the Rust ones without the `ifc write: ` prefix, and
// `cli/src/render/scene_member_labels.cases.json` holds the cases both sides are
// tested against, so the two cannot drift apart unnoticed.
//
// It also holds the host-neutral half of writing them: given a host's own
// carrier (a Tekla UDA, a Revit parameter), write, read back, and report only
// what provably landed.
//
// CONSTRAINTS, because the Revit sink splices this text verbatim into the C#
// *script* it runs inside Revit (see cli-revit/Sidecar/BakeSceneScript.cs):
//   * no `namespace` and no `using` directives — every type is fully qualified;
//   * BCL only (CoreLib, System.Linq, System.Text.Json); no host types;
//   * it compiles on net48 (Tekla) as well as net8/net10.
#nullable disable

/// <summary>One member's validated labels. Null fields were not sent.</summary>
internal sealed class AwareMemberLabelSet
{
    /// <summary>`meta.description`, or null when the member sent none.</summary>
    internal string Description;

    /// <summary>`meta.properties.name`, or null when the member sent no property set.</summary>
    internal string PropertySetName;

    /// <summary>The set's properties in key order (ordinal), as the IFC writer writes them.</summary>
    internal System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>> Properties =
        new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>>();

    internal bool IsEmpty { get { return Description == null && PropertySetName == null; } }
}

/// <summary>What a sink provably wrote of one member's labels.</summary>
internal sealed class AwareMemberLabelOutcome
{
    /// <summary>True only when the description was written AND read back unchanged.</summary>
    internal bool DescriptionWritten;

    /// <summary>The set's name, only when EVERY property was written and read back unchanged.</summary>
    internal string PropertySetWritten;

    /// <summary>(field, reason) for every label that was sent but not written.</summary>
    internal System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>> NotWritten =
        new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>>();

    internal void NotWrittenBecause(string field, string reason)
    {
        NotWritten.Add(new System.Collections.Generic.KeyValuePair<string, string>(field, reason));
    }
}

/// <summary>
/// One host's place for a member's labels, as delegates over its native API, so the
/// write-read-back-report rule is the same text for every host and testable without one.
/// A member the host builds as several native objects (a Tekla double angle is two parts) is
/// one carrier: it writes to all of them, and reads a value back only when every one holds it.
/// </summary>
internal sealed class AwareMemberLabelCarrier
{
    /// <summary>The host key the description is written under, or null when the host has no place for it.</summary>
    internal string DescriptionKey;

    /// <summary>Why the description cannot be written, when <see cref="DescriptionKey"/> is null.</summary>
    internal string DescriptionUnavailable;

    /// <summary>Why the property set cannot be written at all, or null when the host has a place for it.</summary>
    internal string PropertiesUnavailable;

    /// <summary>(set name, property key) -> the host key that property is written under.</summary>
    internal System.Func<string, string, string> PropertyKey;

    /// <summary>(set name, host key) -> why that key may not be written (it is one this sink owns), or null.</summary>
    internal System.Func<string, string, string> Reserved;

    /// <summary>(host key, value) -> whether the host accepted the write.</summary>
    internal System.Func<string, string, bool> Write;

    /// <summary>host key -> the value the host now holds (from a fresh read), or null.</summary>
    internal System.Func<string, string> ReadBack;

    /// <summary>host key -> remove a value that did not land intact, so the model holds no part of it.</summary>
    internal System.Action<string> Clear;
}

/// <summary>The rules and the receipt vocabulary for a member's own labels.</summary>
internal static class AwareMemberLabels
{
    /// <summary>`meta.description` is at most this many UTF-8 bytes (Part 21 `STRING(256)`).</summary>
    internal const int DescriptionMaxBytes = 256;

    /// <summary>Each property value is at most this many UTF-8 bytes (IFC4 `IfcLabel` is `STRING(255)`).</summary>
    internal const int ValueMaxBytes = 255;

    /// <summary>A property set holds 1..this many properties.</summary>
    internal const int PropertiesMax = 16;

    /// <summary>Receipt `code` of a member refused for a malformed label.</summary>
    internal const string RefusalCode = "invalid-member-label";

    /// <summary>Receipt `code` of the warning raised for a label that was sent but not written.</summary>
    internal const string NotWrittenCode = "member-label-not-written";

    internal const string DescriptionField = "meta.description";
    internal const string PropertiesField = "meta.properties";

    const string NamePattern = "^[A-Za-z][A-Za-z0-9_]{0,63}$";

    /// <summary>`^[A-Za-z][A-Za-z0-9_]{0,63}$` — a property set or property name (Rust `is_ifc_name`).</summary>
    internal static bool IsIfcName(string name)
    {
        if (string.IsNullOrEmpty(name) || name.Length > 64) return false;
        if (!IsAsciiLetter(name[0])) return false;
        for (var i = 1; i < name.Length; i++)
        {
            var c = name[i];
            if (!(IsAsciiLetter(c) || (c >= '0' && c <= '9') || c == '_')) return false;
        }
        return true;
    }

    static bool IsAsciiLetter(char c)
    {
        return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
    }

    /// <summary>1..maxBytes UTF-8 bytes with no C0/DEL controls (Rust `is_meta_text`).</summary>
    internal static bool IsMetaText(string text, int maxBytes)
    {
        if (string.IsNullOrEmpty(text)) return false;
        if (System.Text.Encoding.UTF8.GetByteCount(text) > maxBytes) return false;
        foreach (var c in text)
        {
            if (c <= '\u001f' || c == '\u007f') return false;
        }
        return true;
    }

    /// <summary>
    /// Validate one member's `meta` and return its labels. Returns null when well formed (a meta
    /// that is absent or not an object carries no labels, as in Rust), else the refusal message,
    /// which names the member and the rule.
    /// </summary>
    /// <param name="meta">The member's `meta` in neutral form: an object is an
    /// <c>IEnumerable&lt;KeyValuePair&lt;string, object&gt;&gt;</c> in document order, a string is a
    /// <c>string</c>, JSON null is null and anything else is any other object. Tekla's script
    /// arguments already have this shape; <see cref="FromJsonElement"/> and <see cref="FromJsonNode"/>
    /// convert the other sinks' JSON.</param>
    internal static string Validate(string id, object meta, out AwareMemberLabelSet labels)
    {
        labels = new AwareMemberLabelSet();
        var fields = AsObject(meta);
        if (fields == null) return null;
        var subject = "member `" + id + "`";

        object description;
        if (TryGet(fields, "description", out description))
        {
            var text = description as string;
            if (text == null || !IsMetaText(text, DescriptionMaxBytes))
                return subject + " `meta.description` must be a string of 1.." + DescriptionMaxBytes
                       + " UTF-8 bytes with no C0/DEL controls";
            labels.Description = text;
        }

        object block;
        if (!TryGet(fields, "properties", out block)) return null;
        const string field = "properties";
        var set = AsObject(block);
        if (set == null)
            return subject + " `meta." + field + "` must be an object `{ name, properties }`";
        foreach (var pair in set)
        {
            if (pair.Key != "name" && pair.Key != "properties")
                return subject + " `meta." + field + "` has an unknown field `" + pair.Key + "`";
        }
        object nameValue;
        var name = TryGet(set, "name", out nameValue) ? nameValue as string ?? "" : "";
        if (!IsIfcName(name))
            return subject + " `meta." + field + ".name` must match " + NamePattern;
        if (name.StartsWith("pset_", System.StringComparison.OrdinalIgnoreCase)
            || name.StartsWith("qto_", System.StringComparison.OrdinalIgnoreCase))
            return subject + " `meta." + field + ".name` `" + name
                   + "` may not start with Pset_ or Qto_, which buildingSMART reserves for its standard property sets";
        object propertiesValue;
        var properties = TryGet(set, "properties", out propertiesValue) ? AsObject(propertiesValue) : null;
        if (properties == null)
            return subject + " `meta." + field + ".properties` must be an object of strings";
        if (properties.Count == 0 || properties.Count > PropertiesMax)
            return subject + " `meta." + field + ".properties` must hold 1.." + PropertiesMax + " properties";
        foreach (var pair in properties)
        {
            if (!IsIfcName(pair.Key))
                return subject + " `meta." + field + ".properties` key `" + pair.Key + "` must match " + NamePattern;
            var value = pair.Value as string;
            if (value == null || !IsMetaText(value, ValueMaxBytes))
                return subject + " `meta." + field + ".properties." + pair.Key + "` must be a string of 1.."
                       + ValueMaxBytes + " UTF-8 bytes with no C0/DEL controls";
        }

        labels.PropertySetName = name;
        foreach (var pair in properties)
            labels.Properties.Add(new System.Collections.Generic.KeyValuePair<string, string>(pair.Key, (string)pair.Value));
        labels.Properties.Sort((a, b) => string.CompareOrdinal(a.Key, b.Key));
        return null;
    }

    /// <summary>An object in document order with one entry per key: a repeated key keeps its first
    /// position and its last value, which is what serde_json's `preserve_order` map does in Rust.</summary>
    static System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, object>> AsObject(object value)
    {
        if (value == null || value is string) return null;
        var pairs = value as System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<string, object>>;
        if (pairs == null) return null;
        var ordered = new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, object>>();
        var index = new System.Collections.Generic.Dictionary<string, int>(System.StringComparer.Ordinal);
        foreach (var pair in pairs)
        {
            int at;
            if (index.TryGetValue(pair.Key, out at))
            {
                ordered[at] = new System.Collections.Generic.KeyValuePair<string, object>(pair.Key, pair.Value);
                continue;
            }
            index[pair.Key] = ordered.Count;
            ordered.Add(pair);
        }
        return ordered;
    }

    static bool TryGet(System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, object>> fields,
        string key, out object value)
    {
        foreach (var pair in fields)
        {
            if (pair.Key == key)
            {
                value = pair.Value;
                return true;
            }
        }
        value = null;
        return false;
    }

    /// <summary>A JSON element in the neutral form <see cref="Validate"/> reads.</summary>
    internal static object FromJsonElement(System.Text.Json.JsonElement element)
    {
        switch (element.ValueKind)
        {
            case System.Text.Json.JsonValueKind.Object:
                var pairs = new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, object>>();
                foreach (var property in element.EnumerateObject())
                    pairs.Add(new System.Collections.Generic.KeyValuePair<string, object>(property.Name, FromJsonElement(property.Value)));
                return pairs;
            case System.Text.Json.JsonValueKind.String:
                return element.GetString();
            case System.Text.Json.JsonValueKind.Null:
            case System.Text.Json.JsonValueKind.Undefined:
                return null;
            default:
                return element.ValueKind;
        }
    }

    /// <summary>A JSON node in the neutral form <see cref="Validate"/> reads.</summary>
    internal static object FromJsonNode(System.Text.Json.Nodes.JsonNode node)
    {
        if (node == null) return null;
        var obj = node as System.Text.Json.Nodes.JsonObject;
        if (obj != null)
        {
            var pairs = new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, object>>();
            foreach (var property in obj)
                pairs.Add(new System.Collections.Generic.KeyValuePair<string, object>(property.Key, FromJsonNode(property.Value)));
            return pairs;
        }
        var value = node as System.Text.Json.Nodes.JsonValue;
        string text;
        if (value != null && value.GetValueKind() == System.Text.Json.JsonValueKind.String && value.TryGetValue<string>(out text))
            return text;
        return node.GetValueKind();
    }

    /// <summary>
    /// Write a member's labels through one host carrier, read each back, and keep only what landed
    /// intact. A property set is all or nothing: if any of its values does not land, the ones that
    /// did are cleared, so the model never holds part of a set the receipt says was not written.
    /// </summary>
    internal static AwareMemberLabelOutcome Write(AwareMemberLabelSet labels, AwareMemberLabelCarrier carrier)
    {
        var outcome = new AwareMemberLabelOutcome();
        if (labels == null || labels.IsEmpty) return outcome;

        if (labels.Description != null)
        {
            if (carrier.DescriptionKey == null)
                outcome.NotWrittenBecause(DescriptionField, carrier.DescriptionUnavailable ?? "this host has no place for a member description");
            else
            {
                var reason = WriteOne(carrier, carrier.DescriptionKey, labels.Description);
                if (reason == null) outcome.DescriptionWritten = true;
                else outcome.NotWrittenBecause(DescriptionField, reason);
            }
        }

        if (labels.PropertySetName != null)
        {
            if (carrier.PropertiesUnavailable != null)
            {
                outcome.NotWrittenBecause(PropertiesField, carrier.PropertiesUnavailable);
                return outcome;
            }
            var landed = new System.Collections.Generic.List<string>();
            string failure = null;
            foreach (var property in labels.Properties)
            {
                var hostKey = carrier.PropertyKey(labels.PropertySetName, property.Key);
                var reserved = carrier.Reserved == null ? null : carrier.Reserved(labels.PropertySetName, hostKey);
                if (reserved != null)
                {
                    failure = "property `" + property.Key + "` was not written: " + reserved;
                    break;
                }
                var reason = WriteOne(carrier, hostKey, property.Value);
                if (reason != null)
                {
                    failure = "property `" + property.Key + "` was not written: " + reason;
                    break;
                }
                landed.Add(hostKey);
            }
            if (failure == null) outcome.PropertySetWritten = labels.PropertySetName;
            else
            {
                var kept = 0;
                foreach (var hostKey in landed) if (!TryClear(carrier, hostKey)) kept++;
                outcome.NotWrittenBecause(PropertiesField, failure
                    + (landed.Count == 0 ? ""
                       : kept == 0 ? "; the " + landed.Count + " that were written were removed again, so none of the set is in the model"
                       : "; " + kept + " of the " + landed.Count + " that were written could not be removed again, so the model may hold part of the set"));
            }
        }
        return outcome;
    }

    /// <summary>Null when <paramref name="value"/> landed under <paramref name="hostKey"/> intact,
    /// else why not (after clearing whatever did land).</summary>
    static string WriteOne(AwareMemberLabelCarrier carrier, string hostKey, string value)
    {
        bool accepted;
        try { accepted = carrier.Write(hostKey, value); }
        catch (System.Exception ex)
        {
            ClearQuietly(carrier, hostKey);
            return "the host refused `" + hostKey + "`: " + ex.Message;
        }
        if (!accepted)
        {
            ClearQuietly(carrier, hostKey);
            return "the host refused to store `" + hostKey + "`";
        }
        string stored;
        try { stored = carrier.ReadBack(hostKey); }
        catch (System.Exception ex)
        {
            ClearQuietly(carrier, hostKey);
            return "`" + hostKey + "` could not be read back: " + ex.Message;
        }
        if (stored == value) return null;
        var removed = TryClear(carrier, hostKey)
            ? ", so it was removed again"
            : ", and it could not be removed again, so the model may hold part of it";
        if (stored == null) return "`" + hostKey + "` could not be read back" + removed;
        return value.StartsWith(stored, System.StringComparison.Ordinal)
            ? "`" + hostKey + "` read back as " + System.Text.Encoding.UTF8.GetByteCount(stored) + " of its "
              + System.Text.Encoding.UTF8.GetByteCount(value) + " bytes" + removed
            : "`" + hostKey + "` read back differently from what was sent" + removed;
    }

    static void ClearQuietly(AwareMemberLabelCarrier carrier, string hostKey) => TryClear(carrier, hostKey);

    /// <summary>Remove whatever landed under <paramref name="hostKey"/>, and say whether it is
    /// gone: true only when the carrier then reads back nothing (null or empty) there.</summary>
    static bool TryClear(AwareMemberLabelCarrier carrier, string hostKey)
    {
        if (carrier.Clear == null) return false;
        try
        {
            carrier.Clear(hostKey);
            return string.IsNullOrEmpty(carrier.ReadBack(hostKey));
        }
        catch (System.Exception)
        {
            return false;
        }
    }

    /// <summary>
    /// Put the outcome on the member's receipt row, in #819's vocabulary: `description: true` and
    /// `propertySet: "<name>"` only for what was written, and `labelsNotWritten: [{field, reason}]`
    /// for anything sent that was not. A member that sent no labels gains no field.
    /// </summary>
    internal static void Report(System.Collections.Generic.IDictionary<string, object> row, AwareMemberLabelOutcome outcome)
    {
        if (outcome == null) return;
        if (outcome.DescriptionWritten) row["description"] = true;
        if (outcome.PropertySetWritten != null) row["propertySet"] = outcome.PropertySetWritten;
        if (outcome.NotWritten.Count == 0) return;
        var misses = new System.Collections.Generic.List<object>();
        foreach (var miss in outcome.NotWritten)
        {
            var entry = new System.Collections.Generic.Dictionary<string, object>(System.StringComparer.Ordinal);
            entry["field"] = miss.Key;
            entry["reason"] = miss.Value;
            misses.Add(entry);
        }
        row["labelsNotWritten"] = misses;
    }

    /// <summary>The `warnings` row raised for one label that was sent but not written.</summary>
    internal static System.Collections.Generic.Dictionary<string, object> NotWrittenWarning(string id, string kind, string field, string reason)
    {
        var row = new System.Collections.Generic.Dictionary<string, object>(System.StringComparer.Ordinal);
        row["id"] = id;
        row["kind"] = kind;
        row["status"] = "warning";
        row["code"] = NotWrittenCode;
        row["message"] = NotWrittenMessage(field, reason);
        return row;
    }

    /// <summary>The warning text for one label that was sent but not written.</summary>
    internal static string NotWrittenMessage(string field, string reason)
    {
        return "`" + field + "` was sent but not written: " + reason;
    }
}
