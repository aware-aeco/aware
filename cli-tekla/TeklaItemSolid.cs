using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Security.Cryptography;
using System.Text;

namespace AwareTekla;

/// <summary>
/// `item-create` (aware#859, FloLess #3382 A5): the pure geometry half. A member's neutral solid is a section
/// (an outer ring plus holes, in millimetres, in the Y-Z plane) swept along +X from 0 to <see cref="Length"/>;
/// that is the only solid kind this verb builds. Everything here is Tekla-free so it is unit-tested directly.
///
/// Facts this rests on (live Tekla 2026.0, spike run 1, 2026-11-19): the shape is a <c>FacetedBrep</c> whose caps
/// carry the holes as inner loops wound OPPOSITE to the cap's outer loop (the "same" winding is invalid and
/// <c>Insert()</c> still accepts it, so the brep is validated before it is offered); Tekla never stretches a
/// shape to an Item's length, so the length is part of the shape and of its name.
/// </summary>
public sealed class PrismSolid
{
    /// <summary>Largest coordinate or length, in millimetres, this verb accepts.</summary>
    public const double MaxExtentMm = 1e7;
    /// <summary>Smallest length, in millimetres.</summary>
    public const double MinLengthMm = 1e-3;
    /// <summary>Most points in one ring.</summary>
    public const int MaxRingPoints = 5000;
    /// <summary>Most holes in one section (the overlap checks are quadratic in them).</summary>
    public const int MaxHoles = 256;

    /// <summary>Outer ring as [y, z] points, counter-clockwise seen from +X after <see cref="Normalize"/>.</summary>
    public List<double[]> Outer { get; } = new List<double[]>();
    /// <summary>Hole rings as [y, z] points, clockwise seen from +X after <see cref="Normalize"/>.</summary>
    public List<List<double[]>> Holes { get; } = new List<List<double[]>>();
    public double Length { get; set; }

    /// <summary>The section's area in mm2: outer minus holes.</summary>
    public double Area => Math.Abs(Shoelace(Outer)) - Holes.Sum(h => Math.Abs(Shoelace(h)));

    /// <summary>The volume in mm3 Tekla must report for the placed Item.</summary>
    public double Volume => Area * Length;

    /// <summary>The solid's box in its own frame: [minX, minY, minZ, maxX, maxY, maxZ].</summary>
    public double[] Box => new[] { 0.0, Outer.Min(p => p[0]), Outer.Min(p => p[1]), Length, Outer.Max(p => p[0]), Outer.Max(p => p[1]) };

    public static double Shoelace(IReadOnlyList<double[]> ring)
    {
        double a = 0;
        for (int i = 0; i < ring.Count; i++)
        {
            var p = ring[i];
            var q = ring[(i + 1) % ring.Count];
            a += p[0] * q[1] - q[0] * p[1];
        }
        return a / 2.0;
    }

    /// <summary>Outer ring counter-clockwise, hole rings clockwise (the brep builder and the digest rely on it).</summary>
    public void Normalize()
    {
        if (Shoelace(Outer) < 0) Outer.Reverse();
        foreach (var h in Holes) if (Shoelace(h) > 0) h.Reverse();
    }

    /// <summary>
    /// The solid's identity: SHA-256 over its normalised ring orientation, every coordinate and the length,
    /// written with invariant shortest round-trip digits. Two solids of the same section and length give the same
    /// digest; a different length gives a different one, which is what makes "one shape per length" follow.
    /// </summary>
    public string Digest()
    {
        var sb = new StringBuilder("aware-prism-v1;L=").Append(Num(Length)).Append(";outer=");
        AppendRing(sb, Outer);
        foreach (var h in Holes) { sb.Append(";hole="); AppendRing(sb, h); }
        using var sha = SHA256.Create();
        var hash = sha.ComputeHash(Encoding.UTF8.GetBytes(sb.ToString()));
        return "sha256:" + string.Concat(hash.Select(b => b.ToString("x2", CultureInfo.InvariantCulture)));
    }

