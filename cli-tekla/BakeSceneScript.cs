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
using Tekla.Structures.Solid;

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
Func<object,bool> jsonNumber = o => o is double || o is float || o is decimal || o is long || o is int || o is short || o is byte || o is ulong || o is uint || o is ushort || o is sbyte;
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
object sceneUpValue = null;
bool sceneUpPresent = meta != null && meta.TryGetValue("up", out sceneUpValue);
string sourceId = meta == null ? "" : str(meta,"sourceId").Trim();
string sceneHash = meta == null ? "" : str(meta,"sceneHash").Trim();
string materializationHash = args != null && args.TryGetValue("materializationHash", out var mh) ? text(mh) : "";
string resolvedHostVersion = args != null && args.TryGetValue("resolvedHostVersion", out var rhv) && rhv is string ? (string)rhv : "";
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
var gridWarningJournal = new AwareTekla.TeklaGridWarningJournal();
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
if (!AwareTekla.TeklaSceneInputContract.SceneUpIsAbsentOrExactZ(sceneUpPresent, sceneUpValue))
    failed.Add(row("scene","scene","failed","unsupported-scene-up","Tekla bake-scene accepts only absent or `z` meta.up; Y-up export needs an explicit reviewed transform"));
if (String.IsNullOrWhiteSpace(sourceId)) failed.Add(row("scene","scene","failed","missing-source-id","scene.meta.sourceId is required"));
if (String.IsNullOrWhiteSpace(sceneHash)) failed.Add(row("scene","scene","failed","missing-scene-hash","scene.meta.sceneHash is required"));

var elementById = new Dictionary<string,IDictionary<string,object>>(StringComparer.Ordinal);
foreach (var raw in elements) {
    var el = raw as IDictionary<string,object>;
    if (el == null) { failed.Add(row("scene","element","failed","invalid-record","element must be an object")); continue; }
    string id=str(el,"id"), kind=str(el,"kind").ToLowerInvariant(); if (String.IsNullOrEmpty(kind)) kind="member";
    // A rejected id withholds this element's OWN row, never its holes: those are
    // records with ids of their own, and skipping them drops them from the receipt
    // and from the scene-wide id set, so a duplicate among them stays hidden until
    // the parent is fixed (#327).
    var accepted = acceptId(id,kind);
    if (accepted) {
    if (kind=="member" || kind=="line" || kind=="box" || kind=="plate" || kind=="rod" || kind=="bolt-shank" || kind=="washer" || kind=="nut" || kind=="bolt-head") { elementById[id]=el; supportedOrder.Add(Tuple.Create(id,kind)); }
    else unsupported.Add(row(id,kind,"unsupported","unsupported-kind","element kind is not supported by the Tekla sink"));
    }
    if (kind=="plate") {
        var holes=list(el,"holes"); if(el.ContainsKey("holes")&&holes==null)failed.Add(row(id,"opening","failed","invalid-collection","plate holes must be an array"));else if (holes!=null) foreach(var hr in holes) { var h=hr as IDictionary<string,object>; if(h==null){failed.Add(row(id,"opening","failed","invalid-record","plate hole must be an object"));continue;} string hid=str(h,"id"); if(acceptId(hid,"opening")) supportedOrder.Add(Tuple.Create(hid,"opening")); }
    }
}
var operationById = new Dictionary<string,IDictionary<string,object>>(StringComparer.Ordinal);
foreach (var raw in operations) {
    var op=raw as IDictionary<string,object>; if(op==null){failed.Add(row("scene","operation","failed","invalid-record","operation must be an object"));continue;}
    string id=str(op,"id"), kind=str(op,"kind").ToLowerInvariant();
    // Withhold only this operation's own row; still walk its instances (#327).
    if(acceptId(id,kind)) {
    if(kind=="bolt-array" || kind=="weld" || kind=="boolean-cut") { operationById[id]=op; supportedOrder.Add(Tuple.Create(id,kind)); } else unsupported.Add(row(id,kind,"unsupported","unsupported-operation","operation kind is not supported by the Tekla sink"));
    }
    if(kind=="bolt-array") { var ins=list(op,"instances"); if(op.ContainsKey("instances")&&ins==null)failed.Add(row(id,"bolt-instance","failed","invalid-collection","bolt instances must be an array"));else if(ins!=null) foreach(var ir in ins) { var inst=ir as IDictionary<string,object>; if(inst==null){failed.Add(row(id,"bolt-instance","failed","invalid-record","bolt instance must be an object"));continue;} var iid=str(inst,"id"); if(acceptId(iid,"bolt-instance")) supportedOrder.Add(Tuple.Create(iid,"bolt-instance")); var effects=list(inst,"holeEffects"); if(inst.ContainsKey("holeEffects")&&effects==null)failed.Add(row(iid,"opening","failed","invalid-collection","hole effects must be an array"));else if(effects!=null) foreach(var er in effects){var effect=er as IDictionary<string,object>; if(effect==null){failed.Add(row(iid,"opening","failed","invalid-record","hole effect must be an object"));continue;} var eid=str(effect,"id"); if(acceptId(eid,"opening")) supportedOrder.Add(Tuple.Create(eid,"opening"));} } }
}
var referenceById = new Dictionary<string,IDictionary<string,object>>(StringComparer.Ordinal);
foreach(var raw in references){var rf=raw as IDictionary<string,object>;if(rf==null){failed.Add(row("scene","reference-system","failed","invalid-record","reference system must be an object"));continue;}string id=str(rf,"id"),kind=str(rf,"kind").ToLowerInvariant();bool supported=kind=="structural-grid";
// Withhold only this system's own row; still walk its axes and levels (#327).
if(acceptId(id,kind)){if(supported){referenceById[id]=rf;supportedOrder.Add(Tuple.Create(id,kind));}else unsupported.Add(row(id,kind,"unsupported","unsupported-reference-system","reference system kind is not supported by the Tekla sink"));}var axes=list(rf,"axes");if(rf.ContainsKey("axes")&&axes==null)failed.Add(row(id,"grid-axis","failed","invalid-collection","grid axes must be an array"));else if(axes!=null)foreach(var ar in axes){var axis=ar as IDictionary<string,object>;if(axis==null){failed.Add(row(id,"grid-axis","failed","invalid-record","grid axis must be an object"));continue;}var aid=str(axis,"id");if(acceptId(aid,"grid-axis")){if(supported)supportedOrder.Add(Tuple.Create(aid,"grid-axis"));else unsupported.Add(row(aid,"grid-axis","unsupported","unsupported-parent","parent reference system is unsupported"));}}var levels=list(rf,"levels");if(rf.ContainsKey("levels")&&levels==null)failed.Add(row(id,"grid-level","failed","invalid-collection","grid levels must be an array"));else if(levels!=null)foreach(var lr in levels){var level=lr as IDictionary<string,object>;if(level==null){failed.Add(row(id,"grid-level","failed","invalid-record","grid level must be an object"));continue;}var lid=str(level,"id");if(acceptId(lid,"grid-level")){if(supported)supportedOrder.Add(Tuple.Create(lid,"grid-level"));else unsupported.Add(row(lid,"grid-level","unsupported","unsupported-parent","parent reference system is unsupported"));}}}

