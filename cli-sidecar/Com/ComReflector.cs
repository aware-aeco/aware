using System.Diagnostics.CodeAnalysis;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.ComTypes;
using System.Runtime.Versioning;
using Microsoft.Win32;
using AwareSidecar.Protocol;
using AwareReader;

namespace AwareSidecar.Com;

[SupportedOSPlatform("windows")]
public static class ComReflector
{
    public static GeneratedAgent Reflect(string progId, string? agentIdOverride)
    {
        // Step 1: ProgID → CLSID
        int hr = CLSIDFromProgID(progId, out var clsid);
        if (hr < 0)
            throw new InvalidOperationException($"unknown ProgID '{progId}' (hr=0x{hr:X8})");

        // Step 2: Try to locate the TypeLib via registry
        var (typeLibPath, typeLibVer) = LookupTypeLib(clsid);

        var commands = new List<GeneratedCommand>();
        var skills = new List<GeneratedSkill>();
        string descriptionForAgent = $"COM object {progId} (CLSID {clsid:B})";

        if (typeLibPath != null)
        {
            try
            {
                int hr2 = LoadTypeLibEx(typeLibPath, RegKind.None, out var typeLib);
                if (hr2 >= 0 && typeLib != null)
                {
                    try
                    {
                        EnumerateTypeLib(typeLib, progId, commands, skills);
                    }
                    finally
                    {
                        Marshal.ReleaseComObject(typeLib);
                    }
                }
            }
            catch (Exception ex)
            {
                // Fall through to minimal viable; record the issue in the agent description
                descriptionForAgent += $" (TypeLib enumeration failed: {ex.Message})";
            }
        }

        if (commands.Count == 0)
        {
            // Minimum viable: a placeholder
            commands.Add(new GeneratedCommand
            {
                Name = "invoke",
                Lifecycle = "single",
                Description = $"Invoke {progId}. See TypeLib at {typeLibPath ?? "(not found)"} for full method list.",
            });
        }

        if (skills.Count == 0)
        {
            skills.Add(new GeneratedSkill
            {
                Filename = "com-reference.md",
                Body = $"---\nname: {ToKebab(progId)}-com-reference\ndescription: COM type-library reference for {progId}\n---\n\n# {progId}\n\n- CLSID: `{clsid:B}`\n- TypeLib path: `{typeLibPath ?? "(not located)"}`\n- TypeLib version: `{typeLibVer ?? "(unknown)"}`\n",
            });
        }

        var id = agentIdOverride ?? ToKebab(progId);
        return new GeneratedAgent
        {
            Id = id,
            Version = "0.1.0",
            Description = descriptionForAgent,
            License = "UNKNOWN",
            Stateful = false,
            Commands = commands.ToArray(),
            Skills = skills.ToArray(),
        };
    }

    private static void EnumerateTypeLib(ITypeLib typeLib, string progId, List<GeneratedCommand> commands, List<GeneratedSkill> skills)
    {
        int typeCount = typeLib.GetTypeInfoCount();
        for (int i = 0; i < typeCount; i++)
        {
            ITypeInfo? typeInfo = null;
            try
            {
                typeLib.GetTypeInfo(i, out typeInfo);
                if (typeInfo == null) continue;

                typeLib.GetDocumentation(i, out string? typeName, out string? typeDocString, out _, out _);
                typeInfo.GetTypeAttr(out IntPtr attrPtr);
                if (attrPtr == IntPtr.Zero) continue;
                try
                {
                    var attr = Marshal.PtrToStructure<TYPEATTR>(attrPtr);
                    if (attr.typekind != TYPEKIND.TKIND_DISPATCH && attr.typekind != TYPEKIND.TKIND_INTERFACE)
                        continue;

                    for (int f = 0; f < attr.cFuncs; f++)
                    {
                        typeInfo.GetFuncDesc(f, out IntPtr funcPtr);
                        if (funcPtr == IntPtr.Zero) continue;
                        try
                        {
                            var func = Marshal.PtrToStructure<FUNCDESC>(funcPtr);
                            // Skip property accessors
                            if (func.invkind != INVOKEKIND.INVOKE_FUNC) continue;

                            typeInfo.GetDocumentation(func.memid, out string? funcName, out string? funcDoc, out _, out _);
                            if (string.IsNullOrEmpty(funcName)) continue;

                            commands.Add(new GeneratedCommand
                            {
                                Name = ToKebab($"{typeName ?? "type"}-{funcName}"),
                                Lifecycle = "single",
                                Description = funcDoc ?? $"{typeName}.{funcName} ({progId})",
                            });
                        }
                        finally
                        {
                            typeInfo.ReleaseFuncDesc(funcPtr);
                        }
                    }
                }
                finally
                {
                    typeInfo.ReleaseTypeAttr(attrPtr);
                }
            }
            finally
            {
                if (typeInfo != null) Marshal.ReleaseComObject(typeInfo);
            }
        }
    }