    static void AppendRing(StringBuilder sb, List<double[]> ring)
    {
        for (int i = 0; i < ring.Count; i++)
        {
            if (i > 0) sb.Append(' ');
            sb.Append(Num(ring[i][0])).Append(',').Append(Num(ring[i][1]));
        }
    }

    internal static string Num(double v)
    {
        if (v == 0) v = 0; // -0 -> 0
        return v.ToString("R", CultureInfo.InvariantCulture);
    }

    /// <summary>
    /// Why this solid cannot be built, or null: every number finite and bounded, rings of 3 or more distinct
    /// points, no ring crossing itself, every hole strictly inside the outer ring and clear of the others.
    /// Names the member through <paramref name="who"/>.
    /// </summary>
    public string? Validate(string who)
    {
        if (!(Length >= MinLengthMm) || Length > MaxExtentMm || double.IsNaN(Length) || double.IsInfinity(Length))
            return $"{who}: the length must be a number from {MinLengthMm} to {MaxExtentMm} mm.";
        if (Holes.Count > MaxHoles) return $"{who}: the section has {Holes.Count} holes; the limit is {MaxHoles}.";
        var rings = new List<(string name, List<double[]> pts)> { ("outer", Outer) };
        for (int h = 0; h < Holes.Count; h++) rings.Add(($"hole {h}", Holes[h]));
        foreach (var (name, pts) in rings)
        {
            if (pts.Count < 3) return $"{who}: the {name} ring needs 3 or more points.";
            if (pts.Count > MaxRingPoints) return $"{who}: the {name} ring has {pts.Count} points; the limit is {MaxRingPoints}.";
            foreach (var p in pts)
                if (p.Length != 2 || double.IsNaN(p[0]) || double.IsNaN(p[1]) || double.IsInfinity(p[0]) || double.IsInfinity(p[1]) || Math.Abs(p[0]) > MaxExtentMm || Math.Abs(p[1]) > MaxExtentMm)
                    return $"{who}: the {name} ring has a point that is not a finite [y, z] pair within {MaxExtentMm} mm.";
            for (int i = 0; i < pts.Count; i++)
            {
                var a = pts[i]; var b = pts[(i + 1) % pts.Count];
                if (Math.Abs(a[0] - b[0]) < 1e-9 && Math.Abs(a[1] - b[1]) < 1e-9)
                    return $"{who}: the {name} ring repeats a point (points {i} and {(i + 1) % pts.Count}).";
            }
            if (Math.Abs(Shoelace(pts)) < 1e-9) return $"{who}: the {name} ring encloses no area.";
            if (SelfIntersects(pts)) return $"{who}: the {name} ring crosses itself.";
        }
        foreach (var (name, pts) in rings.Skip(1))
        {
            if (pts.Any(p => !InsideStrictly(Outer, p))) return $"{who}: the {name} is not inside the outer ring.";
            if (Cross(Outer, pts)) return $"{who}: the {name} touches or crosses the outer ring.";
        }
        for (int a = 0; a < Holes.Count; a++)
            for (int b = a + 1; b < Holes.Count; b++)
            {
                if (Cross(Holes[a], Holes[b]) || Holes[a].Any(p => InsideStrictly(Holes[b], p)) || Holes[b].Any(p => InsideStrictly(Holes[a], p)))
                    return $"{who}: hole {a} and hole {b} overlap.";
            }
        return null;
    }

    static double Orient(double[] a, double[] b, double[] c) => (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);

    static bool OnSegment(double[] a, double[] b, double[] p) =>
        Math.Min(a[0], b[0]) <= p[0] && p[0] <= Math.Max(a[0], b[0]) && Math.Min(a[1], b[1]) <= p[1] && p[1] <= Math.Max(a[1], b[1]);