Action<string> appendBatchAborted = message => {var failedIds=new HashSet<string>(failed.Select(r=>text(r["id"])),StringComparer.Ordinal);foreach(var item in supportedOrder)if(failedIds.Add(item.Item1))failed.Add(row(item.Item1,item.Item2,"failed","batch-aborted",message));};
if(failed.Count>0) {appendBatchAborted("Batch was aborted because scene identity or envelope validation failed.");return new { ok=false, sourceId, sceneHash, materializationHash, attemptId, emitted=new object[0], failed, unsupported, warnings };}

var realizedChildren = new Dictionary<string,string>(StringComparer.Ordinal);
var realizedEffects = new Dictionary<string,string>(StringComparer.Ordinal);
var realizedReferences = new Dictionary<string,string>(StringComparer.Ordinal);
foreach(var kv in operationById) if(str(kv.Value,"kind")=="bolt-array") { var ins=list(kv.Value,"instances"); if(ins==null)continue; foreach(var ir in ins){var inst=ir as IDictionary<string,object>;if(inst==null)continue;var iid=str(inst,"id");if(!String.IsNullOrEmpty(iid))realizedChildren[iid]=kv.Key;foreach(var key in new[]{"shankId","headId"}){var cid=str(inst,key);if(!String.IsNullOrEmpty(cid))realizedChildren[cid]=kv.Key;}foreach(var key in new[]{"nutIds","washerIds"}){var ids=list(inst,key);if(ids!=null)foreach(var x in ids){var cid=text(x);if(!String.IsNullOrEmpty(cid))realizedChildren[cid]=kv.Key;}}var effects=list(inst,"holeEffects");if(effects!=null)foreach(var er in effects){var effect=er as IDictionary<string,object>;if(effect!=null)realizedEffects[str(effect,"id")]=kv.Key;}}}

// Complete trust-boundary validation before the first Tekla Insert. Native API
// failures remain possible, but malformed geometry/relationships never start a batch.
var memberRollPlans = new Dictionary<string,AwareTekla.TeklaMemberRollPlan>(StringComparer.Ordinal);
var doubleAnglePlans = new Dictionary<string,AwareTekla.TeklaDoubleAnglePlan>(StringComparer.Ordinal);
var doubleAngleLegGuids = new Dictionary<string,List<string>>(StringComparer.Ordinal);
foreach(var pair in elementById) try {var id=pair.Key;var el=pair.Value;var kind=str(el,"kind").ToLowerInvariant();if(String.IsNullOrEmpty(kind))kind="member";
    bool rollKind=kind=="member"||kind=="line"||kind=="box";
    if(!rollKind&&el.ContainsKey("rot"))throw new Exception("rot is applicable only to physical member, line, and box records");
    if(rollKind){var xs=obj(el,"xsection");string xshape=str(xs,"shape");if(xshape=="tee")throw new Exception("xsection "+xshape+" is explicitly unsupported by the Tekla sink; supply an exact native profile after verifying its catalog geometry");var from=list(el,"from");var to=list(el,"to");var em=obj(el,"meta");string profile=str(el,"profile");if(String.IsNullOrWhiteSpace(profile)&&em!=null)profile=str(em,"profile");if(!vec3ok(from)||!vec3ok(to)||Distance.PointToPoint(point(from),point(to))<=1e-6||String.IsNullOrWhiteSpace(profile))throw new Exception("member requires a nonzero finite axis and exact profile");object rv;double roll=0;if(el.TryGetValue("rot",out rv)){if(!jsonNumber(rv)||!finite(number(rv)))throw new Exception("member rot must be a finite JSON number");roll=number(rv);}var fromArr=new[]{number(from[0]),number(from[1]),number(from[2])};var toArr=new[]{number(to[0]),number(to[1]),number(to[2])};memberRollPlans[id]=memberRoll.CreatePlan(fromArr,toArr,roll);if(xshape=="double-angle"){object dav,bav,tav,gav;var anglePlan=AwareTekla.TeklaDoubleAngleContract.CreatePlan(number(xs.TryGetValue("d",out dav)?dav:null),number(xs.TryGetValue("b",out bav)?bav:null),number(xs.TryGetValue("t",out tav)?tav:null),number(xs.TryGetValue("gap",out gav)?gav:null),str(xs,"orientation"),roll,AwareTekla.TeklaMemberRollContract.UsesVerticalSeedFrame(fromArr,toArr));var authoredSection=obj(el,"section");if(authoredSection!=null){object sw,sd;double authoredWidth=number(authoredSection.TryGetValue("w",out sw)?sw:null),authoredDepth=number(authoredSection.TryGetValue("d",out sd)?sd:null);if(finite(authoredWidth)&&finite(authoredDepth)&&(Math.Abs(authoredWidth-anglePlan.EnvelopeWidthMm)>0.1||Math.Abs(authoredDepth-anglePlan.EnvelopeDepthMm)>0.1))throw new Exception("double-angle envelope "+mm(anglePlan.EnvelopeWidthMm)+"x"+mm(anglePlan.EnvelopeDepthMm)+" disagrees with the authored section "+mm(authoredWidth)+"x"+mm(authoredDepth));}doubleAnglePlans[id]=anglePlan;foreach(var leg in anglePlan.Legs)memberRollPlans[id+"#"+leg.Suffix]=memberRoll.CreatePlan(leg.ReversedAxis?toArr:fromArr,leg.ReversedAxis?fromArr:toArr,leg.CanonicalRollDegrees);}}
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
    Action<string,string> rejectDoubleAngleParticipant = (field,participantId) => {if(participantId.Length>0&&doubleAnglePlans.ContainsKey(participantId))throw new Exception("`"+field+"` cannot reference the double-angle member `"+participantId+"`: the Tekla sink materializes it as two native parts, so a native connection to it is ambiguous");};
    foreach(var field in new[]{"partToBoltTo","partToBeBolted","mainId","secondaryId","targetId"})rejectDoubleAngleParticipant(field,str(op,field));
    {var opInstances=list(op,"instances");if(opInstances!=null)foreach(var instanceRaw in opInstances){var holeEffects=list(instanceRaw as IDictionary<string,object>,"holeEffects");if(holeEffects==null)continue;foreach(var effectRaw in holeEffects)rejectDoubleAngleParticipant("holeEffects.targetId",str(effectRaw as IDictionary<string,object>,"targetId"));}}
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
        int? authoredNutSlots=null,authoredWasherSlots=null;List<string> authoredBoltParticipants=null;
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
            var effects=list(inst,"holeEffects");if(effects==null||effects.Count<2||effects.Count>5)throw new Exception(str(inst,"id")+": between two and five participant hole effects are required by Tekla BoltArray");
            var effectTargets=new HashSet<string>(StringComparer.Ordinal);
            foreach(var er in effects){var effect=er as IDictionary<string,object>;string target=str(effect,"targetId");var c=list(effect,"center");var axis=list(effect,"axis");double ed=effect.TryGetValue("diameterMm",out var ev)?number(ev):Double.NaN;bool aligned=vec3ok(axis)&&length(vector(axis))>1e-9&&Math.Abs(Math.Abs(dot(unit(vector(axis)),nn))-1)<=1e-6;if(!elementById.ContainsKey(target)||realizedChildren.ContainsKey(target)||!effectTargets.Add(target)||!vec3ok(c)||Distance.PointToPoint(point(c),instancePoint)>0.1||!aligned||Math.Abs(ed-(dia+tol))>0.1)throw new Exception(str(effect,"id")+": hole effect must match the bolt position, axis, independently materialized participant, and diameter");}
            if(!effectTargets.Contains(pa)||!effectTargets.Contains(pb))throw new Exception(str(inst,"id")+": both participant hole effects are required");
            var orderedParticipants=effects.Cast<object>().Select(er=>str(er as IDictionary<string,object>,"targetId")).ToList();if(authoredBoltParticipants==null)authoredBoltParticipants=orderedParticipants;else if(!authoredBoltParticipants.SequenceEqual(orderedParticipants,StringComparer.Ordinal))throw new Exception(id+": every bolt instance must author the same ordered participant hole effects for one native BoltArray");
        }
        if(unmatched.Count!=0)throw new Exception("bolt instances do not exhaust the authored Cartesian offsets");
        if(components.TryGetValue("bolt",out var boltEnabled)&&!Convert.ToBoolean(boltEnabled))throw new Exception("components.bolt must be true because every bolt instance authors shankId and headId physical components");
        foreach(var component in new[]{Tuple.Create("nut",authoredNutSlots.GetValueOrDefault()),Tuple.Create("washer",authoredWasherSlots.GetValueOrDefault())})if(components.TryGetValue(component.Item1,out var enabled)&&Convert.ToBoolean(enabled)!=(component.Item2>0))throw new Exception("components."+component.Item1+" must agree with the authored "+component.Item1+"Ids cardinality");
    }
    else if(kind=="weld"){string main=str(op,"mainId"),secondary=str(op,"secondaryId");if(!elementById.ContainsKey(main)||!elementById.ContainsKey(secondary)||realizedChildren.ContainsKey(main)||realizedChildren.ContainsKey(secondary)||str(op,"weldType")!="fillet")throw new Exception("fillet weld participants must be independently materialized parts and the type must be fillet");var path=list(op,"path");double size=op.TryGetValue("sizeMm",out var sv2)?number(sv2):Double.NaN;if(path==null||path.Count<2||!(size>0)||path.Cast<object>().Any(x=>!vec3ok(x as IList)))throw new Exception("fillet weld path/size is invalid");if(!op.TryGetValue("around",out var aroundValue)||!(aroundValue is bool)||!op.TryGetValue("shop",out var shopValue)||!(shopValue is bool))throw new Exception("fillet weld around and shop fields must be boolean");}
    else {string target=str(op,"targetId");if(!elementById.ContainsKey(target)||realizedChildren.ContainsKey(target))throw new Exception("boolean target must resolve to an independently materialized part");var tool=obj(op,"tool");string tkind=str(tool,"kind");if(tkind=="cylinder"){var axis=obj(tool,"axis");var from=list(axis,"from");var to=list(axis,"to");double d=tool!=null&&tool.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(!vec3ok(from)||!vec3ok(to)||Distance.PointToPoint(point(from),point(to))<=1e-6||!(d>0))throw new Exception("finite cylindrical boolean tool is invalid");}else if(tkind=="box"){var frame=obj(tool,"frame");var he=list(tool,"halfExtents");if(frame==null||!vec3ok(list(frame,"origin"))||!vec3ok(list(frame,"uDir"))||!vec3ok(list(frame,"vDir"))||!vec3ok(list(frame,"normal"))||he==null||he.Count!=3||!(number(he[0])>0)||!(number(he[1])>0)||!(number(he[2])>0))throw new Exception("finite box boolean tool is invalid");}else throw new Exception("boolean tool kind must be cylinder or box");}
}catch(Exception ex){failed.Add(row(pair.Key,str(pair.Value,"kind"),"failed","invalid-operation",ex.Message));}

