namespace AwareTekla;

/// <summary>
/// Canonical scene-to-Tekla materializer. It deliberately lives as a Roslyn
/// script so the sidecar stays version-neutral across Tekla 2025 and 2026.
/// </summary>
internal static class BakeSceneScript
{
    internal const string Code = """
using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Tekla.Structures;
using Tekla.Structures.Model;
using Tekla.Structures.Geometry3d;

var inv = CultureInfo.InvariantCulture;
Func<double,bool> finite = d => !Double.IsNaN(d) && !Double.IsInfinity(d);
Func<object,double> number = o => {
    if (o == null) return Double.NaN;
    if (o is double d) return d;
    if (o is long l) return l;
    if (o is int i) return i;
    double value;
    return Double.TryParse(o.ToString(), NumberStyles.Float, inv, out value) ? value : Double.NaN;
};
Func<object,string> text = o => o == null ? "" : o.ToString();
Func<IDictionary<string,object>,string,string> str = (o,k) => o != null && o.TryGetValue(k, out var v) ? text(v) : "";
Func<IDictionary<string,object>,string,IDictionary<string,object>> obj = (o,k) => o != null && o.TryGetValue(k, out var v) ? v as IDictionary<string,object> : null;
Func<IDictionary<string,object>,string,IList> list = (o,k) => o != null && o.TryGetValue(k, out var v) ? v as IList : null;
Func<IDictionary<string,object>,string,bool> booleanField = (o,k) => o != null && o.TryGetValue(k, out var v) && v is bool;
Func<IList,bool> vec3ok = v => v != null && v.Count >= 3 && Enumerable.Range(0,3).All(i => finite(number(v[i])));
Func<IList,Point> point = v => new Point(number(v[0]), number(v[1]), number(v[2]));
Func<IList,Vector> vector = v => new Vector(number(v[0]), number(v[1]), number(v[2]));
Func<Vector,double> length = v => Math.Sqrt(v.X*v.X + v.Y*v.Y + v.Z*v.Z);
Func<Vector,Vector> unit = v => { var n=length(v); if (!(n > 1e-9)) throw new Exception("zero-length direction"); return new Vector(v.X/n,v.Y/n,v.Z/n); };
Func<Vector,Vector,Vector> cross = (a,b) => new Vector(a.Y*b.Z-a.Z*b.Y,a.Z*b.X-a.X*b.Z,a.X*b.Y-a.Y*b.X);
Func<Vector,Vector,double> dot = (a,b) => a.X*b.X+a.Y*b.Y+a.Z*b.Z;
Func<Point,Vector,double,Point> move = (p,v,s) => new Point(p.X+v.X*s,p.Y+v.Y*s,p.Z+v.Z*s);
Func<double,string> mm = d => d.ToString("0.############", inv);
Func<IEnumerable<double>,string> spacing = values => {
    var ordered=values.OrderBy(x=>x).ToList();
    if(ordered.Count==0)return "0";
    var tokens=new List<string>{mm(ordered[0])};
    for(int i=1;i<ordered.Count;i++)tokens.Add(mm(ordered[i]-ordered[i-1]));
    return String.Join(" ",tokens);
};
Func<IList,List<Tuple<double,double>>> polygonPoints = raw => {
    if(raw==null)return null;
    var points=new List<Tuple<double,double>>();
    foreach(var item in raw){var xy=item as IList;if(xy==null||xy.Count<2||!finite(number(xy[0]))||!finite(number(xy[1])))return null;points.Add(Tuple.Create(number(xy[0]),number(xy[1])));}
    return points;
};
Func<List<Tuple<double,double>>,bool> simplePolygon = points => {
    if(points==null||points.Count<3)return false;
    Func<Tuple<double,double>,Tuple<double,double>,Tuple<double,double>,double> orient=(a,b,c)=>(b.Item1-a.Item1)*(c.Item2-a.Item2)-(b.Item2-a.Item2)*(c.Item1-a.Item1);
    Func<Tuple<double,double>,Tuple<double,double>,Tuple<double,double>,bool> on=(a,b,p)=>Math.Abs(orient(a,b,p))<=1e-9&&p.Item1>=Math.Min(a.Item1,b.Item1)-1e-9&&p.Item1<=Math.Max(a.Item1,b.Item1)+1e-9&&p.Item2>=Math.Min(a.Item2,b.Item2)-1e-9&&p.Item2<=Math.Max(a.Item2,b.Item2)+1e-9;
    Func<Tuple<double,double>,Tuple<double,double>,Tuple<double,double>,Tuple<double,double>,bool> intersects=(a,b,c,d)=>{double ab_c=orient(a,b,c),ab_d=orient(a,b,d),cd_a=orient(c,d,a),cd_b=orient(c,d,b);return (Math.Sign(ab_c)!=Math.Sign(ab_d)&&Math.Sign(cd_a)!=Math.Sign(cd_b))||on(a,b,c)||on(a,b,d)||on(c,d,a)||on(c,d,b);};
    double area2=0;for(int i=0;i<points.Count;i++){var a=points[i];var b=points[(i+1)%points.Count];if(Math.Sqrt(Math.Pow(a.Item1-b.Item1,2)+Math.Pow(a.Item2-b.Item2,2))<=1e-9)return false;area2+=a.Item1*b.Item2-b.Item1*a.Item2;}
    if(Math.Abs(area2)<=1e-9)return false;
    for(int i=0;i<points.Count;i++)for(int j=i+1;j<points.Count;j++){if(j==(i+1)%points.Count||(j+1)%points.Count==i)continue;if(intersects(points[i],points[(i+1)%points.Count],points[j],points[(j+1)%points.Count]))return false;}
    return true;
};
Func<Tuple<double,double>,List<Tuple<double,double>>,bool> insidePolygon = (p,points) => {bool inside=false;for(int i=0;i<points.Count;i++){var a=points[i];var b=points[(i+1)%points.Count];if((a.Item2>p.Item2)!=(b.Item2>p.Item2)&&p.Item1<(b.Item1-a.Item1)*(p.Item2-a.Item2)/(b.Item2-a.Item2)+a.Item1)inside=!inside;}return inside;};
Func<Tuple<double,double>,Tuple<double,double>,Tuple<double,double>,double> pointSegmentDistance = (p,a,b) => {double dx=b.Item1-a.Item1,dy=b.Item2-a.Item2,n=dx*dx+dy*dy;if(n<=1e-18)return Math.Sqrt(Math.Pow(p.Item1-a.Item1,2)+Math.Pow(p.Item2-a.Item2,2));double t=Math.Max(0,Math.Min(1,((p.Item1-a.Item1)*dx+(p.Item2-a.Item2)*dy)/n));return Math.Sqrt(Math.Pow(p.Item1-(a.Item1+t*dx),2)+Math.Pow(p.Item2-(a.Item2+t*dy),2));};
Func<string,string,string,string,string,Dictionary<string,object>> row = (id,kind,status,code,message) => {
    var r = new Dictionary<string,object> { {"id",id}, {"kind",kind}, {"status",status} };
    if (!String.IsNullOrEmpty(code)) r["code"] = code;
    if (!String.IsNullOrEmpty(message)) r["message"] = message;
    return r;
};

var m = model as Model;
if (m == null || !m.GetConnectionStatus())
    return new { ok=false, emitted=new object[0], failed=new[]{row("scene","scene","failed","host-unavailable","No live Tekla model is open.")}, unsupported=new object[0], warnings=new object[0] };

var scene = args != null && args.TryGetValue("scene", out var sv) ? sv as IDictionary<string,object> : null;
if (scene == null)
    return new { ok=false, emitted=new object[0], failed=new[]{row("scene","scene","failed","invalid-scene","scene must be an object")}, unsupported=new object[0], warnings=new object[0] };
var meta = obj(scene,"meta");
string sceneName = meta == null ? "Steel from Drawings" : str(meta,"name");
if (String.IsNullOrWhiteSpace(sceneName)) sceneName = "Steel from Drawings";
string units = meta == null ? "" : str(meta,"units");
string sourceId = meta == null ? "" : str(meta,"sourceId").Trim();
string sceneHash = meta == null ? "" : str(meta,"sceneHash").Trim();
string materializationHash = args != null && args.TryGetValue("materializationHash", out var mh) ? text(mh) : "";
Func<string,string> advancedOption = name => { string value=""; try { TeklaStructuresSettings.GetAdvancedOption(name, ref value); } catch {} return value ?? ""; };
string environmentSignature = String.Join("\u001f", new[]{advancedOption("XS_SYSTEM"),advancedOption("XS_FIRM"),advancedOption("XS_PROJECT")});
Func<string,string> sha256 = value => { using(var sha=System.Security.Cryptography.SHA256.Create()) return String.Concat(sha.ComputeHash(System.Text.Encoding.UTF8.GetBytes(value)).Select(b=>b.ToString("x2",inv))); };
string sourceKey = "AWARE_SRC_V1:"+sha256(sourceId);
string sceneKey = "AWARE_SCN_V1:"+sha256(sceneHash);
materializationHash=sha256(materializationHash+"\0"+environmentSignature);
Func<string,string> recordKey = id => "AWARE_RID_V1:"+sha256(id);
// Tekla USER_FIELD strings are commonly capped near 79 characters. This
// namespaced marker plus the 64-character SHA-256 digest is exactly 78.
string ownershipMarker = "AWARE_BAKE_V1:"+materializationHash;
string attemptId = Guid.NewGuid().ToString("N");

var emitted = new List<Dictionary<string,object>>();
var failed = new List<Dictionary<string,object>>();
var unsupported = new List<Dictionary<string,object>>();
var warnings = new List<Dictionary<string,object>>();
var supportedOrder = new List<Tuple<string,string>>();
var seen = new HashSet<string>(StringComparer.Ordinal);
IList elements, operations, references;
object collectionValue;
if(!scene.TryGetValue("elements",out collectionValue))elements=new object[0];else{elements=collectionValue as IList;if(elements==null){failed.Add(row("scene","scene","failed","invalid-collection","scene.elements must be an array"));elements=new object[0];}}
if(!scene.TryGetValue("operations",out collectionValue))operations=new object[0];else{operations=collectionValue as IList;if(operations==null){failed.Add(row("scene","scene","failed","invalid-collection","scene.operations must be an array"));operations=new object[0];}}
if(!scene.TryGetValue("referenceSystems",out collectionValue))references=new object[0];else{references=collectionValue as IList;if(references==null){failed.Add(row("scene","scene","failed","invalid-collection","scene.referenceSystems must be an array"));references=new object[0];}}

Func<string,string,bool> acceptId = (id,kind) => {
    if (String.IsNullOrWhiteSpace(id) || id != id.Trim() || System.Text.Encoding.UTF8.GetByteCount(id) > 256 || id.Any(c => c < 32 || c == 127)) {
        failed.Add(row(String.IsNullOrEmpty(id) ? "scene" : id,kind,"failed","invalid-id","id must be unique, trimmed, 1-256 UTF-8 bytes, and contain no control characters"));
        return false;
    }
    if (!seen.Add(id)) { failed.Add(row(id,kind,"failed","duplicate-id","id is duplicated in the scene")); return false; }
    return true;
};

if (!String.IsNullOrEmpty(units) && !String.Equals(units,"mm",StringComparison.OrdinalIgnoreCase))
    failed.Add(row("scene","scene","failed","unsupported-units","Tekla bake-scene accepts only millimetres"));
if (String.IsNullOrWhiteSpace(sourceId)) failed.Add(row("scene","scene","failed","missing-source-id","scene.meta.sourceId is required"));
if (String.IsNullOrWhiteSpace(sceneHash)) failed.Add(row("scene","scene","failed","missing-scene-hash","scene.meta.sceneHash is required"));

var elementById = new Dictionary<string,IDictionary<string,object>>(StringComparer.Ordinal);
foreach (var raw in elements) {
    var el = raw as IDictionary<string,object>;
    if (el == null) { failed.Add(row("scene","element","failed","invalid-record","element must be an object")); continue; }
    string id=str(el,"id"), kind=str(el,"kind").ToLowerInvariant(); if (String.IsNullOrEmpty(kind)) kind="member";
    if (!acceptId(id,kind)) continue;
    if (kind=="member" || kind=="line" || kind=="box" || kind=="plate" || kind=="rod" || kind=="bolt-shank" || kind=="washer" || kind=="nut" || kind=="bolt-head") { elementById[id]=el; supportedOrder.Add(Tuple.Create(id,kind)); }
    else unsupported.Add(row(id,kind,"unsupported","unsupported-kind","element kind is not supported by the Tekla sink"));
    if (kind=="plate") {
        var holes=list(el,"holes"); if(el.ContainsKey("holes")&&holes==null)failed.Add(row(id,"opening","failed","invalid-collection","plate holes must be an array"));else if (holes!=null) foreach(var hr in holes) { var h=hr as IDictionary<string,object>; if(h==null){failed.Add(row(id,"opening","failed","invalid-record","plate hole must be an object"));continue;} string hid=str(h,"id"); if(acceptId(hid,"opening")) supportedOrder.Add(Tuple.Create(hid,"opening")); }
    }
}
var operationById = new Dictionary<string,IDictionary<string,object>>(StringComparer.Ordinal);
foreach (var raw in operations) {
    var op=raw as IDictionary<string,object>; if(op==null){failed.Add(row("scene","operation","failed","invalid-record","operation must be an object"));continue;}
    string id=str(op,"id"), kind=str(op,"kind").ToLowerInvariant(); if(!acceptId(id,kind))continue;
    if(kind=="bolt-array" || kind=="weld" || kind=="boolean-cut") { operationById[id]=op; supportedOrder.Add(Tuple.Create(id,kind)); } else unsupported.Add(row(id,kind,"unsupported","unsupported-operation","operation kind is not supported by the Tekla sink"));
    if(kind=="bolt-array") { var ins=list(op,"instances"); if(op.ContainsKey("instances")&&ins==null)failed.Add(row(id,"bolt-instance","failed","invalid-collection","bolt instances must be an array"));else if(ins!=null) foreach(var ir in ins) { var inst=ir as IDictionary<string,object>; if(inst==null){failed.Add(row(id,"bolt-instance","failed","invalid-record","bolt instance must be an object"));continue;} var iid=str(inst,"id"); if(acceptId(iid,"bolt-instance")) supportedOrder.Add(Tuple.Create(iid,"bolt-instance")); var effects=list(inst,"holeEffects"); if(inst.ContainsKey("holeEffects")&&effects==null)failed.Add(row(iid,"opening","failed","invalid-collection","hole effects must be an array"));else if(effects!=null) foreach(var er in effects){var effect=er as IDictionary<string,object>; if(effect==null){failed.Add(row(iid,"opening","failed","invalid-record","hole effect must be an object"));continue;} var eid=str(effect,"id"); if(acceptId(eid,"opening")) supportedOrder.Add(Tuple.Create(eid,"opening"));} } }
}
var referenceById = new Dictionary<string,IDictionary<string,object>>(StringComparer.Ordinal);
foreach(var raw in references){var rf=raw as IDictionary<string,object>;if(rf==null){failed.Add(row("scene","reference-system","failed","invalid-record","reference system must be an object"));continue;}string id=str(rf,"id"),kind=str(rf,"kind").ToLowerInvariant();if(!acceptId(id,kind))continue;bool supported=kind=="structural-grid";if(supported){referenceById[id]=rf;supportedOrder.Add(Tuple.Create(id,kind));}else unsupported.Add(row(id,kind,"unsupported","unsupported-reference-system","reference system kind is not supported by the Tekla sink"));var axes=list(rf,"axes");if(rf.ContainsKey("axes")&&axes==null)failed.Add(row(id,"grid-axis","failed","invalid-collection","grid axes must be an array"));else if(axes!=null)foreach(var ar in axes){var axis=ar as IDictionary<string,object>;if(axis==null){failed.Add(row(id,"grid-axis","failed","invalid-record","grid axis must be an object"));continue;}var aid=str(axis,"id");if(acceptId(aid,"grid-axis")){if(supported)supportedOrder.Add(Tuple.Create(aid,"grid-axis"));else unsupported.Add(row(aid,"grid-axis","unsupported","unsupported-parent","parent reference system is unsupported"));}}var levels=list(rf,"levels");if(rf.ContainsKey("levels")&&levels==null)failed.Add(row(id,"grid-level","failed","invalid-collection","grid levels must be an array"));else if(levels!=null)foreach(var lr in levels){var level=lr as IDictionary<string,object>;if(level==null){failed.Add(row(id,"grid-level","failed","invalid-record","grid level must be an object"));continue;}var lid=str(level,"id");if(acceptId(lid,"grid-level")){if(supported)supportedOrder.Add(Tuple.Create(lid,"grid-level"));else unsupported.Add(row(lid,"grid-level","unsupported","unsupported-parent","parent reference system is unsupported"));}}}

Action<string> appendBatchAborted = message => {var failedIds=new HashSet<string>(failed.Select(r=>text(r["id"])),StringComparer.Ordinal);foreach(var item in supportedOrder)if(failedIds.Add(item.Item1))failed.Add(row(item.Item1,item.Item2,"failed","batch-aborted",message));};
if(failed.Count>0) {appendBatchAborted("Batch was aborted because scene identity or envelope validation failed.");return new { ok=false, sourceId, sceneHash, materializationHash, attemptId, emitted=new object[0], failed, unsupported, warnings };}

var realizedChildren = new Dictionary<string,string>(StringComparer.Ordinal);
var realizedEffects = new Dictionary<string,string>(StringComparer.Ordinal);
var realizedReferences = new Dictionary<string,string>(StringComparer.Ordinal);
foreach(var kv in operationById) if(str(kv.Value,"kind")=="bolt-array") { var ins=list(kv.Value,"instances"); if(ins==null)continue; foreach(var ir in ins){var inst=ir as IDictionary<string,object>;if(inst==null)continue;var iid=str(inst,"id");if(!String.IsNullOrEmpty(iid))realizedChildren[iid]=kv.Key;foreach(var key in new[]{"shankId","headId"}){var cid=str(inst,key);if(!String.IsNullOrEmpty(cid))realizedChildren[cid]=kv.Key;}foreach(var key in new[]{"nutIds","washerIds"}){var ids=list(inst,key);if(ids!=null)foreach(var x in ids){var cid=text(x);if(!String.IsNullOrEmpty(cid))realizedChildren[cid]=kv.Key;}}var effects=list(inst,"holeEffects");if(effects!=null)foreach(var er in effects){var effect=er as IDictionary<string,object>;if(effect!=null)realizedEffects[str(effect,"id")]=kv.Key;}}}
foreach(var kv in referenceById){var axes=list(kv.Value,"axes");if(axes!=null)foreach(var ar in axes){var axis=ar as IDictionary<string,object>;if(axis!=null)realizedReferences[str(axis,"id")]=kv.Key;}var levels=list(kv.Value,"levels");if(levels!=null)foreach(var lr in levels){var level=lr as IDictionary<string,object>;if(level!=null)realizedReferences[str(level,"id")]=kv.Key;}}

// Complete trust-boundary validation before the first Tekla Insert. Native API
// failures remain possible, but malformed geometry/relationships never start a batch.
foreach(var pair in elementById) try {var id=pair.Key;var el=pair.Value;var kind=str(el,"kind").ToLowerInvariant();if(String.IsNullOrEmpty(kind))kind="member";
    if(kind=="member"||kind=="line"||kind=="box"){var from=list(el,"from");var to=list(el,"to");var em=obj(el,"meta");string profile=str(el,"profile");if(String.IsNullOrWhiteSpace(profile)&&em!=null)profile=str(em,"profile");if(!vec3ok(from)||!vec3ok(to)||Distance.PointToPoint(point(from),point(to))<=1e-6||String.IsNullOrWhiteSpace(profile))throw new Exception("member requires a nonzero finite axis and exact profile");}
    else if(kind=="plate"){
        var frame=obj(el,"frame");var origin=list(frame,"origin");var u=list(frame,"uDir");var v=list(frame,"vDir");var n=list(frame,"normal");var outline=list(el,"outline");double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;
        var polygon=polygonPoints(outline);
        if(!vec3ok(origin)||!vec3ok(u)||!vec3ok(v)||!vec3ok(n)||!simplePolygon(polygon)||!(th>0))throw new Exception("invalid plate frame, simple outline, or thickness");
        var uu=unit(vector(u));var vv=unit(vector(v));var nn=unit(vector(n));if(Math.Abs(dot(uu,vv))>1e-6||Math.Abs(dot(unit(cross(uu,vv)),nn)-1)>1e-6)throw new Exception("plate frame is not right-handed orthonormal");
        var holes=list(el,"holes");if(holes!=null){var circles=new List<Tuple<double,double,double>>();foreach(var hr in holes){var h=hr as IDictionary<string,object>;var c=list(h,"center");double d=h.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(c==null||c.Count<2||!finite(number(c[0]))||!finite(number(c[1]))||!(d>0))throw new Exception(str(h,"id")+": invalid plate hole");var cp=Tuple.Create(number(c[0]),number(c[1]));if(!insidePolygon(cp,polygon)||Enumerable.Range(0,polygon.Count).Any(i=>pointSegmentDistance(cp,polygon[i],polygon[(i+1)%polygon.Count])+1e-9<d/2))throw new Exception(str(h,"id")+": plate hole must lie wholly inside the outline");foreach(var prior in circles)if(Math.Sqrt(Math.Pow(prior.Item1-cp.Item1,2)+Math.Pow(prior.Item2-cp.Item2,2))<=(prior.Item3+d)/2+0.1)throw new Exception(str(h,"id")+": plate holes overlap or touch");circles.Add(Tuple.Create(cp.Item1,cp.Item2,d));}}
    }
    else if(kind=="rod"||kind=="bolt-shank"){var axis=obj(el,"axis");var from=list(axis,"from");var to=list(axis,"to");double d=el.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(!vec3ok(from)||!vec3ok(to)||Distance.PointToPoint(point(from),point(to))<=1e-6||!(d>0))throw new Exception("invalid rod geometry");}
    else if(kind=="washer"){var c=list(el,"center");var a=list(el,"axis");double od=el.TryGetValue("outerDiameterMm",out var ov)?number(ov):Double.NaN;double inn=el.TryGetValue("innerDiameterMm",out var iv)?number(iv):Double.NaN;double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;if(!vec3ok(c)||!vec3ok(a)||!(length(vector(a))>1e-9)||!(od>inn&&inn>0&&th>0))throw new Exception("invalid washer geometry");}
    else {var c=list(el,"center");var a=list(el,"axis");double af=el.TryGetValue("acrossFlatsMm",out var fv)?number(fv):Double.NaN;double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;if(!vec3ok(c)||!vec3ok(a)||!(length(vector(a))>1e-9)||!(af>0&&th>0))throw new Exception("invalid hex-prism geometry");}
}catch(Exception ex){failed.Add(row(pair.Key,str(pair.Value,"kind"),"failed","invalid-geometry",ex.Message));}

var claimedBoltChildren = new HashSet<string>(StringComparer.Ordinal);
foreach(var pair in operationById) try {var id=pair.Key;var op=pair.Value;var kind=str(op,"kind");
    if(kind=="bolt-array"){
        string pa=str(op,"partToBoltTo"),pb=str(op,"partToBeBolted");
        if(!elementById.ContainsKey(pa)||!elementById.ContainsKey(pb)||realizedChildren.ContainsKey(pa)||realizedChildren.ContainsKey(pb))throw new Exception("bolt participants must reference independently materialized physical parts");
        var frame=obj(op,"frame");var origin=list(frame,"origin");var u=list(frame,"uDir");var v=list(frame,"vDir");var n=list(frame,"normal");
        var us=list(op,"uOffsetsMm");var vs=list(op,"vOffsetsMm");var ins=list(op,"instances");
        double dia=op.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;double tol=op.TryGetValue("toleranceMm",out var tv)?number(tv):0;
        string boltType=str(op,"boltType");var components=obj(op,"components");
        if(boltType!="shop"&&boltType!="site")throw new Exception("boltType must be `shop` or `site`");
        if(op.ContainsKey("threadInMaterial")&&!booleanField(op,"threadInMaterial"))throw new Exception("threadInMaterial must be boolean");
        if(components==null)throw new Exception("components must be an object");
        foreach(var field in new[]{"bolt","nut","washer"})if(components.ContainsKey(field)&&!booleanField(components,field))throw new Exception("components."+field+" must be boolean");
        if(pa==pb||!vec3ok(origin)||!vec3ok(u)||!vec3ok(v)||!vec3ok(n)||us==null||vs==null||ins==null||us.Count==0||vs.Count==0||!(dia>0)||!(tol>=0)||String.IsNullOrWhiteSpace(str(op,"standard")))throw new Exception("bolt array requires distinct participants, an exact frame, offsets, diameter, standard, and instances");
        var uu=unit(vector(u));var vv=unit(vector(v));var nn=unit(vector(n));
        if(Math.Abs(dot(uu,vv))>1e-6||Math.Abs(dot(unit(cross(uu,vv)),nn)-1)>1e-6)throw new Exception("bolt frame is not right-handed orthonormal");
        var uValues=us.Cast<object>().Select(number).ToList();var vValues=vs.Cast<object>().Select(number).ToList();
        if(uValues.Any(x=>!finite(x))||vValues.Any(x=>!finite(x))||uValues.Distinct().Count()!=uValues.Count||vValues.Distinct().Count()!=vValues.Count||ins.Count!=us.Count*vs.Count)throw new Exception("bolt offsets must be finite and unique, and the instance count must match their Cartesian product");
        var unmatched=new List<Point>();foreach(var x in us)foreach(var y in vs)unmatched.Add(move(move(point(origin),uu,number(x)),vv,number(y)));
        int? authoredNutSlots=null,authoredWasherSlots=null;
        foreach(var ir in ins){
            var inst=ir as IDictionary<string,object>;var p=list(inst,"point");
            if(!vec3ok(p))throw new Exception(str(inst,"id")+": instance point must be finite");
            var instancePoint=point(p);int match=unmatched.FindIndex(q=>Distance.PointToPoint(q,instancePoint)<=0.1);
            if(match<0)throw new Exception(str(inst,"id")+": instance point does not uniquely match the authored Cartesian offsets");
            unmatched.RemoveAt(match);
            string shankId=str(inst,"shankId");IDictionary<string,object> shank;
            if(!elementById.TryGetValue(shankId,out shank)||str(shank,"kind")!="bolt-shank"||!claimedBoltChildren.Add(shankId))throw new Exception(str(inst,"id")+": shankId must uniquely reference a bolt-shank");
            var shankAxis=obj(shank,"axis");var shankFrom=list(shankAxis,"from");var shankTo=list(shankAxis,"to");double shankDiameter=shank.TryGetValue("diameterMm",out var sdv)?number(sdv):Double.NaN;
            if(!vec3ok(shankFrom)||!vec3ok(shankTo)||Math.Abs(shankDiameter-dia)>0.1)throw new Exception(str(inst,"id")+": shank geometry must match the bolt diameter");
            var shankStart=point(shankFrom);var shankDirection=unit(new Vector(number(shankTo[0])-number(shankFrom[0]),number(shankTo[1])-number(shankFrom[1]),number(shankTo[2])-number(shankFrom[2])));var shankOffset=new Vector(instancePoint.X-shankStart.X,instancePoint.Y-shankStart.Y,instancePoint.Z-shankStart.Z);
            if(Math.Abs(Math.Abs(dot(shankDirection,nn))-1)>1e-6||length(cross(shankOffset,shankDirection))>0.1)throw new Exception(str(inst,"id")+": shank axis must pass through the instance point and align with the bolt normal");
            Func<string,string,bool> validateAxialChild=(childId,expectedKind)=>{IDictionary<string,object> child;if(!elementById.TryGetValue(childId,out child)||str(child,"kind")!=expectedKind||!claimedBoltChildren.Add(childId))return false;var center=list(child,"center");var axis=list(child,"axis");if(!vec3ok(center)||!vec3ok(axis)||!(length(vector(axis))>1e-9))return false;var childDirection=unit(vector(axis));var childCenter=point(center);var offset=new Vector(childCenter.X-instancePoint.X,childCenter.Y-instancePoint.Y,childCenter.Z-instancePoint.Z);return Math.Abs(Math.Abs(dot(childDirection,nn))-1)<=1e-6&&length(cross(offset,nn))<=0.1;};
            string headId=str(inst,"headId");if(!validateAxialChild(headId,"bolt-head"))throw new Exception(str(inst,"id")+": headId must uniquely reference an aligned bolt-head on the instance axis");
            foreach(var field in new[]{"nutIds","washerIds"}){var ids=list(inst,field);if(ids==null)throw new Exception(str(inst,"id")+": "+field+" is required");int maxSlots=field=="nutIds"?2:3;var priorSlots=field=="nutIds"?authoredNutSlots:authoredWasherSlots;if(ids.Count>maxSlots)throw new Exception(str(inst,"id")+": "+field+" exceeds Tekla BoltArray's "+maxSlots+" native slots");if(priorSlots.HasValue&&priorSlots.Value!=ids.Count)throw new Exception(id+": every bolt instance must author the same "+field+" cardinality for one native BoltArray");if(field=="nutIds")authoredNutSlots=ids.Count;else authoredWasherSlots=ids.Count;string expectedKind=field=="nutIds"?"nut":"washer";foreach(var cid in ids){var childId=text(cid);if(!validateAxialChild(childId,expectedKind))throw new Exception(str(inst,"id")+": "+field+" child "+childId+" must uniquely reference an aligned "+expectedKind+" on the instance axis");}}
            var effects=list(inst,"holeEffects");if(effects==null||effects.Count!=2)throw new Exception(str(inst,"id")+": exactly one hole effect per participant is required");
            var effectTargets=new HashSet<string>(StringComparer.Ordinal);
            foreach(var er in effects){var effect=er as IDictionary<string,object>;string target=str(effect,"targetId");var c=list(effect,"center");var axis=list(effect,"axis");double ed=effect.TryGetValue("diameterMm",out var ev)?number(ev):Double.NaN;bool aligned=vec3ok(axis)&&length(vector(axis))>1e-9&&Math.Abs(Math.Abs(dot(unit(vector(axis)),nn))-1)<=1e-6;if((target!=pa&&target!=pb)||!effectTargets.Add(target)||!vec3ok(c)||Distance.PointToPoint(point(c),instancePoint)>0.1||!aligned||Math.Abs(ed-(dia+tol))>0.1)throw new Exception(str(effect,"id")+": hole effect must match the bolt position, axis, participant, and diameter");}
            if(!effectTargets.Contains(pa)||!effectTargets.Contains(pb))throw new Exception(str(inst,"id")+": both participant hole effects are required");
        }
        if(unmatched.Count!=0)throw new Exception("bolt instances do not exhaust the authored Cartesian offsets");
        if(components.TryGetValue("bolt",out var boltEnabled)&&!Convert.ToBoolean(boltEnabled))throw new Exception("components.bolt must be true because every bolt instance authors shankId and headId physical components");
        foreach(var component in new[]{Tuple.Create("nut",authoredNutSlots.GetValueOrDefault()),Tuple.Create("washer",authoredWasherSlots.GetValueOrDefault())})if(components.TryGetValue(component.Item1,out var enabled)&&Convert.ToBoolean(enabled)!=(component.Item2>0))throw new Exception("components."+component.Item1+" must agree with the authored "+component.Item1+"Ids cardinality");
    }
    else if(kind=="weld"){string main=str(op,"mainId"),secondary=str(op,"secondaryId");if(!elementById.ContainsKey(main)||!elementById.ContainsKey(secondary)||realizedChildren.ContainsKey(main)||realizedChildren.ContainsKey(secondary)||str(op,"weldType")!="fillet")throw new Exception("fillet weld participants must be independently materialized parts and the type must be fillet");var path=list(op,"path");double size=op.TryGetValue("sizeMm",out var sv2)?number(sv2):Double.NaN;if(path==null||path.Count<2||!(size>0)||path.Cast<object>().Any(x=>!vec3ok(x as IList)))throw new Exception("fillet weld path/size is invalid");if(!op.TryGetValue("around",out var aroundValue)||!(aroundValue is bool)||!op.TryGetValue("shop",out var shopValue)||!(shopValue is bool))throw new Exception("fillet weld around and shop fields must be boolean");}
    else {string target=str(op,"targetId");if(!elementById.ContainsKey(target)||realizedChildren.ContainsKey(target))throw new Exception("boolean target must resolve to an independently materialized part");var tool=obj(op,"tool");var axis=obj(tool,"axis");var from=list(axis,"from");var to=list(axis,"to");double d=tool!=null&&tool.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(str(tool,"kind")!="cylinder"||!vec3ok(from)||!vec3ok(to)||Distance.PointToPoint(point(from),point(to))<=1e-6||!(d>0))throw new Exception("finite cylindrical boolean tool is invalid");}
}catch(Exception ex){failed.Add(row(pair.Key,str(pair.Value,"kind"),"failed","invalid-operation",ex.Message));}

var gridsWithIndependentAxisExtents=new HashSet<string>(StringComparer.Ordinal);
foreach(var pair in referenceById) try {var rf=pair.Value;var origin=list(rf,"origin");var axes=list(rf,"axes");var levels=list(rf,"levels");var bounds=obj(rf,"bounds");if(!vec3ok(origin)||axes==null||levels==null||levels.Count==0||bounds==null)throw new Exception("grid origin, axes, non-empty levels, and bounds are required");double minX=number(bounds["minX"]),maxX=number(bounds["maxX"]),minY=number(bounds["minY"]),maxY=number(bounds["maxY"]);if(!finite(minX)||!finite(maxX)||!finite(minY)||!finite(maxY)||!(maxX>minX&&maxY>minY))throw new Exception("grid bounds are invalid");foreach(var ar in axes){var a=ar as IDictionary<string,object>;double off=a.TryGetValue("offsetMm",out var ov)?number(ov):Double.NaN;string dir=str(a,"direction");if(!finite(off)||(dir!="x"&&dir!="y")||String.IsNullOrWhiteSpace(str(a,"label"))||(dir=="x"&&(off<minX||off>maxX))||(dir=="y"&&(off<minY||off>maxY)))throw new Exception(str(a,"id")+": grid axis is invalid or outside the grid bounds");double expectedStart=dir=="x"?minY:minX,expectedEnd=dir=="x"?maxY:maxX;double start=a.TryGetValue("startMm",out var sv)?number(sv):expectedStart,end=a.TryGetValue("endMm",out var ev)?number(ev):expectedEnd;if(!finite(start)||!finite(end)||!(end>start))throw new Exception(str(a,"id")+": grid axis start/end extents must be finite and increasing");if(Math.Abs(start-expectedStart)>0.1||Math.Abs(end-expectedEnd)>0.1)gridsWithIndependentAxisExtents.Add(pair.Key);}foreach(var lr in levels){var l=lr as IDictionary<string,object>;if(!finite(number(l["elevationMm"]))||String.IsNullOrWhiteSpace(str(l,"label")))throw new Exception(str(l,"id")+": grid level is invalid");}
}catch(Exception ex){failed.Add(row(pair.Key,"structural-grid","failed","invalid-grid",ex.Message));}
foreach(var gridId in gridsWithIndependentAxisExtents){var rf=referenceById[gridId];var childIds=new HashSet<string>(StringComparer.Ordinal);unsupported.Add(row(gridId,"structural-grid","unsupported","tekla-grid-axis-extents-unsupported","Tekla native Grid cannot represent independent startMm/endMm extents per axis."));var axes=list(rf,"axes");if(axes!=null)foreach(var ar in axes){var axis=ar as IDictionary<string,object>;var aid=str(axis,"id");childIds.Add(aid);unsupported.Add(row(aid,"grid-axis","unsupported","unsupported-parent","The parent grid uses independent per-axis extents that Tekla native Grid cannot represent."));realizedReferences.Remove(aid);}var levels=list(rf,"levels");if(levels!=null)foreach(var lr in levels){var level=lr as IDictionary<string,object>;var lid=str(level,"id");childIds.Add(lid);unsupported.Add(row(lid,"grid-level","unsupported","unsupported-parent","The parent grid uses independent per-axis extents that Tekla native Grid cannot represent."));realizedReferences.Remove(lid);}supportedOrder.RemoveAll(item=>item.Item1==gridId||childIds.Contains(item.Item1));referenceById.Remove(gridId);}

if(failed.Count>0){appendBatchAborted("Batch was aborted because another supported record failed preflight.");return new{ok=false,sourceId,sceneHash,materializationHash,attemptId,emitted=new object[0],failed,unsupported,warnings};}

var staged = new List<ModelObject>();
bool retirementStarted = false;
var parts = new Dictionary<string,Part>(StringComparer.Ordinal);
var nativeById = new Dictionary<string,ModelObject>(StringComparer.Ordinal);
var nativeRecordIdByGuid = new Dictionary<Guid,string>();
var profileById = new Dictionary<string,string>(StringComparer.Ordinal);
string activeId = supportedOrder.Count>0 ? supportedOrder[0].Item1 : "scene";
Action<ModelObject,string> tagAfterInsert = (o,id) => {
    staged.Add(o);
    if(o.Identifier == null || o.Identifier.GUID == Guid.Empty || m.SelectModelObject(new Identifier(o.Identifier.GUID)) == null) throw new Exception(id+": Insert did not yield an exact GUID read-back");
    if(!o.SetUserProperty("USER_FIELD_1",sourceKey) || !o.SetUserProperty("USER_FIELD_2",recordKey(id)) || !o.SetUserProperty("USER_FIELD_3",sceneKey) || !o.SetUserProperty("USER_FIELD_4",ownershipMarker)) throw new Exception(id+": post-Insert ownership UDA write failed");
    nativeById[id]=o;
    nativeRecordIdByGuid[o.Identifier.GUID]=id;
};
Action<ModelObject,string> tagTransientAfterInsert = (o,id) => {
    staged.Add(o);
    if(o.Identifier == null || o.Identifier.GUID == Guid.Empty || m.SelectModelObject(new Identifier(o.Identifier.GUID)) == null) throw new Exception(id+": transient Insert did not yield a GUID read-back");
    if(!o.SetUserProperty("USER_FIELD_1",sourceKey) || !o.SetUserProperty("USER_FIELD_2",recordKey(id)) || !o.SetUserProperty("USER_FIELD_3",sceneKey) || !o.SetUserProperty("USER_FIELD_4",ownershipMarker)) throw new Exception(id+": transient ownership UDA write failed");
};
Action<ModelObject> labelBeforeInsert = o => { if(!o.SetLabel(attemptId)) throw new Exception("failed to set pre-Insert attempt label"); };
Action<Part,IDictionary<string,object>,string> decorate = (p,el,defaultName) => { p.Name=String.IsNullOrWhiteSpace(str(el,"name"))?defaultName:str(el,"name"); p.Class=String.IsNullOrWhiteSpace(str(el,"teklaClass"))?"1":str(el,"teklaClass"); var material=str(el,"material"); if(!String.IsNullOrWhiteSpace(material))p.Material.MaterialString=material; };
Func<string,string,string,Point,Point,IDictionary<string,object>,Beam> insertBeam = (id,kind,profiles,p1,p2,el) => {
    if(Distance.PointToPoint(p1,p2)<=1e-6)throw new Exception(id+": axis has zero length");
    Exception last=null;
    foreach(var profile in profiles.Split(new[]{'|'},StringSplitOptions.RemoveEmptyEntries)) { Beam b;try{b=new Beam(p1,p2);b.Profile.ProfileString=profile;decorate(b,el,kind=="member"?"MEMBER":kind.ToUpperInvariant());b.Position.Depth=Position.DepthEnum.MIDDLE;b.Position.Plane=Position.PlaneEnum.MIDDLE;b.Position.Rotation=Position.RotationEnum.TOP;labelBeforeInsert(b);}catch(Exception ex){last=ex;continue;}bool inserted;try{inserted=b.Insert();}catch(Exception ex){last=ex;continue;}if(!inserted)continue;tagAfterInsert(b,id);var selected=m.SelectModelObject(new Identifier(b.Identifier.GUID)) as Beam;if(selected==null)throw new Exception(id+": authoritative Beam GUID read-back failed");string resolved=selected.Profile.ProfileString;if(!String.Equals(resolved,profile,StringComparison.OrdinalIgnoreCase))throw new Exception(id+": resolved profile `"+resolved+"` differs from requested `"+profile+"`");parts[id]=selected;profileById[id]=resolved;return selected;}
    throw new Exception(id+": no exact requested profile inserted"+(last==null?"":": "+last.Message));
};
Func<IDictionary<string,object>,string,ContourPlate> insertPlate = (el,id) => {
    var frame=obj(el,"frame");var origin=list(frame,"origin");var ud=list(frame,"uDir");var vd=list(frame,"vDir");var nd=list(frame,"normal");var outline=list(el,"outline");double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;
    if(!vec3ok(origin)||!vec3ok(ud)||!vec3ok(vd)||!vec3ok(nd)||outline==null||outline.Count<3||!(th>0))throw new Exception(id+": invalid plate frame, outline, or thickness");
    var u=unit(vector(ud));var v=unit(vector(vd));var n=unit(vector(nd));if(Math.Abs(dot(u,v))>1e-6||Math.Abs(dot(unit(cross(u,v)),n)-1)>1e-6)throw new Exception(id+": plate frame must be orthonormal and right-handed");
    var p=new ContourPlate();p.Profile.ProfileString="PL"+mm(th);decorate(p,el,"PLATE");p.Position.Depth=Position.DepthEnum.MIDDLE;
    foreach(var xyRaw in outline){var xy=xyRaw as IList;if(xy==null||xy.Count<2||!finite(number(xy[0]))||!finite(number(xy[1])))throw new Exception(id+": invalid outline point");var q=move(move(point(origin),u,number(xy[0])),v,number(xy[1]));p.AddContourPoint(new ContourPoint(q,new Chamfer()));}
    string requested="PL"+mm(th);labelBeforeInsert(p);if(!p.Insert())throw new Exception(id+": exact ContourPlate insert failed for "+requested);tagAfterInsert(p,id);var selected=m.SelectModelObject(new Identifier(p.Identifier.GUID)) as ContourPlate;if(selected==null)throw new Exception(id+": authoritative ContourPlate GUID read-back failed");string resolved=selected.Profile.ProfileString;if(!String.Equals(resolved,requested,StringComparison.OrdinalIgnoreCase))throw new Exception(id+": resolved profile `"+resolved+"` differs from requested `"+requested+"`");parts[id]=selected;profileById[id]=resolved;return selected;
};
Func<IDictionary<string,object>,string,string,ContourPlate> insertHex = (el,id,kind) => {
    var center=list(el,"center");var av=list(el,"axis");double af=el.TryGetValue("acrossFlatsMm",out var fv)?number(fv):Double.NaN;double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;double phase=el.TryGetValue("phaseRad",out var ph)?number(ph):0;
    if(!vec3ok(center)||!vec3ok(av)||!(af>0)||!(th>0)||!finite(phase))throw new Exception(id+": invalid hex-prism geometry");var n=unit(vector(av));var seed=Math.Abs(n.Z)<0.9?new Vector(0,0,1):new Vector(1,0,0);var u=unit(cross(seed,n));var v=unit(cross(n,u));double radius=af/Math.Sqrt(3);string requested="PL"+mm(th);var p=new ContourPlate();p.Profile.ProfileString=requested;decorate(p,el,kind=="nut"?"NUT":"BOLT HEAD");p.Position.Depth=Position.DepthEnum.MIDDLE;for(int i=0;i<6;i++){double a=phase+i*Math.PI/3;var q=move(move(point(center),u,radius*Math.Cos(a)),v,radius*Math.Sin(a));p.AddContourPoint(new ContourPoint(q,new Chamfer()));}labelBeforeInsert(p);if(!p.Insert())throw new Exception(id+": exact hex ContourPlate insert failed");tagAfterInsert(p,id);var selected=m.SelectModelObject(new Identifier(p.Identifier.GUID)) as ContourPlate;if(selected==null)throw new Exception(id+": authoritative ContourPlate GUID read-back failed");string resolved=selected.Profile.ProfileString;if(!String.Equals(resolved,requested,StringComparison.OrdinalIgnoreCase))throw new Exception(id+": resolved profile `"+resolved+"` differs from requested `"+requested+"`");parts[id]=selected;profileById[id]=resolved;return selected; // exact authored hex prism = truthful native ContourPlate; NOT a parametric fallback (proven editable in the live 2025/2026 gate)
};
Func<string,Part,Point,Vector,double,double,BooleanPart> insertCircularOpening = (id,target,center,axis,diameter,depth) => {
    var n=unit(axis);var a=move(center,n,-depth/2-1);var b=move(center,n,depth/2+1);var tool=new Beam(a,b);tool.Profile.ProfileString="ROD"+mm(diameter);tool.Class=BooleanPart.BooleanOperativeClassName;tool.Name="AWARE OPENING TOOL";labelBeforeInsert(tool);if(!tool.Insert())throw new Exception(id+": circular operative tool insert failed");tagTransientAfterInsert(tool,id+":tool");var cut=new BooleanPart{Father=target,Type=BooleanPart.BooleanTypeEnum.BOOLEAN_CUT};cut.SetOperativePart(tool);labelBeforeInsert(cut);if(!cut.Insert())throw new Exception(id+": BooleanPart opening insert failed");tagAfterInsert(cut,id);if(!tool.Delete())throw new Exception(id+": transient opening tool deletion failed");staged.Remove(tool);nativeById[id]=cut;return cut;
};

var scenePlaneHandler=m.GetWorkPlaneHandler();
var previousScenePlane=scenePlaneHandler.GetCurrentTransformationPlane();
bool scenePlaneChanged=false;
try {
    if(!scenePlaneHandler.SetCurrentTransformationPlane(new TransformationPlane()))throw new Exception("failed to set the global scene work plane");
    scenePlaneChanged=true;
    // Ownership tags select only the retirement set. They are never proof that
    // geometry still matches: every retry rebuilds and verifies a fresh staged set.
    var oldOwned=new List<ModelObject>();var all=m.GetModelObjectSelector().GetAllObjects();while(all.MoveNext()){var o=all.Current as ModelObject;if(o==null)continue;string src="",recordId="",priorSceneHash="",owner="";if(o.GetUserProperty("USER_FIELD_1",ref src)&&(src==sourceKey||src==sourceId)&&o.GetUserProperty("USER_FIELD_2",ref recordId)&&!String.IsNullOrWhiteSpace(recordId)&&o.GetUserProperty("USER_FIELD_3",ref priorSceneHash)&&!String.IsNullOrWhiteSpace(priorSceneHash)&&o.GetUserProperty("USER_FIELD_4",ref owner)&&owner.Length==78&&owner.StartsWith("AWARE_BAKE_V1:",StringComparison.Ordinal)&&owner.Skip(14).All(c=>(c>='0'&&c<='9')||(c>='a'&&c<='f')))oldOwned.Add(o);}

    // Physical parts. Children realized by a BoltArray are intentionally not duplicated.
    foreach(var pair in elementById){string id=pair.Key;activeId=id;var el=pair.Value;string kind=str(el,"kind").ToLowerInvariant();if(String.IsNullOrEmpty(kind))kind="member";if(realizedChildren.ContainsKey(id))continue;
        if(kind=="member"||kind=="line"||kind=="box"){var from=list(el,"from");var to=list(el,"to");if(!vec3ok(from)||!vec3ok(to))throw new Exception(id+": member requires finite from/to");string profile=str(el,"profile");var em=obj(el,"meta");if(String.IsNullOrWhiteSpace(profile)&&em!=null)profile=str(em,"profile");if(String.IsNullOrWhiteSpace(profile))throw new Exception(id+": member requires an exact requested profile; fallback profiles are forbidden");insertBeam(id,"member",profile,point(from),point(to),el);}
        else if(kind=="plate")insertPlate(el,id);
        else if(kind=="rod"||kind=="bolt-shank"){var axis=obj(el,"axis");var from=list(axis,"from");var to=list(axis,"to");double d=el.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(!vec3ok(from)||!vec3ok(to)||!(d>0))throw new Exception(id+": invalid rod geometry");string requested=str(el,"profile");if(String.IsNullOrWhiteSpace(requested))requested="ROD"+mm(d)+"|D"+mm(d);insertBeam(id,kind,requested,point(from),point(to),el);}
        else if(kind=="washer"){var center=list(el,"center");var axis=list(el,"axis");double od=el.TryGetValue("outerDiameterMm",out var ov)?number(ov):Double.NaN;double inn=el.TryGetValue("innerDiameterMm",out var iv)?number(iv):Double.NaN;double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;if(!vec3ok(center)||!vec3ok(axis)||!(od>inn&&inn>0&&th>0))throw new Exception(id+": invalid washer geometry");var n=unit(vector(axis));string requested=str(el,"profile");if(String.IsNullOrWhiteSpace(requested)){requested="PD"+mm(od)+"*"+mm((od-inn)/2);} // exact annular profile = truthful native geometry; NOT a parametric fallback (proven in the live gate)insertBeam(id,kind,requested,move(point(center),n,-th/2),move(point(center),n,th/2),el);}
        else if(kind=="nut"||kind=="bolt-head")insertHex(el,id,kind);
    }

    // Independent plate holes become source-owned native BooleanPart openings.
    foreach(var pair in elementById)if(str(pair.Value,"kind")=="plate"){var plate=parts[pair.Key];var frame=obj(pair.Value,"frame");var origin=point(list(frame,"origin"));var u=unit(vector(list(frame,"uDir")));var v=unit(vector(list(frame,"vDir")));var n=unit(vector(list(frame,"normal")));double th=number(pair.Value["thicknessMm"]);var holes=list(pair.Value,"holes");if(holes==null)continue;foreach(var hr in holes){var h=hr as IDictionary<string,object>;string hid=str(h,"id");activeId=hid;if(realizedEffects.ContainsKey(hid))continue;var uv=list(h,"center");double dia=h.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(uv==null||uv.Count<2||!(dia>0))throw new Exception(hid+": invalid plate hole");insertCircularOpening(hid,plate,move(move(origin,u,number(uv[0])),v,number(uv[1])),n,dia,th);}}

    // Native relationships after all participant parts exist.
    foreach(var pair in operationById){string id=pair.Key;activeId=id;var op=pair.Value;string kind=str(op,"kind");
        if(kind=="bolt-array"){
            string a=str(op,"partToBoltTo");string b=str(op,"partToBeBolted");
            if(!parts.ContainsKey(a)||!parts.ContainsKey(b))throw new Exception(id+": bolt participants do not resolve to physical parts");
            var frame=obj(op,"frame");var origin=list(frame,"origin");var u=list(frame,"uDir");var v=list(frame,"vDir");
            var us=list(op,"uOffsetsMm").Cast<object>().Select(number).OrderBy(x=>x).ToList();
            var vs=list(op,"vOffsetsMm").Cast<object>().Select(number).OrderBy(x=>x).ToList();
            var firstInstance=list(op,"instances")[0] as IDictionary<string,object>;
            int nutSlots=list(firstInstance,"nutIds").Count,washerSlots=list(firstInstance,"washerIds").Count;
            double dia=op.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;double tol=op.TryGetValue("toleranceMm",out var tv)?number(tv):0;
            var uu=unit(vector(u));var vv=unit(vector(v));var handler=m.GetWorkPlaneHandler();var previous=handler.GetCurrentTransformationPlane();
            try{
                if(!handler.SetCurrentTransformationPlane(new TransformationPlane(point(origin),uu,vv)))throw new Exception(id+": failed to set the authored bolt work plane");
                double u0=us[0],v0=(vs[0]+vs[vs.Count-1])/2;
                var ba=new BoltArray{PartToBoltTo=parts[a],PartToBeBolted=parts[b],FirstPosition=new Point(u0,v0,0),SecondPosition=new Point(u0+100,v0,0),BoltSize=dia,BoltStandard=str(op,"standard"),Tolerance=tol,CutLength=1000,ExtraLength=0,BoltType=str(op,"boltType")=="site"?BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE:BoltGroup.BoltTypeEnum.BOLT_TYPE_WORKSHOP,ThreadInMaterial=op.TryGetValue("threadInMaterial",out var tm)&&Convert.ToBoolean(tm)?BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_YES:BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_NO,Bolt=true,Hole1=true,Hole2=true,Nut1=nutSlots>=1,Nut2=nutSlots>=2,Washer1=washerSlots>=1,Washer2=washerSlots>=2,Washer3=washerSlots>=3};
                var comp=obj(op,"components");if(comp!=null)ba.Bolt=!comp.TryGetValue("bolt",out var cv)||Convert.ToBoolean(cv);
                ba.AddBoltDistX(0);for(int i=1;i<us.Count;i++)ba.AddBoltDistX(us[i]-us[i-1]);ba.AddBoltDistY(0);for(int i=1;i<vs.Count;i++)ba.AddBoltDistY(vs[i]-vs[i-1]);
                labelBeforeInsert(ba);if(!ba.Insert())throw new Exception(id+": native BoltArray insert failed for exact standard and diameter");
                tagAfterInsert(ba,id);var selected=m.SelectModelObject(new Identifier(ba.Identifier.GUID)) as BoltArray;if(selected==null)throw new Exception(id+": native BoltArray GUID read-back failed");
                var actual=selected.BoltPositions.Cast<Point>().ToList();var expected=new List<Point>();foreach(var x in us)foreach(var y in vs)expected.Add(new Point(x,y,0));
                if(actual.Count!=expected.Count||expected.Any(p=>!actual.Any(q=>Distance.PointToPoint(p,q)<=0.1)))throw new Exception(id+": BoltPositions read-back differs from authored instances; actual="+String.Join("|",actual.Select(p=>p.X+","+p.Y+","+p.Z))+" expected="+String.Join("|",expected.Select(p=>p.X+","+p.Y+","+p.Z)));
                var expectedBoltType=str(op,"boltType")=="site"?BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE:BoltGroup.BoltTypeEnum.BOLT_TYPE_WORKSHOP;var expectedThread=op.TryGetValue("threadInMaterial",out var threadValue)&&Convert.ToBoolean(threadValue)?BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_YES:BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_NO;
                if(Math.Abs(selected.BoltSize-dia)>0.1||!String.Equals(selected.BoltStandard,str(op,"standard"),StringComparison.OrdinalIgnoreCase)||Math.Abs(selected.Tolerance-tol)>0.1||selected.BoltType!=expectedBoltType||selected.ThreadInMaterial!=expectedThread||!selected.Bolt||selected.Nut1!=(nutSlots>=1)||selected.Nut2!=(nutSlots>=2)||selected.Washer1!=(washerSlots>=1)||selected.Washer2!=(washerSlots>=2)||selected.Washer3!=(washerSlots>=3)||selected.PartToBoltTo==null||selected.PartToBeBolted==null||selected.PartToBoltTo.Identifier.GUID!=parts[a].Identifier.GUID||selected.PartToBeBolted.Identifier.GUID!=parts[b].Identifier.GUID)throw new Exception(id+": native BoltArray parameter/participant read-back differs from the authored bolt contract");
                nativeById[id]=selected;
            }finally{if(!handler.SetCurrentTransformationPlane(previous))throw new Exception(id+": failed to restore the prior work plane");}
        }
        else if(kind=="weld"){string a=str(op,"mainId");string b=str(op,"secondaryId");if(!parts.ContainsKey(a)||!parts.ContainsKey(b))throw new Exception(id+": weld participants do not resolve to parts");var path=list(op,"path");double size=op.TryGetValue("sizeMm",out var sv2)?number(sv2):Double.NaN;if(path==null||path.Count<2||!(size>0))throw new Exception(id+": fillet weld requires a path and positive size");var poly=new Polygon();foreach(var pr in path){var p=pr as IList;if(!vec3ok(p))throw new Exception(id+": invalid weld path point");poly.Points.Add(point(p));}var w=new PolygonWeld{MainObject=parts[a],SecondaryObject=parts[b],Polygon=poly,TypeAbove=BaseWeld.WeldTypeEnum.WELD_TYPE_FILLET,TypeBelow=BaseWeld.WeldTypeEnum.WELD_TYPE_NONE,SizeAbove=size,AroundWeld=(bool)op["around"],ShopWeld=(bool)op["shop"]};labelBeforeInsert(w);if(!w.Insert())throw new Exception(id+": native PolygonWeld insert failed");tagAfterInsert(w,id);nativeById[id]=w;}
        else if(kind=="boolean-cut"){string target=str(op,"targetId");var tool=obj(op,"tool");string tkind=str(tool,"kind");if(!parts.ContainsKey(target)||tool==null)throw new Exception(id+": boolean-cut needs a resolved target and tool");Part cutter;
            if(tkind=="cylinder"){var axis=obj(tool,"axis");var from=list(axis,"from");var to=list(axis,"to");double dia=tool.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(!vec3ok(from)||!vec3ok(to)||!(dia>0))throw new Exception(id+": invalid finite cylindrical boolean tool");var beam=new Beam(point(from),point(to));beam.Profile.ProfileString="ROD"+mm(dia);cutter=beam;}
            else if(tkind=="box"){
                // A finite oriented rectangular prism (a cope). The operative is a ContourPlate in the frame's
                // u-v plane, thickness 2*hn along the normal (Depth=MIDDLE straddles the frame), with a rounding
                // chamfer on the single re-entrant corner so the true coped (filleted) shape is removed, never a
                // square over-cut. Frame legality is proven by the producer (validateBoxTool) and re-checked here.
                var frame=obj(tool,"frame");var he=list(tool,"halfExtents");if(frame==null||he==null||he.Count!=3)throw new Exception(id+": invalid box boolean tool");
                var origin=point(list(frame,"origin"));var u=unit(vector(list(frame,"uDir")));var v=unit(vector(list(frame,"vDir")));
                double hu=number(he[0]),hv=number(he[1]),hn=number(he[2]);if(!(hu>0&&hv>0&&hn>0))throw new Exception(id+": box halfExtents must be positive");
                var reentrant=obj(tool,"reentrant");double rr=0;int vsign=0;if(reentrant!=null){rr=reentrant.TryGetValue("radiusMm",out var rv)?number(rv):0;vsign=(int)Math.Round(number(reentrant["vSign"]));}
                var cp=new ContourPlate();cp.Profile.ProfileString="PL"+mm(2*hn);cp.Position.Depth=Position.DepthEnum.MIDDLE;
                int[] su={-1,1,1,-1};int[] sv={-1,-1,1,1};
                for(int i=0;i<4;i++){var q=move(move(origin,u,su[i]*hu),v,sv[i]*hv);var ch=new Chamfer();if(rr>0&&vsign!=0&&su[i]>0&&sv[i]==vsign){ch.Type=Chamfer.ChamferTypeEnum.CHAMFER_ROUNDING;ch.X=rr;ch.Y=rr;}cp.AddContourPoint(new ContourPoint(q,ch));}
                cutter=cp;
            }
            else throw new Exception(id+": unsupported boolean tool kind '"+tkind+"'");
            cutter.Class=BooleanPart.BooleanOperativeClassName;cutter.Name="AWARE BOOLEAN TOOL";labelBeforeInsert(cutter);if(!cutter.Insert())throw new Exception(id+": Boolean operative tool insert failed");tagTransientAfterInsert(cutter,id+":tool");var cut=new BooleanPart{Father=parts[target],Type=BooleanPart.BooleanTypeEnum.BOOLEAN_CUT};cut.SetOperativePart(cutter);labelBeforeInsert(cut);if(!cut.Insert())throw new Exception(id+": native BooleanPart insert failed");tagAfterInsert(cut,id);if(!cutter.Delete())throw new Exception(id+": transient Boolean tool deletion failed");staged.Remove(cutter);nativeById[id]=cut;}
    }

    // Structural grids are inserted without modifying similarly named user grids.
    foreach(var pair in referenceById){string id=pair.Key;activeId=id;var rf=pair.Value;var origin=list(rf,"origin");var axes=list(rf,"axes");var levels=list(rf,"levels");var bounds=obj(rf,"bounds");var xs=new List<Tuple<double,string,string>>();var ys=new List<Tuple<double,string,string>>();foreach(var ar in axes){var a=ar as IDictionary<string,object>;double off=number(a["offsetMm"]);(str(a,"direction")=="x"?xs:ys).Add(Tuple.Create(off,str(a,"label"),str(a,"id")));}var zs=new List<Tuple<double,string,string>>();if(levels!=null)foreach(var lr in levels){var l=lr as IDictionary<string,object>;zs.Add(Tuple.Create(number(l["elevationMm"]),str(l,"label"),str(l,"id")));}xs.Sort((a,b)=>a.Item1.CompareTo(b.Item1));ys.Sort((a,b)=>a.Item1.CompareTo(b.Item1));zs.Sort((a,b)=>a.Item1.CompareTo(b.Item1));if(xs.Count==0||ys.Count==0)throw new Exception(id+": native Grid requires both X and Y axis families");double minX=number(bounds["minX"]),maxX=number(bounds["maxX"]),minY=number(bounds["minY"]),maxY=number(bounds["maxY"]),oz=number(origin[2]);string cx=spacing(xs.Select(x=>x.Item1)),cy=spacing(ys.Select(x=>x.Item1)),cz=spacing(zs.Select(x=>x.Item1-oz));var g=new Grid{Name=String.IsNullOrWhiteSpace(str(rf,"name"))?id:str(rf,"name"),Origin=point(origin),CoordinateX=cx,CoordinateY=cy,CoordinateZ=cz,LabelX=String.Join(" ",xs.Select(x=>x.Item2)),LabelY=String.Join(" ",ys.Select(x=>x.Item2)),LabelZ=String.Join(" ",zs.Select(x=>x.Item2)),ExtensionLeftX=Math.Max(0,ys.First().Item1-minY),ExtensionRightX=Math.Max(0,maxY-ys.Last().Item1),ExtensionLeftY=Math.Max(0,xs.First().Item1-minX),ExtensionRightY=Math.Max(0,maxX-xs.Last().Item1),ExtensionLeftZ=0,ExtensionRightZ=0,IsMagnetic=rf.TryGetValue("isMagnetic",out var im)&&Convert.ToBoolean(im)};labelBeforeInsert(g);if(!g.Insert())throw new Exception(id+": native Grid insert failed");tagAfterInsert(g,id);var rb=m.SelectModelObject(new Identifier(g.Identifier.GUID)) as Grid;if(rb==null||rb.CoordinateX!=cx||rb.CoordinateY!=cy||rb.CoordinateZ!=cz||rb.LabelX!=g.LabelX||rb.LabelY!=g.LabelY||rb.LabelZ!=g.LabelZ)throw new Exception(id+": Grid coordinate/label read-back differs from the authored grid");foreach(var x in xs)nativeById[x.Item3]=g;foreach(var y in ys)nativeById[y.Item3]=g;foreach(var z in zs)nativeById[z.Item3]=g;}

    // GUID/tag read-back proves the staged set before replacing any old set.
    foreach(var item in nativeRecordIdByGuid){var rb=m.SelectModelObject(new Identifier(item.Key));if(rb==null)throw new Exception(item.Value+": authoritative GUID read-back failed");string src="";string sid="";string scn="";string owner="";rb.GetUserProperty("USER_FIELD_1",ref src);rb.GetUserProperty("USER_FIELD_2",ref sid);rb.GetUserProperty("USER_FIELD_3",ref scn);rb.GetUserProperty("USER_FIELD_4",ref owner);if(src!=sourceKey||sid!=recordKey(item.Value)||scn!=sceneKey||owner!=ownershipMarker)throw new Exception(item.Value+": ownership read-back failed");}
    // Retire relationships before their participant parts. Tekla cascades a part
    // deletion to its welds/bolt groups/cuts; treating that cascade as a later
    // relationship-delete failure would leave the replacement state uncertain.
    var retirementOrder=oldOwned.OrderBy(old=>old is BaseWeld||old is BoltGroup||old is BooleanPart?0:old is Part?2:1).ToList();
    foreach(var old in retirementOrder){retirementStarted=true;if(!old.Delete()){var still=m.SelectModelObject(new Identifier(old.Identifier.GUID));if(still!=null)throw new Exception("failed to delete prior source-owned object "+old.Identifier.GUID);}}
    foreach(var old in retirementOrder)if(m.SelectModelObject(new Identifier(old.Identifier.GUID))!=null)throw new Exception("prior source-owned object remains after retirement "+old.Identifier.GUID);
    if(!m.CommitChanges("AWARE bake-scene "+sceneName+" "+attemptId))throw new Exception("Tekla CommitChanges returned false");
    if(scenePlaneChanged){if(!scenePlaneHandler.SetCurrentTransformationPlane(previousScenePlane))throw new Exception("failed to restore the user's prior work plane after the committed bake");scenePlaneChanged=false;}

    foreach(var item in supportedOrder){string id=item.Item1;string kind=item.Item2;ModelObject o=null;string realizedBy="";if(nativeById.TryGetValue(id,out o)){}else if(realizedChildren.TryGetValue(id,out realizedBy)||realizedEffects.TryGetValue(id,out realizedBy)||realizedReferences.TryGetValue(id,out realizedBy))nativeById.TryGetValue(realizedBy,out o);if(o==null)throw new Exception(id+": supported record was not classified by the materializer");var r=row(id,kind,"emitted","","");r["nativeGuid"]=o.Identifier.GUID.ToString();if(!String.IsNullOrEmpty(realizedBy))r["realizedBy"]=realizedBy;if(profileById.ContainsKey(id))r["profile"]=profileById[id];emitted.Add(r);}
    Func<IDictionary<string,object>,string> legacyMemberRole=e=>{string role=str(e,"role").ToLowerInvariant();if(String.IsNullOrEmpty(role))role=str(e,"group").ToLowerInvariant();return role;};var legacyMembers=elementById.Values.Where(e=>{string kind=str(e,"kind").ToLowerInvariant();return String.IsNullOrEmpty(kind)||kind=="member"||kind=="line"||kind=="box";}).ToList();int columns=legacyMembers.Count(e=>legacyMemberRole(e)=="column"),beams=legacyMembers.Count(e=>legacyMemberRole(e)!="column"&&legacyMemberRole(e)!="brace");string modelName;try{modelName=m.GetInfo().ModelName;}catch{modelName="Tekla model";}
    int nativeCount=nativeRecordIdByGuid.Count;
    return new { ok=true,recovered=false,sourceId,sceneHash,materializationHash,attemptId,scene_name=sceneName,model=modelName,created=nativeCount,columns,beams,native=nativeCount,placeholder=0,failed_count=0,skipped=unsupported.Count,profiles=profileById,emitted,failed=new object[0],unsupported,warnings };
}
catch(Exception ex){
    // Tekla CommitChanges creates an undo step; it is not an ACID transaction.
    // Before retirement begins, cleanup is safe because the prior set was never
    // touched. Once old deletion starts, another commit could make those deletes
    // durable, so deliberately do not issue a cleanup commit and report uncertainty.
    string failureMessage=ex.Message;bool workPlaneRestored=!scenePlaneChanged;
    if(scenePlaneChanged){try{workPlaneRestored=scenePlaneHandler.SetCurrentTransformationPlane(previousScenePlane);if(workPlaneRestored)scenePlaneChanged=false;}catch{workPlaneRestored=false;}if(!workPlaneRestored)failureMessage+=" The user's prior work plane could not be restored.";}
    bool cleanupDeleted=!retirementStarted,cleanupCommitted=false;
    if(!retirementStarted){for(int i=staged.Count-1;i>=0;i--){try{if(!staged[i].Delete())cleanupDeleted=false;}catch{cleanupDeleted=false;}}if(cleanupDeleted)try{cleanupCommitted=m.CommitChanges("AWARE bake-scene cleanup "+attemptId);}catch{}}
    bool uncertain=retirementStarted||!cleanupDeleted||!cleanupCommitted||!workPlaneRestored;
    string causeCode=uncertain?"commit-state-uncertain":"materialization-failed";
    string causeMessage=uncertain?failureMessage+" Tekla has no rollback API and the bridge could not prove complete, durably committed cleanup and work-plane restoration. Re-run bake-scene to reconcile source ownership before editing the model.":failureMessage;
    if(uncertain)warnings.Add(row("scene","scene","failed","commit-state-uncertain","Complete cleanup and a durable cleanup commit could not both be proved."));
    failed.Clear();bool causeAssigned=false;foreach(var item in supportedOrder){bool cause=!causeAssigned&&item.Item1==activeId;var r=row(item.Item1,item.Item2,"failed",cause?causeCode:"batch-aborted",cause?causeMessage:"Batch was aborted after another supported record failed.");r["rolledBack"]=!uncertain&&cleanupCommitted;failed.Add(r);if(cause)causeAssigned=true;}if(!causeAssigned){var r=row(activeId,"scene","failed",causeCode,causeMessage);r["rolledBack"]=!uncertain&&cleanupCommitted;failed.Insert(0,r);}
    return new { ok=false,sourceId,sceneHash,materializationHash,attemptId,emitted=new object[0],failed,unsupported,warnings };
}
finally {
    if(scenePlaneChanged)try{scenePlaneHandler.SetCurrentTransformationPlane(previousScenePlane);}catch{}
}
""";
}
