// A2 SPIKE Run 3 (aware#858). READ-ONLY. For each prefix in args.prefixes: SelectByPrefix, then report CreateProfileString(), ParameterString, the parameter defaults and the item type flags. Writes nothing.
using System;
using System.Collections.Generic;
using Tekla.Structures.Catalogs;
object value;
var rows = new List<object>();
if (args != null && args.TryGetValue("prefixes", out value) && value is System.Collections.IList wanted)
    foreach (var raw in wanted)
    {
        var prefix = raw as string;
        if (String.IsNullOrWhiteSpace(prefix)) continue;
        var item = new ParametricProfileItem();
        bool sel = false; string str = null, err = null;
        try { sel = item.SelectByPrefix(prefix); if (sel) str = item.CreateProfileString(); } catch (Exception ex) { err = ex.Message; }
        var pars = new List<object>();
        if (sel && item.aProfileItemParameters != null)
            foreach (var p in item.aProfileItemParameters) { var pp = p as ProfileItemParameter; if (pp != null) pars.Add(new Dictionary<string, object> { { "symbol", pp.Symbol }, { "value", pp.Value }, { "property", pp.Property } }); }
        rows.Add(new Dictionary<string, object> { { "prefix", prefix }, { "selected", sel }, { "profile_string", str }, { "parameter_string", sel ? item.ParameterString : null }, { "parameters", pars }, { "error", err } });
    }
return new Dictionary<string, object> { { "rows", rows } };
