// A2 SPIKE Run 3 (aware#858). READ-ONLY. Lists every parametric item whose prefix starts with args.starts_with (default "AW"), with its type and parameter string, plus the total. Writes nothing.
using System;
using System.Collections.Generic;
using Tekla.Structures.Catalogs;
string starts = "AW";
object value;
if (args != null && args.TryGetValue("starts_with", out value) && value is string s0 && s0.Length > 0) starts = s0;
var found = new List<object>();
int total = 0;
var en = new CatalogHandler().GetParametricProfileItems();
while (en.MoveNext())
{
    var item = en.Current as ParametricProfileItem;
    if (item == null) continue;
    total++;
    var prefix = item.ProfilePrefix ?? "";
    if (prefix.StartsWith(starts, StringComparison.Ordinal))
        found.Add(new Dictionary<string, object> { { "prefix", prefix }, { "prefix_length", prefix.Length }, { "type", item.ProfileItemType.ToString() }, { "subtype", item.ProfileItemSubType.ToString() }, { "parameter_string", item.ParameterString ?? "" }, { "clb", item.IsCLBProfile }, { "sketched", item.IsSketchedUserParametric } });
}
return new Dictionary<string, object> { { "total", total }, { "found", found } };
