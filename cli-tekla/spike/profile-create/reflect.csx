// A2 SPIKE Run 3 (aware#858). READ-ONLY reflection: lists the public members of CatalogHandler and ParametricProfileItem. Writes nothing.
using System;
using System.Linq;
using System.Collections.Generic;
using Tekla.Structures.Catalogs;
var o = new Dictionary<string, object>();
foreach (var t in new[] { typeof(CatalogHandler), typeof(ParametricProfileItem), typeof(ProfileItem) })
    o[t.Name] = t.GetMembers(System.Reflection.BindingFlags.Public | System.Reflection.BindingFlags.Instance | System.Reflection.BindingFlags.Static | System.Reflection.BindingFlags.DeclaredOnly).Select(x => x.ToString()).ToList();
return o;