var gridPlans = new Dictionary<string,AwareTekla.TeklaGridMaterializationPlan>(StringComparer.Ordinal);
var unsupportedGridEnvelopes = new Dictionary<string,AwareTekla.TeklaGridEnvelopeResult>(StringComparer.Ordinal);
var unsupportedGridRows = new Dictionary<string,IReadOnlyList<Dictionary<string,object>>>(StringComparer.Ordinal);
foreach(var pair in referenceById) try {
    var rf=pair.Value;var origin=list(rf,"origin");var axes=list(rf,"axes");var levels=list(rf,"levels");var bounds=obj(rf,"bounds");
    if(!vec3ok(origin)||axes==null||levels==null||levels.Count==0||bounds==null)throw new Exception("grid origin, axes, non-empty levels, and bounds are required");
    double minX=number(bounds["minX"]),maxX=number(bounds["maxX"]),minY=number(bounds["minY"]),maxY=number(bounds["maxY"]);
    if(!finite(minX)||!finite(maxX)||!finite(minY)||!finite(maxY)||!(maxX>minX&&maxY>minY))throw new Exception("grid bounds are invalid");
    if(rf.TryGetValue("isMagnetic",out var magneticValue)&&!(magneticValue is bool))throw new Exception("grid isMagnetic must be boolean");
    var axisContracts=new List<AwareTekla.GridAxisExtentContract>();
    foreach(var ar in axes){
        var a=ar as IDictionary<string,object>;if(a==null)throw new Exception("grid axis must be an object");
        double off=a.TryGetValue("offsetMm",out var ov)?number(ov):Double.NaN;string dir=str(a,"direction");object axisLabelValue;if(!a.TryGetValue("label",out axisLabelValue)||!(axisLabelValue is string))throw new Exception(str(a,"id")+": grid axis label must be a string");string axisLabel=(string)axisLabelValue;
        if(!finite(off)||(dir!="x"&&dir!="y")||(dir=="x"&&(off<minX||off>maxX))||(dir=="y"&&(off<minY||off>maxY)))throw new Exception(str(a,"id")+": grid axis is invalid or outside the grid bounds");
        double expectedStart=dir=="x"?minY:minX,expectedEnd=dir=="x"?maxY:maxX;
        double start=a.TryGetValue("startMm",out var sv)?number(sv):expectedStart,end=a.TryGetValue("endMm",out var ev)?number(ev):expectedEnd;
        if(!finite(start)||!finite(end)||!(end>start))throw new Exception(str(a,"id")+": grid axis start/end extents must be finite and increasing");
        axisContracts.Add(new AwareTekla.GridAxisExtentContract(str(a,"id"),dir,off,axisLabel,start,end));
    }
    var levelContracts=new List<AwareTekla.GridLevelContract>();
    foreach(var lr in levels){var l=lr as IDictionary<string,object>;if(l==null)throw new Exception("grid level must be an object");double elevation=l.TryGetValue("elevationMm",out var elevationValue)?number(elevationValue):Double.NaN;object levelLabelValue;if(!l.TryGetValue("label",out levelLabelValue)||!(levelLabelValue is string))throw new Exception(str(l,"id")+": grid level label must be a string");if(!finite(elevation))throw new Exception(str(l,"id")+": grid level elevation is invalid");levelContracts.Add(new AwareTekla.GridLevelContract(str(l,"id"),elevation,(string)levelLabelValue));}
    var envelope=gridEnvelope.Evaluate(axisContracts,levelContracts,number(origin[2]),resolvedHostVersion);
    if(envelope.IsSupported){var plan=gridEnvelope.CreatePlan(pair.Key,axisContracts,levelContracts,number(origin[2]),resolvedHostVersion);gridPlans[pair.Key]=plan;foreach(var child in plan.Children)realizedReferences[child.Id]=child.RealizedBy;var expansionWarning=plan.CreateExpansionWarning();if(expansionWarning!=null)gridWarningJournal.Queue(expansionWarning);var labelWarning=plan.CreateLabelTokenWarning();if(labelWarning!=null)gridWarningJournal.Queue(labelWarning);}
    else {unsupportedGridEnvelopes[pair.Key]=envelope;unsupportedGridRows[pair.Key]=gridEnvelope.CreateUnsupportedRows(pair.Key,axisContracts,levelContracts,envelope);}
}catch(Exception ex){failed.Add(row(pair.Key,"structural-grid","failed","invalid-grid",ex.Message));}
foreach(var item in unsupportedGridEnvelopes){var gridId=item.Key;var rows=unsupportedGridRows[gridId];unsupported.AddRange(rows);var childIds=new HashSet<string>(rows.Skip(1).Select(receipt=>text(receipt["id"])),StringComparer.Ordinal);supportedOrder.RemoveAll(orderItem=>orderItem.Item1==gridId||childIds.Contains(orderItem.Item1));referenceById.Remove(gridId);}