    static bool SegmentsMeet(double[] p1, double[] p2, double[] p3, double[] p4)
    {
        double d1 = Orient(p3, p4, p1), d2 = Orient(p3, p4, p2), d3 = Orient(p1, p2, p3), d4 = Orient(p1, p2, p4);
        if (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))) return true;
        if (d1 == 0 && OnSegment(p3, p4, p1)) return true;
        if (d2 == 0 && OnSegment(p3, p4, p2)) return true;
        if (d3 == 0 && OnSegment(p1, p2, p3)) return true;
        if (d4 == 0 && OnSegment(p1, p2, p4)) return true;
        return false;
    }

    static bool SelfIntersects(List<double[]> ring)
    {
        int n = ring.Count;
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++)
            {
                bool adjacent = j == i + 1 || (i == 0 && j == n - 1);
                if (adjacent) continue;
                if (SegmentsMeet(ring[i], ring[(i + 1) % n], ring[j], ring[(j + 1) % n])) return true;
            }
        return false;
    }

    static bool Cross(List<double[]> a, List<double[]> b)
    {
        for (int i = 0; i < a.Count; i++)
            for (int j = 0; j < b.Count; j++)
                if (SegmentsMeet(a[i], a[(i + 1) % a.Count], b[j], b[(j + 1) % b.Count])) return true;
        return false;
    }

    /// <summary>Point strictly inside a ring (ray casting); a point on the boundary is not inside.</summary>
    static bool InsideStrictly(List<double[]> ring, double[] p)
    {
        for (int i = 0; i < ring.Count; i++)
        {
            var a = ring[i]; var b = ring[(i + 1) % ring.Count];
            if (Orient(a, b, p) == 0 && OnSegment(a, b, p)) return false;
        }
        bool inside = false;
        for (int i = 0, j = ring.Count - 1; i < ring.Count; j = i++)
        {
            var a = ring[i]; var b = ring[j];
            if ((a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]) inside = !inside;
        }
        return inside;
    }

    /// <summary>The faceted brep Tekla is offered: vertices, face loops and the caps' inner loops.</summary>
    public PrismBrep BuildBrep()
    {
        var rings = new List<List<double[]>> { Outer };
        rings.AddRange(Holes);
        int total = rings.Sum(r => r.Count);
        var brep = new PrismBrep();
        for (int level = 0; level < 2; level++)
            foreach (var r in rings)
                foreach (var p in r) brep.Vertices.Add(new[] { level == 0 ? 0.0 : Length, p[0], p[1] });
        int Vid(int level, int k) => level * total + k;
        int[] RingIds(int level, int ring) => Enumerable.Range(0, rings[ring].Count).Select(i => Vid(level, rings.Take(ring).Sum(r => r.Count) + i)).ToArray();

        // The cap at x = 0 faces -X, so its outer loop runs clockwise seen from +X; the cap at x = L faces +X and
        // runs counter-clockwise. Hole rings are stored clockwise (seen from +X); a cap's inner loops wind OPPOSITE
        // to its outer loop (the winding that is valid in Tekla; the "same" winding is reported invalid).
        brep.Faces.Add(RingIds(0, 0).Reverse().ToArray());
        if (rings.Count > 1)
            brep.InnerLoops[0] = Enumerable.Range(1, rings.Count - 1).Select(h => RingIds(0, h).Reverse().ToArray()).ToArray();
        int capEnd = brep.Faces.Count;
        brep.Faces.Add(RingIds(1, 0));
        if (rings.Count > 1)
            brep.InnerLoops[capEnd] = Enumerable.Range(1, rings.Count - 1).Select(h => RingIds(1, h)).ToArray();
        // Side faces: one quad per ring edge, facing out of the material.
        for (int r = 0; r < rings.Count; r++)
        {
            int start = rings.Take(r).Sum(x => x.Count);
            int n = rings[r].Count;
            for (int i = 0; i < n; i++)
                brep.Faces.Add(new[] { Vid(0, start + i), Vid(0, start + (i + 1) % n), Vid(1, start + (i + 1) % n), Vid(1, start + i) });
        }
        return brep;
    }
}