    /// <summary>Returns (path, version) or (null, null) on lookup failure.</summary>
    private static (string?, string?) LookupTypeLib(Guid clsid)
    {
        try
        {
            using var clsKey = Registry.ClassesRoot.OpenSubKey($"CLSID\\{clsid:B}\\TypeLib");
            var typeLibGuidString = clsKey?.GetValue(null) as string;
            if (string.IsNullOrEmpty(typeLibGuidString)) return (null, null);

            using var tlKey = Registry.ClassesRoot.OpenSubKey($"TypeLib\\{typeLibGuidString}");
            if (tlKey == null) return (null, typeLibGuidString);

            // Find the highest version subkey
            string? bestVer = null;
            foreach (var ver in tlKey.GetSubKeyNames())
            {
                if (bestVer == null || string.CompareOrdinal(ver, bestVer) > 0)
                    bestVer = ver;
            }
            if (bestVer == null) return (null, typeLibGuidString);

            using var verKey = tlKey.OpenSubKey(bestVer);
            // Try win64 first then win32
            foreach (var arch in new[] { "win64", "win32" })
            {
                using var archKey = verKey?.OpenSubKey($"0\\{arch}");
                var path = archKey?.GetValue(null) as string;
                if (!string.IsNullOrEmpty(path) && File.Exists(path))
                    return (path, bestVer);
            }
            return (null, typeLibGuidString);
        }
        catch
        {
            return (null, null);
        }
    }

    private static string ToKebab(string s)
    {
        var sb = new System.Text.StringBuilder(s.Length + 4);
        var prevLower = false;
        foreach (var c in s)
        {
            if (char.IsUpper(c))
            {
                if (prevLower && sb.Length > 0) sb.Append('-');
                sb.Append(char.ToLowerInvariant(c));
                prevLower = false;
            }
            else if (char.IsWhiteSpace(c) || c == '_' || c == '.' || c == '-')
            {
                if (sb.Length > 0 && sb[sb.Length - 1] != '-') sb.Append('-');
                prevLower = false;
            }
            else if (char.IsLetterOrDigit(c))
            {
                sb.Append(char.ToLowerInvariant(c));
                prevLower = char.IsLower(c) || char.IsDigit(c);
            }
        }
        return sb.ToString().Trim('-');
    }

    [DllImport("ole32.dll", CharSet = CharSet.Unicode)]
    private static extern int CLSIDFromProgID(string lpszProgID, out Guid clsid);

    // IL2050: LoadTypeLibEx returns an ITypeLib COM interface via out-param marshalling.
    // We call Marshal.ReleaseComObject on it immediately after use, so the COM ref-count
    // is managed manually. The interface definition (ITypeLib) is part of the BCL and will
    // not be trimmed by the ILC. Suppressing IL2050 here is safe for NativeAOT on Windows.
    [UnconditionalSuppressMessage("AOT", "IL2050", Justification = "ITypeLib is BCL-defined and retained; COM ref managed via Marshal.ReleaseComObject")]
    [DllImport("oleaut32.dll", CharSet = CharSet.Unicode)]
    private static extern int LoadTypeLibEx(string szFile, RegKind regkind, out ITypeLib pptlib);

    private enum RegKind { Default = 0, Register = 1, None = 2 }
}