if(failed.Count>0){appendBatchAborted("Batch was aborted because another supported record failed preflight.");return new{ok=false,sourceId,sceneHash,materializationHash,attemptId,emitted=new object[0],failed,unsupported,warnings};}

string expectedModelPath=args!=null&&args.TryGetValue("expectedModelPath",out var expectedModelValue)?text(expectedModelValue).Trim():"";
if(!String.IsNullOrEmpty(expectedModelPath)){
    string actualModelPath=AwareTekla.TeklaSceneInputContract.CanonicalModelDirectoryPath(m.GetInfo().ModelPath);
    string canonicalExpectedPath=AwareTekla.TeklaSceneInputContract.CanonicalModelDirectoryPath(expectedModelPath);
    if(!String.Equals(actualModelPath,canonicalExpectedPath,StringComparison.OrdinalIgnoreCase)){
        failed.Add(row("scene","scene","failed","unexpected-model-path","Refusing to mutate Tekla model `"+actualModelPath+"`; expected `"+canonicalExpectedPath+"`."));
        appendBatchAborted("Batch was aborted because the connected Tekla model did not match the expected QA target.");
        return new{ok=false,sourceId,sceneHash,materializationHash,attemptId,emitted=new object[0],failed,unsupported,warnings};
    }
}

var staged = new List<ModelObject>();
bool retirementStarted = false;
var parts = new Dictionary<string,Part>(StringComparer.Ordinal);
var nativeById = new Dictionary<string,ModelObject>(StringComparer.Ordinal);
var nativeRecordIdByGuid = new Dictionary<Guid,string>();
var profileById = new Dictionary<string,string>(StringComparer.Ordinal);
var memberRollById = new Dictionary<string,double>(StringComparer.Ordinal);
var nativeRotationById = new Dictionary<string,string>(StringComparer.Ordinal);
var nativeRotationOffsetById = new Dictionary<string,double>(StringComparer.Ordinal);
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
Func<Beam,List<Point>> solidVertices = beam => {var points=new List<Point>();var faces=beam.GetSolid().GetFaceEnumerator();while(faces.MoveNext()){var face=faces.Current as Face;if(face==null)continue;var loops=face.GetLoopEnumerator();while(loops.MoveNext()){var loop=loops.Current as Loop;if(loop==null)continue;var vertices=loop.GetVertexEnumerator();while(vertices.MoveNext()){var p=vertices.Current as Point;if(p!=null&&!points.Any(q=>Distance.PointToPoint(p,q)<=0.01))points.Add(p);}}}return points;};
Func<Point,Point,Vector,double,Point> rotateAroundAxis = (p,origin,axis,degrees) => {var n=unit(axis);var v=new Vector(p.X-origin.X,p.Y-origin.Y,p.Z-origin.Z);double radians=degrees*Math.PI/180;var rotated=new Vector(v.X*Math.Cos(radians)+cross(n,v).X*Math.Sin(radians)+n.X*dot(n,v)*(1-Math.Cos(radians)),v.Y*Math.Cos(radians)+cross(n,v).Y*Math.Sin(radians)+n.Y*dot(n,v)*(1-Math.Cos(radians)),v.Z*Math.Cos(radians)+cross(n,v).Z*Math.Sin(radians)+n.Z*dot(n,v)*(1-Math.Cos(radians)));return move(origin,rotated,1);};
Func<string,string,string,string,Point,Point,IDictionary<string,object>,Beam> insertBeam = (id,planKey,kind,profiles,p1,p2,el) => {
    if(Distance.PointToPoint(p1,p2)<=1e-6)throw new Exception(planKey+": axis has zero length");
    Exception last=null;
    foreach(var profile in profiles.Split(new[]{'|'},StringSplitOptions.RemoveEmptyEntries)) { Beam b;try{b=new Beam(p1,p2);b.Profile.ProfileString=profile;decorate(b,el,kind=="member"?"MEMBER":kind.ToUpperInvariant());b.Position.Depth=Position.DepthEnum.MIDDLE;b.Position.Plane=Position.PlaneEnum.MIDDLE;AwareTekla.TeklaMemberRollPlan rollPlan;if(memberRollPlans.TryGetValue(planKey,out rollPlan)){b.Position.Rotation=Position.RotationEnum.FRONT;b.Position.RotationOffset=rollPlan.NativeFrontOffsetDegrees;}else b.Position.Rotation=Position.RotationEnum.TOP;labelBeforeInsert(b);}catch(Exception ex){last=ex;continue;}bool inserted;try{inserted=b.Insert();}catch(Exception ex){last=ex;try{b.Delete();}catch{}continue;}if(!inserted){last=new Exception("Tekla rejected profile `"+profile+"`");continue;}tagAfterInsert(b,id);var selected=m.SelectModelObject(new Identifier(b.Identifier.GUID)) as Beam;if(selected==null)throw new Exception(planKey+": authoritative Beam GUID read-back failed");string resolved=selected.Profile.ProfileString;if(!String.Equals(resolved,profile,StringComparison.OrdinalIgnoreCase))throw new Exception(planKey+": resolved profile `"+resolved+"` differs from requested `"+profile+"`");AwareTekla.TeklaMemberRollPlan expectedRoll;if(memberRollPlans.TryGetValue(planKey,out expectedRoll)){var desiredVertices=solidVertices(selected);selected.Position.Rotation=Position.RotationEnum.FRONT;selected.Position.RotationOffset=0;if(!selected.Modify())throw new Exception(planKey+": native zero-roll verification Modify failed");var zeroSelected=m.SelectModelObject(new Identifier(b.Identifier.GUID)) as Beam;if(zeroSelected==null)throw new Exception(planKey+": native zero-roll verification read-back failed");var zeroVertices=solidVertices(zeroSelected);zeroSelected.Position.Rotation=Position.RotationEnum.FRONT;zeroSelected.Position.RotationOffset=expectedRoll.NativeFrontOffsetDegrees;if(!zeroSelected.Modify())throw new Exception(planKey+": native roll restoration Modify failed");selected=m.SelectModelObject(new Identifier(b.Identifier.GUID)) as Beam;if(selected==null)throw new Exception(planKey+": native roll restoration read-back failed");double effective=AwareTekla.TeklaMemberRollContract.EffectiveNativeDegrees(selected.Position.Rotation.ToString(),selected.Position.RotationOffset);if(!AwareTekla.TeklaMemberRollContract.AnglesMatch(expectedRoll.NativeFrontOffsetDegrees,effective))throw new Exception(planKey+": native rotation read-back differs from the canonical roll plan");var restoredVertices=solidVertices(selected);var memberAxis=new Vector(p2.X-p1.X,p2.Y-p1.Y,p2.Z-p1.Z);if(zeroVertices.Count<4||restoredVertices.Count!=zeroVertices.Count||desiredVertices.Count!=zeroVertices.Count||desiredVertices.Any(source=>!restoredVertices.Any(actual=>Distance.PointToPoint(source,actual)<=0.1))||zeroVertices.Any(source=>!restoredVertices.Any(actual=>Distance.PointToPoint(rotateAroundAxis(source,p1,memberAxis,expectedRoll.NativeFrontOffsetDegrees),actual)<=0.1)))throw new Exception(planKey+": native B-rep section orientation differs from the canonical roll plan");memberRollById[planKey]=expectedRoll.NormalizedDegrees;nativeRotationById[planKey]=selected.Position.Rotation.ToString();nativeRotationOffsetById[planKey]=selected.Position.RotationOffset;}parts[id]=selected;profileById[id]=resolved;return selected;}
    throw new Exception(planKey+": no exact requested profile inserted from `"+profiles+"`"+(last==null?"":": "+last.Message));
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
    if(!vec3ok(center)||!vec3ok(av)||!(af>0)||!(th>0)||!finite(phase))throw new Exception(id+": invalid hex-prism geometry");var n=unit(vector(av));var seed=Math.Abs(n.Z)<0.9?new Vector(0,0,1):new Vector(1,0,0);var u=unit(cross(seed,n));var v=unit(cross(n,u));double radius=af/Math.Sqrt(3);string requested="PL"+mm(th);var p=new ContourPlate();p.Profile.ProfileString=requested;decorate(p,el,kind=="nut"?"NUT":"BOLT HEAD");p.Position.Depth=Position.DepthEnum.MIDDLE;for(int i=0;i<6;i++){double a=phase+i*Math.PI/3;var q=move(move(point(center),u,radius*Math.Cos(a)),v,radius*Math.Sin(a));p.AddContourPoint(new ContourPoint(q,new Chamfer()));}labelBeforeInsert(p);if(!p.Insert())throw new Exception(id+": exact hex ContourPlate insert failed");tagAfterInsert(p,id);var selected=m.SelectModelObject(new Identifier(p.Identifier.GUID)) as ContourPlate;if(selected==null)throw new Exception(id+": authoritative ContourPlate GUID read-back failed");string resolved=selected.Profile.ProfileString;if(!String.Equals(resolved,requested,StringComparison.OrdinalIgnoreCase))throw new Exception(id+": resolved profile `"+resolved+"` differs from requested `"+requested+"`");parts[id]=selected;profileById[id]=resolved;return selected;
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
        if(kind=="member"||kind=="line"||kind=="box"){var from=list(el,"from");var to=list(el,"to");if(!vec3ok(from)||!vec3ok(to))throw new Exception(id+": member requires finite from/to");string profile=str(el,"profile");var em=obj(el,"meta");if(String.IsNullOrWhiteSpace(profile)&&em!=null)profile=str(em,"profile");if(String.IsNullOrWhiteSpace(profile))throw new Exception(id+": member requires an exact requested profile; fallback profiles are forbidden");AwareTekla.TeklaDoubleAnglePlan anglePlan;if(doubleAnglePlans.TryGetValue(id,out anglePlan)){var rolledX=new Vector(memberRollPlans[id].RolledX[0],memberRollPlans[id].RolledX[1],memberRollPlans[id].RolledX[2]);var head=point(from);var tail=point(to);var legGuids=new List<string>();var placedLegs=new List<Beam>();Beam firstLeg=null;foreach(var leg in anglePlan.Legs){var legHead=move(head,rolledX,leg.OffsetMm);var legTail=move(tail,rolledX,leg.OffsetMm);var placed=insertBeam(id,id+"#"+leg.Suffix,"member",anglePlan.LegProfile,leg.ReversedAxis?legTail:legHead,leg.ReversedAxis?legHead:legTail,el);legGuids.Add(placed.Identifier.GUID.ToString());placedLegs.Add(placed);if(firstLeg==null)firstLeg=placed;}var rolledY=new Vector(memberRollPlans[id].RolledY[0],memberRollPlans[id].RolledY[1],memberRollPlans[id].RolledY[2]);var sectionSeen=new List<double[]>();foreach(var legBeam in placedLegs)foreach(var v in solidVertices(legBeam)){var d3=new Vector(v.X-head.X,v.Y-head.Y,v.Z-head.Z);var xy=new[]{dot(d3,rolledX),dot(d3,rolledY)};if(!sectionSeen.Any(q=>Math.Abs(q[0]-xy[0])<=0.1&&Math.Abs(q[1]-xy[1])<=0.1))sectionSeen.Add(xy);}var wanted=new List<double[]>();foreach(var w in anglePlan.SectionOutline)if(!wanted.Any(q=>Math.Abs(q[0]-w[0])<=0.1&&Math.Abs(q[1]-w[1])<=0.1))wanted.Add(w);if(sectionSeen.Count!=wanted.Count||wanted.Any(w=>!sectionSeen.Any(q=>Math.Abs(q[0]-w[0])<=0.1&&Math.Abs(q[1]-w[1])<=0.1))||sectionSeen.Any(q=>!wanted.Any(w=>Math.Abs(q[0]-w[0])<=0.1&&Math.Abs(q[1]-w[1])<=0.1)))throw new Exception(id+": the inserted double-angle pair does not match the canonical `"+str(obj(el,"xsection"),"orientation")+"` section; this environment's `"+anglePlan.LegProfile+"` is seated differently from the one the contract was probed against");doubleAngleLegGuids[id]=legGuids;nativeById[id]=firstLeg;parts[id]=firstLeg;profileById[id]=profile;}else insertBeam(id,id,"member",profile,point(from),point(to),el);}
        else if(kind=="plate")insertPlate(el,id);
        else if(kind=="rod"||kind=="bolt-shank"){var axis=obj(el,"axis");var from=list(axis,"from");var to=list(axis,"to");double d=el.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;if(!vec3ok(from)||!vec3ok(to)||!(d>0))throw new Exception(id+": invalid rod geometry");string requested=str(el,"profile");if(String.IsNullOrWhiteSpace(requested))requested="ROD"+mm(d)+"|D"+mm(d);insertBeam(id,id,kind,requested,point(from),point(to),el);}
        else if(kind=="washer"){var center=list(el,"center");var axis=list(el,"axis");double od=el.TryGetValue("outerDiameterMm",out var ov)?number(ov):Double.NaN;double inn=el.TryGetValue("innerDiameterMm",out var iv)?number(iv):Double.NaN;double th=el.TryGetValue("thicknessMm",out var tv)?number(tv):Double.NaN;if(!vec3ok(center)||!vec3ok(axis)||!(od>inn&&inn>0&&th>0))throw new Exception(id+": invalid washer geometry");var n=unit(vector(axis));string requested=str(el,"profile");if(String.IsNullOrWhiteSpace(requested)){requested="PD"+mm(od)+"*"+mm((od-inn)/2);}insertBeam(id,id,kind,requested,move(point(center),n,-th/2),move(point(center),n,th/2),el);}
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
            var otherParticipantIds=list(firstInstance,"holeEffects").Cast<object>().Select(er=>str(er as IDictionary<string,object>,"targetId")).Where(target=>target!=a&&target!=b).ToList();
            if(otherParticipantIds.Any(target=>!parts.ContainsKey(target)))throw new Exception(id+": additional bolt participants do not resolve to physical parts");
            int nutSlots=list(firstInstance,"nutIds").Count,washerSlots=list(firstInstance,"washerIds").Count;
            double dia=op.TryGetValue("diameterMm",out var dv)?number(dv):Double.NaN;double tol=op.TryGetValue("toleranceMm",out var tv)?number(tv):0;
            var uu=unit(vector(u));var vv=unit(vector(v));var handler=m.GetWorkPlaneHandler();var previous=handler.GetCurrentTransformationPlane();
            try{
                if(!handler.SetCurrentTransformationPlane(new TransformationPlane(point(origin),uu,vv)))throw new Exception(id+": failed to set the authored bolt work plane");
                double u0=us[0],v0=(vs[0]+vs[vs.Count-1])/2;
                var ba=new BoltArray{PartToBoltTo=parts[a],PartToBeBolted=parts[b],FirstPosition=new Point(u0,v0,0),SecondPosition=new Point(u0+100,v0,0),BoltSize=dia,BoltStandard=str(op,"standard"),Tolerance=tol,CutLength=1000,ExtraLength=0,BoltType=str(op,"boltType")=="site"?BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE:BoltGroup.BoltTypeEnum.BOLT_TYPE_WORKSHOP,ThreadInMaterial=op.TryGetValue("threadInMaterial",out var tm)&&Convert.ToBoolean(tm)?BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_YES:BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_NO,Bolt=true,Hole1=true,Hole2=true,Hole3=otherParticipantIds.Count>=1,Hole4=otherParticipantIds.Count>=2,Hole5=otherParticipantIds.Count>=3,Nut1=nutSlots>=1,Nut2=nutSlots>=2,Washer1=washerSlots>=1,Washer2=washerSlots>=2,Washer3=washerSlots>=3};
                var comp=obj(op,"components");if(comp!=null)ba.Bolt=!comp.TryGetValue("bolt",out var cv)||Convert.ToBoolean(cv);
                foreach(var participantId in otherParticipantIds)if(!ba.AddOtherPartToBolt(parts[participantId]))throw new Exception(id+": failed to bind additional native bolt participant "+participantId);
                ba.AddBoltDistX(0);for(int i=1;i<us.Count;i++)ba.AddBoltDistX(us[i]-us[i-1]);ba.AddBoltDistY(0);for(int i=1;i<vs.Count;i++)ba.AddBoltDistY(vs[i]-vs[i-1]);
                labelBeforeInsert(ba);if(!ba.Insert())throw new Exception(id+": native BoltArray insert failed for exact standard and diameter");
                tagAfterInsert(ba,id);var selected=m.SelectModelObject(new Identifier(ba.Identifier.GUID)) as BoltArray;if(selected==null)throw new Exception(id+": native BoltArray GUID read-back failed");
                var actual=selected.BoltPositions.Cast<Point>().ToList();var expected=new List<Point>();foreach(var x in us)foreach(var y in vs)expected.Add(new Point(x,y,0));
                if(actual.Count!=expected.Count||expected.Any(p=>!actual.Any(q=>Distance.PointToPoint(p,q)<=0.1)))throw new Exception(id+": BoltPositions read-back differs from authored instances; actual="+String.Join("|",actual.Select(p=>p.X+","+p.Y+","+p.Z))+" expected="+String.Join("|",expected.Select(p=>p.X+","+p.Y+","+p.Z)));
                var expectedBoltType=str(op,"boltType")=="site"?BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE:BoltGroup.BoltTypeEnum.BOLT_TYPE_WORKSHOP;var expectedThread=op.TryGetValue("threadInMaterial",out var threadValue)&&Convert.ToBoolean(threadValue)?BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_YES:BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_NO;
                var actualOtherParticipants=selected.GetOtherPartsToBolt().Cast<Part>().Select(part=>part.Identifier.GUID).ToList();var expectedOtherParticipants=otherParticipantIds.Select(participantId=>parts[participantId].Identifier.GUID).ToList();bool otherParticipantsMatch=actualOtherParticipants.Count==expectedOtherParticipants.Count&&expectedOtherParticipants.All(guid=>actualOtherParticipants.Contains(guid));
                if(Math.Abs(selected.BoltSize-dia)>0.1||!String.Equals(selected.BoltStandard,str(op,"standard"),StringComparison.OrdinalIgnoreCase)||Math.Abs(selected.Tolerance-tol)>0.1||selected.BoltType!=expectedBoltType||selected.ThreadInMaterial!=expectedThread||!selected.Bolt||!selected.Hole1||!selected.Hole2||selected.Hole3!=(otherParticipantIds.Count>=1)||selected.Hole4!=(otherParticipantIds.Count>=2)||selected.Hole5!=(otherParticipantIds.Count>=3)||selected.Nut1!=(nutSlots>=1)||selected.Nut2!=(nutSlots>=2)||selected.Washer1!=(washerSlots>=1)||selected.Washer2!=(washerSlots>=2)||selected.Washer3!=(washerSlots>=3)||selected.PartToBoltTo==null||selected.PartToBeBolted==null||selected.PartToBoltTo.Identifier.GUID!=parts[a].Identifier.GUID||selected.PartToBeBolted.Identifier.GUID!=parts[b].Identifier.GUID||!otherParticipantsMatch)throw new Exception(id+": native BoltArray parameter/participant read-back differs from the authored bolt contract");
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
    foreach(var pair in referenceById){string id=pair.Key;activeId=id;var rf=pair.Value;var origin=list(rf,"origin");var originPoint=point(origin);var plan=gridPlans[id];var envelope=plan.Envelope;bool isMagnetic=rf.TryGetValue("isMagnetic",out var im)&&Convert.ToBoolean(im);var g=new Grid{Name=String.IsNullOrWhiteSpace(str(rf,"name"))?id:str(rf,"name"),Origin=originPoint,CoordinateX=plan.CoordinateX,CoordinateY=plan.CoordinateY,CoordinateZ=plan.CoordinateZ,LabelX=plan.LabelX,LabelY=plan.LabelY,LabelZ=plan.LabelZ,ExtensionLeftX=plan.ExtensionLeftX,ExtensionRightX=plan.ExtensionRightX,ExtensionLeftY=plan.ExtensionLeftY,ExtensionRightY=plan.ExtensionRightY,ExtensionLeftZ=0,ExtensionRightZ=0,IsMagnetic=isMagnetic};labelBeforeInsert(g);if(!g.Insert())throw new Exception(id+": native Grid insert failed");tagAfterInsert(g,id);var rb=m.SelectModelObject(new Identifier(g.Identifier.GUID)) as Grid;if(rb==null)throw new Exception(id+": authoritative Grid GUID read-back failed");double rbXStart=plan.MinYOffsetMm-rb.ExtensionLeftX,rbXEnd=plan.MaxYOffsetMm+rb.ExtensionRightX,rbYStart=plan.MinXOffsetMm-rb.ExtensionLeftY,rbYEnd=plan.MaxXOffsetMm+rb.ExtensionRightY;if(Distance.PointToPoint(rb.Origin,originPoint)>0.1||rb.CoordinateX!=plan.CoordinateX||rb.CoordinateY!=plan.CoordinateY||rb.CoordinateZ!=plan.CoordinateZ||rb.LabelX!=plan.LabelX||rb.LabelY!=plan.LabelY||rb.LabelZ!=plan.LabelZ||Math.Abs(rbXStart-envelope.XFamilyStartMm)>0.1||Math.Abs(rbXEnd-envelope.XFamilyEndMm)>0.1||Math.Abs(rbYStart-envelope.YFamilyStartMm)>0.1||Math.Abs(rbYEnd-envelope.YFamilyEndMm)>0.1||Math.Abs(rb.ExtensionLeftZ)>0.1||Math.Abs(rb.ExtensionRightZ)>0.1||rb.IsMagnetic!=isMagnetic)throw new Exception(id+": Grid origin/coordinate/label/envelope/magnetism read-back differs from the authored grid");var nativePlanes=new List<GridPlane>();var childEnumerator=rb.GetChildren();while(childEnumerator.MoveNext()){var nativePlane=childEnumerator.Current as GridPlane;if(nativePlane!=null)nativePlanes.Add(nativePlane);}if(nativePlanes.Count!=plan.Children.Count)throw new Exception(id+": native Grid automatic plane count differs from the authored grid");foreach(var expectedChild in plan.Children){var matches=nativePlanes.Where(nativePlane=>{if(nativePlane.Label!=expectedChild.NativeLabel||nativePlane.Plane==null)return false;var normal=unit(cross(nativePlane.Plane.AxisX,nativePlane.Plane.AxisY));var expectedAxis=expectedChild.Family=="x"?new Vector(1,0,0):expectedChild.Family=="y"?new Vector(0,1,0):new Vector(0,0,1);double actualCoordinate=expectedChild.Family=="x"?nativePlane.Plane.Origin.X:expectedChild.Family=="y"?nativePlane.Plane.Origin.Y:nativePlane.Plane.Origin.Z;double expectedCoordinate=expectedChild.Family=="x"?originPoint.X+expectedChild.OffsetMm:expectedChild.Family=="y"?originPoint.Y+expectedChild.OffsetMm:originPoint.Z+expectedChild.OffsetMm;return Math.Abs(actualCoordinate-expectedCoordinate)<=0.1&&Math.Abs(dot(normal,expectedAxis))>=1-1e-9;}).ToList();if(matches.Count!=1)throw new Exception(expectedChild.Id+": native Grid automatic plane label/coordinate/orientation association differs from the parent Grid plan");nativePlanes.Remove(matches[0]);}if(nativePlanes.Count!=0)throw new Exception(id+": unmatched native Grid automatic plane remains after verification");}

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
    foreach(var item in supportedOrder){string id=item.Item1;string kind=item.Item2;ModelObject o=null;string realizedBy="";if(nativeById.TryGetValue(id,out o)){}else if(realizedChildren.TryGetValue(id,out realizedBy)||realizedEffects.TryGetValue(id,out realizedBy)||realizedReferences.TryGetValue(id,out realizedBy))nativeById.TryGetValue(realizedBy,out o);if(o==null)throw new Exception(id+": supported record was not classified by the materializer");var r=row(id,kind,"emitted","","");r["nativeGuid"]=o.Identifier.GUID.ToString();if(!String.IsNullOrEmpty(realizedBy))r["realizedBy"]=realizedBy;if(profileById.ContainsKey(id))r["profile"]=profileById[id];if(memberRollById.ContainsKey(id)){r["rot"]=memberRollById[id];r["nativeRotation"]=nativeRotationById[id];r["nativeRotationOffset"]=nativeRotationOffsetById[id];}AwareTekla.TeklaDoubleAnglePlan emittedAnglePlan;List<string> legGuids;AwareTekla.TeklaMemberRollPlan emittedRollPlan;IDictionary<string,object> emittedElement;if(doubleAnglePlans.TryGetValue(id,out emittedAnglePlan)){if(!doubleAngleLegGuids.TryGetValue(id,out legGuids)||!memberRollPlans.TryGetValue(id,out emittedRollPlan)||!elementById.TryGetValue(id,out emittedElement)||legGuids.Count!=emittedAnglePlan.Legs.Length)throw new Exception(id+": double-angle bookkeeping is inconsistent after commit; the pair is in the model but cannot be reported");var emittedMeta=obj(emittedElement,"meta");string requestedProfile=str(emittedElement,"profile");if(String.IsNullOrWhiteSpace(requestedProfile)&&emittedMeta!=null)requestedProfile=str(emittedMeta,"profile");r["profile"]=requestedProfile;r["legProfile"]=emittedAnglePlan.LegProfile;r["rot"]=emittedRollPlan.NormalizedDegrees;r["nativeGuids"]=legGuids;r["legs"]=emittedAnglePlan.Legs.Select((leg,index)=>{var legRow=new Dictionary<string,object>(StringComparer.Ordinal){{"suffix",leg.Suffix},{"nativeGuid",legGuids[index]},{"reversedAxis",leg.ReversedAxis},{"offsetMm",leg.OffsetMm}};string legRotation;double legRotationOffset;if(nativeRotationById.TryGetValue(id+"#"+leg.Suffix,out legRotation))legRow["nativeRotation"]=legRotation;if(nativeRotationOffsetById.TryGetValue(id+"#"+leg.Suffix,out legRotationOffset))legRow["nativeRotationOffset"]=legRotationOffset;return (object)legRow;}).ToList();gridWarningJournal.Queue(row(id,kind,"warning","tekla-double-angle-materialized-as-pair","Tekla has no native double-angle profile, so requested `"+requestedProfile+"` was materialized as two `"+emittedAnglePlan.LegProfile+"` parts "+String.Join(" and ",legGuids.ToArray())+" separated by the authored gap."));}emitted.Add(r);}
    Func<IDictionary<string,object>,string> legacyMemberRole=e=>{string role=str(e,"role").ToLowerInvariant();if(String.IsNullOrEmpty(role))role=str(e,"group").ToLowerInvariant();return role;};var legacyMembers=elementById.Values.Where(e=>{string kind=str(e,"kind").ToLowerInvariant();return String.IsNullOrEmpty(kind)||kind=="member"||kind=="line"||kind=="box";}).ToList();int columns=legacyMembers.Count(e=>legacyMemberRole(e)=="column"),beams=legacyMembers.Count(e=>legacyMemberRole(e)!="column"&&legacyMemberRole(e)!="brace");string modelName;try{modelName=m.GetInfo().ModelName;}catch{modelName="Tekla model";}
    int nativeCount=nativeRecordIdByGuid.Count;
    // Replace the "adding..." message only once the bake has actually reached its successful end.
    // Announcing at CommitChanges would be true about the objects but premature about the operation:
    // the work-plane restore and the classification below can still throw, and the caller would then
    // report a failure Tekla had already called a success — inviting a retry that duplicates parts.
    try{string doneLabel=str(args,"label");int doneMembers=emitted.Count(r=>str(r as IDictionary<string,object>,"kind")=="member");Tekla.Structures.Model.Operations.Operation.DisplayPrompt((String.IsNullOrWhiteSpace(doneLabel)?"":doneLabel+": ")+doneMembers.ToString(inv)+" member"+(doneMembers==1?"":"s")+" added.");}catch{}
    warnings.AddRange(gridWarningJournal.PublishAfterCommit());
    return new { ok=true,recovered=false,sourceId,sceneHash,materializationHash,attemptId,scene_name=sceneName,model=modelName,created=nativeCount,columns,beams,native=nativeCount,placeholder=0,failed_count=0,skipped=unsupported.Count,profiles=profileById,emitted,failed=new object[0],unsupported,warnings };
}
catch(Exception ex){
    gridWarningJournal.Abort();
    // Tekla CommitChanges creates an undo step; it is not an ACID transaction.
    // Before retirement begins, cleanup is safe because the prior set was never
    // touched. Once old deletion starts, another commit could make those deletes
    // durable, so deliberately do not issue a cleanup commit and report uncertainty.
    string failureMessage=ex.Message;bool workPlaneRestored=!scenePlaneChanged;
    if(scenePlaneChanged){try{workPlaneRestored=scenePlaneHandler.SetCurrentTransformationPlane(previousScenePlane);if(workPlaneRestored)scenePlaneChanged=false;}catch{workPlaneRestored=false;}if(!workPlaneRestored)failureMessage+=" The user's prior work plane could not be restored.";}
    bool cleanupDeleted=!retirementStarted,cleanupCommitted=false;
    if(!retirementStarted){for(int i=staged.Count-1;i>=0;i--){try{if(!staged[i].Delete())cleanupDeleted=false;}catch{cleanupDeleted=false;}}if(cleanupDeleted)foreach(var stagedObject in staged){try{if(m.SelectModelObject(new Identifier(stagedObject.Identifier.GUID))!=null)cleanupDeleted=false;}catch{cleanupDeleted=false;}}if(cleanupDeleted)try{cleanupCommitted=m.CommitChanges("AWARE bake-scene cleanup "+attemptId);}catch{}}
    bool uncertain=retirementStarted||!cleanupDeleted||!cleanupCommitted||!workPlaneRestored;
    string causeCode=uncertain?"commit-state-uncertain":"materialization-failed";
    string causeMessage=uncertain?failureMessage+" Tekla has no rollback API and the bridge could not prove complete, durably committed cleanup and work-plane restoration. Re-run bake-scene to reconcile source ownership before editing the model.":failureMessage;
    if(uncertain)warnings.Add(row("scene","scene","failed","commit-state-uncertain","Complete cleanup and a durable cleanup commit could not both be proved."));
    failed.Clear();bool causeAssigned=false;foreach(var item in supportedOrder){bool cause=!causeAssigned&&item.Item1==activeId;var r=row(item.Item1,item.Item2,"failed",cause?causeCode:"batch-aborted",cause?causeMessage:"Batch was aborted after another supported record failed.");r["rolledBack"]=!uncertain&&cleanupCommitted;failed.Add(r);if(cause)causeAssigned=true;}if(!causeAssigned){var r=row(activeId,"scene","failed",causeCode,causeMessage);r["rolledBack"]=!uncertain&&cleanupCommitted;failed.Insert(0,r);}
    // Take back the "adding..." message the caller put up before compiling. Leaving it standing after
    // a failure is the worst version of a stale status: it claims work is still under way in a model
    // where nothing landed, or where ownership is uncertain and the user most needs the truth.
    try{string failLabel=str(args,"label");Tekla.Structures.Model.Operations.Operation.DisplayPrompt((String.IsNullOrWhiteSpace(failLabel)?"":failLabel+": ")+(uncertain?"could not finish adding objects — re-run to reconcile the model.":"nothing was added."));}catch{}
    return new { ok=false,sourceId,sceneHash,materializationHash,attemptId,emitted=new object[0],failed,unsupported,warnings };
}
finally {
    if(scenePlaneChanged)try{scenePlaneHandler.SetCurrentTransformationPlane(previousScenePlane);}catch{}
}
""";
}