/// <summary>A faceted brep without a Tekla type, so the engine and its tests never need Tekla's assemblies.</summary>
public sealed class PrismBrep
{
    /// <summary>[x, y, z] per vertex, in millimetres.</summary>
    public List<double[]> Vertices { get; } = new List<double[]>();
    /// <summary>Vertex indices of each face's outer loop.</summary>
    public List<int[]> Faces { get; } = new List<int[]>();
    /// <summary>Per face index, its inner loops (holes).</summary>
    public Dictionary<int, int[][]> InnerLoops { get; } = new Dictionary<int, int[][]>();
}

/// <summary>
/// What a shape in the model's shape catalogue may be called, and the name this verb derives. The live rule is
/// not fully known (spike experiment E6 has not run); the verb therefore uses a conservative allow-list, so it
/// can only ever write names that Tekla's own file names and its documented 1023-character limit allow.
/// </summary>
public static class ItemShapeNames
{
    /// <summary>Longest shape name this verb writes. Tekla documents 1023; this is deliberately far shorter.</summary>
    public const int MaxLength = 64;
    /// <summary>Longest prefix a caller may ask for.</summary>
    public const int MaxPrefixLength = 16;
    /// <summary>The prefix of every shape this verb makes.</summary>
    public const string DefaultPrefix = "AWI";
    /// <summary>How many hex digits of the solid's digest go into the name (96 bits).</summary>
    public const int DigestHexLength = 24;

    /// <summary>Why a shape name is not allowed, or null. Allowed: ASCII letters, digits, '-', '_' and '.', starting with a letter.</summary>
    public static string? Validate(string? name)
    {
        if (string.IsNullOrEmpty(name)) return "a shape name cannot be empty.";
        if (name!.Length > MaxLength) return $"'{name}' is {name.Length} characters; this verb writes names of at most {MaxLength}.";
        if (!IsAsciiLetter(name[0])) return $"'{name}' must start with an ASCII letter.";
        foreach (var c in name)
            if (!(IsAsciiLetter(c) || (c >= '0' && c <= '9') || c == '-' || c == '_' || c == '.'))
                return $"'{name}' holds '{c}'; this verb writes only ASCII letters, digits, '-', '_' and '.'.";
        if (name.EndsWith(".", StringComparison.Ordinal)) return $"'{name}' cannot end with a dot.";
        return null;
    }

    /// <summary>Why a prefix is not allowed, or null.</summary>
    public static string? ValidatePrefix(string? prefix)
    {
        if (string.IsNullOrEmpty(prefix)) return "name_prefix cannot be empty.";
        if (prefix!.Length > MaxPrefixLength) return $"name_prefix '{prefix}' is longer than {MaxPrefixLength} characters.";
        return Validate(prefix + "-" + new string('0', DigestHexLength));
    }

    /// <summary>
    /// True when the name has the form this verb derives (a prefix, a dash and 24 hex digits) and, when a digest is
    /// given, those digits are that digest's first ones. The delete path uses it so it only removes shapes it can show
    /// it makes: a hand-made shape or one of Tekla's own never has this form.
    /// </summary>
    public static bool IsDerived(string? name, string? digest = null)
    {
        if (name == null || Validate(name) != null) return false;
        int dash = name.Length - DigestHexLength - 1;
        if (dash < 1 || dash > MaxPrefixLength || name[dash] != '-') return false;
        var hex = name.Substring(dash + 1);
        if (!hex.All(c => (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))) return false;
        if (digest == null) return true;
        var want = digest.StartsWith("sha256:", StringComparison.Ordinal) ? digest.Substring(7) : digest;
        return want.StartsWith(hex, StringComparison.Ordinal);
    }

    /// <summary>The shape name for a solid digest ("sha256:..."): prefix, a dash and the first hex digits.</summary>
    public static string For(string prefix, string digest)
    {
        var hex = digest.StartsWith("sha256:", StringComparison.Ordinal) ? digest.Substring(7) : digest;
        return prefix + "-" + hex.Substring(0, Math.Min(DigestHexLength, hex.Length));
    }

    static bool IsAsciiLetter(char c) => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}
