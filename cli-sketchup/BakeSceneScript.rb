# AWARE bake-scene materializer for SketchUp.
#
# Canonical scene (millimetres, host-neutral) -> source-owned Sketchup::Group
# objects, one per member, extruded along the member axis and stamped with the
# ownership attributes that make the next bake a retire-and-replace rather than
# an append.
#
# This file is EMBEDDED IN THE SIDECAR (see BakeSceneScript.cs) and injected
# through the same `exec` request the bridge already serves. It is deliberately
# NOT shipped in BridgeAssets/: the installed bridge only reloads when SketchUp
# restarts, so a materializer living there would silently pair a new sidecar
# with a stale script. Injecting it keeps script and sidecar in lockstep, the
# way cli-tekla's Roslyn BakeSceneScript.Code does.
#
# Contract
# --------
# args['scene']        the canonical scene: meta{name,units,sourceId,sceneHash},
#                      elements[], operations[], referenceSystems[]
# args['supported']    [{id,kind}] the records the sidecar preflight accepted,
#                      in scene order. The sidecar owns envelope/id/kind
#                      validation; this script owns geometry and the model.
# args['ownership']    the ownership vocabulary (dictionary + key names + the
#                      values to stamp). The SIDECAR is the single source of
#                      truth for it; nothing here hardcodes a key name, so the
#                      C# constants and this script cannot drift apart.
# args['label']        optional name to show in SketchUp's status bar.
#
# Returns {ok, emitted[], failed[], unsupported[], warnings[]} with rows shaped
# {id, kind, status, code?, message?} -- the same receipt vocabulary cli-tekla's
# bake-scene emits.
#
# Style note: the bridge evaluates this body inside `->(args) { ... }`. A Ruby
# constant assignment inside a block is a SyntaxError ("dynamic constant
# assignment"), so EVERYTHING here is a local variable or a lambda. `lambda`
# (never `proc`) so an inner `return` is local to that lambda.

# --- receipt helpers ---------------------------------------------------------

row = lambda do |id, kind, status, code, message|
  r = { 'id' => id.to_s, 'kind' => kind.to_s, 'status' => status.to_s }
  r['code'] = code.to_s unless code.nil? || code.to_s.empty?
  r['message'] = message.to_s unless message.nil? || message.to_s.empty?
  r
end

refuse = lambda do |code, message|
  {
    'ok' => false,
    'emitted' => [],
    'failed' => [row.call('scene', 'scene', 'failed', code, message)],
    'unsupported' => [],
    'warnings' => [],
  }
end

# --- host + unit preconditions ----------------------------------------------

model = (Sketchup.active_model rescue nil)
return refuse.call('host-unavailable', 'No live SketchUp model is open.') if model.nil?

# The scene is millimetres; SketchUp's internal unit is inches. Use SketchUp's
# own Numeric#mm / Length#to_mm rather than a hand-rolled 25.4, and prove the
# pair round-trips on THIS host before any geometry is built from it.
unit_probe = 1234.5
unless ((unit_probe.mm).to_mm - unit_probe).abs < 1.0e-6 && ((25.4).mm - 1.0).abs < 1.0e-9
  return refuse.call('host-unit-conversion',
                     'SketchUp millimetre conversion did not round-trip on this host; refusing to bake.')
end

# SketchUp collapses geometry closer together than roughly 0.001 inch. A member
# shorter than that is valid in a millimetre scene and degenerate here, so it is
# refused rather than silently producing nothing.
tol_in = 1.0e-3

# --- scene envelope ----------------------------------------------------------

scene = args['scene']
return refuse.call('invalid-scene', 'scene must be an object') unless scene.is_a?(Hash)

meta = scene['meta'].is_a?(Hash) ? scene['meta'] : {}
units      = meta['units'].to_s.strip
source_id  = meta['sourceId'].to_s.strip
scene_hash = meta['sceneHash'].to_s.strip
scene_name = meta['name'].to_s.strip
scene_name = 'Steel from Drawings' if scene_name.empty?

unless units.empty? || units.downcase == 'mm'
  return refuse.call('unsupported-units', 'SketchUp bake-scene accepts only millimetres')
end
return refuse.call('missing-source-id', 'scene.meta.sourceId is required') if source_id.empty?
return refuse.call('missing-scene-hash', 'scene.meta.sceneHash is required') if scene_hash.empty?

own = args['ownership'].is_a?(Hash) ? args['ownership'] : {}
dict     = own['dictionary'].to_s
k_src    = own['sourceIdKey'].to_s
k_rec    = own['recordIdKey'].to_s
k_scn    = own['sceneHashKey'].to_s
k_mrk    = own['markerKey'].to_s
marker   = own['marker'].to_s
tag_name = own['tag'].to_s
marker_pattern = own['markerPattern'].to_s
if dict.empty? || k_src.empty? || k_rec.empty? || k_scn.empty? || k_mrk.empty? ||
   marker.empty? || tag_name.empty? || marker_pattern.empty?
  return refuse.call('invalid-ownership',
                     'args.ownership must carry the dictionary, key names, marker and tag from the sidecar')
end
marker_re = (Regexp.new(marker_pattern) rescue nil)
return refuse.call('invalid-ownership', 'args.ownership.markerPattern is not a valid regular expression') if marker_re.nil?

# The ownership source id is what selects the retirement set. An empty one would
# match every entity that has never been stamped -- the single mistake that turns
# a bake into "erase the user's model". Refuse before touching anything.
own_source = own['sourceId'].to_s
return refuse.call('invalid-ownership', 'args.ownership.sourceId is required') if own_source.empty?
# Stamping one source id while scanning for another would append forever and
# never retire. They come from the same field; prove it rather than assume it.
unless own_source == source_id
  return refuse.call('invalid-ownership',
                     'args.ownership.sourceId does not match scene.meta.sourceId')
end

materialization_hash = args['materializationHash'].to_s
attempt_id = (0...32).map { rand(16).to_s(16) }.join
label = args['label'].to_s.strip

# --- small numeric / vector helpers -----------------------------------------

num = lambda do |v|
  return v.to_f if v.is_a?(Numeric)
  return nil unless v.is_a?(String)
  (Float(v) rescue nil)
end

finite = lambda { |f| f.is_a?(Float) && !f.nan? && f.infinite?.nil? }

vec3 = lambda do |raw|
  return nil unless raw.is_a?(Array) && raw.length >= 3
  xs = (0..2).map { |i| num.call(raw[i]) }
  return nil unless xs.all? { |f| !f.nil? && finite.call(f) }
  xs
end

vlen   = lambda { |v| Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) }
vdot   = lambda { |a, b| a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
vcross = lambda { |a, b| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
vnorm  = lambda do |v|
  l = vlen.call(v)
  [v[0] / l, v[1] / l, v[2] / l]
end
# Rodrigues rotation of `v` about the UNIT axis `k` by `ang` radians.
vrot = lambda do |v, k, ang|
  c = Math.cos(ang)
  s = Math.sin(ang)
  kv = vcross.call(k, v)
  d = vdot.call(k, v) * (1.0 - c)
  [v[0] * c + kv[0] * s + k[0] * d,
   v[1] * c + kv[1] * s + k[1] * d,
   v[2] * c + kv[2] * s + k[2] * d]
end

# --- cross-section outlines --------------------------------------------------
#
# These mirror floless's 3D viewer (web/steel-3d-view.js profileShape) point for
# point, so what SketchUp receives is the shape the user was looking at when
# they dragged it out. Local axes: x = section width (w), y = section depth (d),
# both centred on the reference line; the member axis is local z.

real_thk = lambda do |xs, key, half|
  t = xs.is_a?(Hash) ? num.call(xs[key]) : nil
  (!t.nil? && t > 0 && t < 2 * half) ? t : nil
end

# Which outline family a member draws with.
#
# THE SUPPLIED `xsection` DECIDES, and the designation is only a legacy fallback. That is this
# substrate's own rule -- "producers own domain meaning; the renderer owns exact visualization"
# (viewer-3d/skills/scene-schema.md) -- and `_core/viewer-3d`, `_core/ifc` and `rhino-8` already
# follow it. This sink did not, and the consequences were not subtle:
#
#   * The `explicit` branch below read `section['shape']`, a key producers do not emit. floless sends
#     `section: {w, d, round}`. So the branch never once executed and every member was classified by
#     the designation regex, which is a US-only prefix list.
#   * `CHS508.0*14.2` matched the CHANNEL rule (a bare `C`) before the tube rule, so a 508 mm round
#     column baked as a channel. The old comment above called that ordering deliberate "so what
#     SketchUp receives is the shape the user was looking at" -- but the viewer resolves from
#     `xsection.shape`, so mirroring its REGRESSION rather than its answer is what caused the drift.
#   * Every UK angle (`UKA…`, `RSA…`) and channel (`UKPFC…`, `RSC…`) matched nothing and fell through
#     to `BOX` -- a solid rectangle where a 100x100x10 angle belongs, with the supplied `t` ignored.
#
# `round` is carried too: a circular section is NOT a square tube. `TUBE` here draws a rectangle, so
# a US `PIPE` has always baked square. That is a shipped US defect the same rule fixes.
#
# Returns [kind, round]. `kind` is one of I, C, L, T, TUBE, BOX.
#
# A LOCAL, not a constant: the bridge wraps this script in a lambda and Ruby makes constant
# assignment a SyntaxError inside one. The sink's own test pins that footgun.
xs_kind = {
  'i' => 'I', 'channel' => 'C', 'tee' => 'T', 'angle' => 'L',
  'rhs' => 'TUBE', 'chs' => 'TUBE', 'double-angle' => 'LL',
  'rect' => 'BOX', 'solid-circle' => 'BOX',
  # A shape the catalogue refuses has no drawable geometry; `BOX` is the honest placeholder and the
  # receipt records it, exactly as the producer's own viewer does.
  'unsupported-custom' => 'BOX', 'joist' => 'BOX'
}.freeze

shape_of = lambda do |el|
  sec = el['section'].is_a?(Hash) ? el['section'] : {}
  round = sec['round'] == true
  xs = el['xsection']
  if xs.is_a?(Hash)
    mapped = xs_kind[xs['shape'].to_s.downcase.strip]
    return [mapped, xs['shape'].to_s.downcase.strip == 'chs' || round] if mapped
  end
  # LEGACY ONLY: a producer that predates `xsection`. Kept so an older scene still bakes, and
  # ordered longest-prefix-first so `CHS`/`SHS` cannot be eaten by the one-letter families -- the
  # exact shadowing that made `C` swallow `CHS`.
  m = el['meta'].is_a?(Hash) ? el['meta'] : {}
  prof = m['profile'].to_s.upcase.strip
  return ['TUBE', prof =~ /\A(PIPE|CHS)/ ? true : round] if prof =~ /\A(HSS|PIPE|TS|SHS|RHS|CHS|TUBE|HSQ|EHS)/
  return ['C', round] if prof =~ /\A(UKPFC|RSC|MC|PFC|C)/
  return ['L', round] if prof =~ /\A(UKA|RSA|L)/
  return ['T', round] if prof =~ /\A(UKT|TEE|WT|ST|MT)/
  return ['I', round] if prof =~ /\A(UKBP|UKB|UKC|RSJ|JUMBO|IPE|HE|HP|UB|UC|W|M|S)/
  ['BOX', round]
end

# Returns [outer_points_mm, inner_points_mm_or_nil].
#
# ROUND_SEGMENTS matches floless's `web/steel-3d-core.js` exactly (48, inscribed). The polygon is
# INSCRIBED, so its widest point between two vertices is r*cos(pi/48); a different count here would
# put a baked pipe's flush face a fraction off the one the user approved on screen. A LOCAL, not a
# constant, for the same lambda-scope reason as `xs_kind` above.
round_segments = 48

ring = lambda do |rad|
  (0...round_segments).map do |i|
    a = (i.to_f / round_segments) * Math::PI * 2.0
    [rad * Math.cos(a), rad * Math.sin(a)]
  end
end

outline = lambda do |kind, w, d, xs, round = false|
  hw = w / 2.0
  hd = d / 2.0
  if kind == 'TUBE' && round
    # A ROUND section is a CIRCLE, not the w x d rectangle those two numbers imply: `profileDims`
    # squares a pipe into w = d = OD, and this sink took that at face value and baked a square tube
    # for every PIPE and every CHS. The producer says `section.round` / `xsection.shape == 'chs'`.
    r = [hw, hd].min
    t = real_thk.call(xs, 't', r) || [r * 0.12, 4.0].max
    [ring.call(r), (r - t > 1.0e-6 ? ring.call(r - t) : nil)]
  elsif kind == 'T'
    # A TEE is half an I: one flange plus a stem hanging off it. Baking it as a full I (which the
    # designation classifier did, because `WT`/`UKT` start with letters the I-rule claims) doubles
    # the steel. UK ships UKT/TEE families and US ships WT/ST/MT, so both were wrong.
    tf = real_thk.call(xs, 'tf', hd) || [[d * 0.10, 6.0].max, d * 0.5].min
    tw = real_thk.call(xs, 'tw', hw) || [[w * 0.10, 5.0].max, w * 0.5].min
    [[[-hw, -hd], [hw, -hd], [hw, -hd + tf], [tw / 2.0, -hd + tf],
      [tw / 2.0, hd], [-tw / 2.0, hd], [-tw / 2.0, -hd + tf], [-hw, -hd + tf]], nil]
  elsif kind == 'LL'
    # A DOUBLE ANGLE is two separate legs with a gap; one closed loop cannot express it, so this
    # bakes the pair's ENVELOPE and the receipt says so. Better than the solid box it used to get,
    # and honest about what it is not.
    t = real_thk.call(xs, 't', [hw, hd].min) || [[w, d].min * 0.18, 5.0].max
    [[[-hw, -hd], [hw, -hd], [hw, -hd + t], [-hw + t, -hd + t], [-hw + t, hd], [-hw, hd]], nil]
  elsif kind == 'I'
    tf = real_thk.call(xs, 'tf', hd) || [[d * 0.10, 6.0].max, d * 0.5].min
    tw = real_thk.call(xs, 'tw', hw) || [[w * 0.10, 5.0].max, w * 0.5].min
    [[[-hw, -hd], [hw, -hd], [hw, -hd + tf], [tw / 2.0, -hd + tf],
      [tw / 2.0, hd - tf], [hw, hd - tf], [hw, hd], [-hw, hd],
      [-hw, hd - tf], [-tw / 2.0, hd - tf], [-tw / 2.0, -hd + tf], [-hw, -hd + tf]], nil]
  elsif kind == 'C'
    tf = real_thk.call(xs, 'tf', hd) || [d * 0.10, 5.0].max
    tw = real_thk.call(xs, 'tw', hw) || [w * 0.12, 5.0].max
    [[[-hw, -hd], [hw, -hd], [hw, -hd + tf], [-hw + tw, -hd + tf],
      [-hw + tw, hd - tf], [hw, hd - tf], [hw, hd], [-hw, hd]], nil]
  elsif kind == 'L'
    t = real_thk.call(xs, 't', [hw, hd].min) || [[w, d].min * 0.18, 5.0].max
    [[[-hw, -hd], [hw, -hd], [hw, -hd + t], [-hw + t, -hd + t], [-hw + t, hd], [-hw, hd]], nil]
  elsif kind == 'TUBE'
    t = real_thk.call(xs, 't', [hw, hd].min) || [[w, d].min * 0.12, 4.0].max
    outer = [[-hw, -hd], [hw, -hd], [hw, hd], [-hw, hd]]
    inner = nil
    if hw - t > 1.0e-6 && hd - t > 1.0e-6
      inner = [[-hw + t, -hd + t], [hw - t, -hd + t], [hw - t, hd - t], [-hw + t, hd - t]]
    end
    [outer, inner]
  else
    [[[-hw, -hd], [hw, -hd], [hw, hd], [-hw, hd]], nil]
  end
end

# --- preflight: prove every member BEFORE the model is opened for writing ----

supported = args['supported'].is_a?(Array) ? args['supported'] : []
elements  = scene['elements'].is_a?(Array) ? scene['elements'] : []
by_id = {}
elements.each { |e| by_id[e['id'].to_s] = e if e.is_a?(Hash) }

emitted  = []
failed   = []
warnings = []
plans    = []

supported.each do |entry|
  id = (entry.is_a?(Hash) ? entry['id'] : entry).to_s
  kind = (entry.is_a?(Hash) ? entry['kind'] : 'member').to_s
  kind = 'member' if kind.empty?
  begin
    el = by_id[id]
    raise "#{id}: the scene carries no element with this id" if el.nil?

    from = vec3.call(el['from'])
    to   = vec3.call(el['to'])
    raise "#{id}: member requires finite from/to points in millimetres" if from.nil? || to.nil?

    axis = [to[0] - from[0], to[1] - from[1], to[2] - from[2]]
    len_mm = vlen.call(axis)
    len_in = len_mm.mm
    if len_in <= tol_in
      raise "#{id}: member axis is #{format('%.6f', len_mm)} mm, shorter than SketchUp's 0.001 inch tolerance"
    end

    sec = el['section'].is_a?(Hash) ? el['section'] : {}
    w = num.call(sec['w'])
    d = num.call(sec['d'])
    if w.nil? || d.nil? || !finite.call(w) || !finite.call(d) || w <= 0 || d <= 0
      raise "#{id}: member requires a positive section {w,d} in millimetres; bake-scene never invents a section"
    end
    if w.mm <= tol_in || d.mm <= tol_in
      raise "#{id}: member section #{w} x #{d} mm is below SketchUp's 0.001 inch tolerance"
    end

    member_kind, member_round = shape_of.call(el)
    outer, inner = outline.call(member_kind, w, d, el['xsection'], member_round)

    # Local frame. mv is the member axis (local z); ydir is the section depth and
    # is rolled so the web stands vertical, exactly as the viewer orients it; a
    # vertical member has no world-up to align to, so it keeps the authored roll.
    mv = vnorm.call(axis)
    dz = vdot.call([0.0, 0.0, 1.0], mv)
    proj = [-mv[0] * dz, -mv[1] * dz, 1.0 - mv[2] * dz]
    ydir = vlen.call(proj) > 1.0e-6 ? vnorm.call(proj) : [0.0, 1.0, 0.0]
    xdir = vcross.call(ydir, mv)
    roll = num.call(el['rot'])
    roll = 0.0 if roll.nil? || !finite.call(roll)
    if roll != 0.0
      rad = roll * Math::PI / 180.0
      xdir = vrot.call(xdir, mv, rad)
      ydir = vrot.call(ydir, mv, rad)
    end

    el_meta = el['meta'].is_a?(Hash) ? el['meta'] : {}
    name = el['name'].to_s.strip
    name = el_meta['name'].to_s.strip if name.empty?
    name = el_meta['profile'].to_s.strip if name.empty?
    display = name.empty? ? id : "#{name} [#{id}]"

    plans << {
      'id' => id, 'kind' => kind, 'name' => display,
      'origin' => from, 'x' => xdir, 'y' => ydir, 'z' => mv,
      'lengthIn' => len_in, 'outer' => outer, 'inner' => inner,
      'profile' => el_meta['profile'].to_s,
    }
  rescue StandardError => e
    failed << row.call(id, kind, 'failed', 'invalid-geometry', e.message)
  end
end

if failed.any?
  done = {}
  failed.each { |f| done[f['id']] = true }
  supported.each do |entry|
    id = (entry.is_a?(Hash) ? entry['id'] : entry).to_s
    kind = (entry.is_a?(Hash) ? entry['kind'] : 'member').to_s
    next if done[id]
    done[id] = true
    failed << row.call(id, kind, 'failed', 'batch-aborted',
                       'Batch was aborted because another supported record failed preflight.')
  end
  return {
    'ok' => false, 'sourceId' => source_id, 'sceneHash' => scene_hash,
    'materializationHash' => materialization_hash, 'attemptId' => attempt_id,
    'emitted' => [], 'failed' => failed, 'unsupported' => [], 'warnings' => warnings,
  }
end

# --- ownership scan ----------------------------------------------------------
#
# An entity is retired only when it carries ALL FOUR ownership attributes, its
# source id equals THIS bake's source id, and its marker is well formed. Anything
# stamped by another source -- another drag-out, another app, another bake -- and
# anything the user drew themselves fails the first test and is never touched.

owned = lambda do |e|
  return false unless e.respond_to?(:get_attribute)
  return false unless e.get_attribute(dict, k_src).to_s == own_source
  return false if e.get_attribute(dict, k_rec).to_s.strip.empty?
  return false if e.get_attribute(dict, k_scn).to_s.strip.empty?
  !(marker_re =~ e.get_attribute(dict, k_mrk).to_s).nil?
end

collect_owned = nil
collect_owned = lambda do |entities, out, depth|
  return if depth > 16
  entities.each do |e|
    next unless e.valid?
    if owned.call(e)
      out << e
      next
    end
    # Descend into plain groups only. A ComponentDefinition is shared by every
    # instance of it, so erasing inside one would silently edit unrelated
    # geometry; bake-scene never creates component instances, so it never needs to.
    collect_owned.call(e.entities, out, depth + 1) if e.is_a?(Sketchup::Group)
  end
end

# --- one operation, one undo -------------------------------------------------

status = lambda do |text|
  begin
    Sketchup.status_text = label.empty? ? text : "#{label}: #{text}"
  rescue StandardError
    nil
  end
end

plural = plans.length == 1 ? '' : 's'
status.call("adding #{plans.length} object#{plural} to this model...")

active_id = plans.empty? ? 'scene' : plans[0]['id']
committed = false
model.start_operation("AWARE bake-scene #{scene_name}", true)
begin
  prior = []
  collect_owned.call(model.entities, prior, 0)

  layer = model.layers.add(tag_name)
  root = model.entities
  fresh = []

  plans.each do |plan|
    active_id = plan['id']
    grp = root.add_group
    ge = grp.entities

    face = ge.add_face(plan['outer'].map { |xy| Geom::Point3d.new(xy[0].mm, xy[1].mm, 0) })
    raise "#{plan['id']}: SketchUp could not build the profile face" if face.nil? || !face.valid?
    if plan['inner']
      hole = ge.add_face(plan['inner'].map { |xy| Geom::Point3d.new(xy[0].mm, xy[1].mm, 0) })
      hole.erase! if !hole.nil? && hole.valid?
      raise "#{plan['id']}: the hollow profile outline collapsed" unless face.valid?
    end
    face.reverse! if face.normal.z < 0
    face.pushpull(plan['lengthIn'])
    # pushpull returns nil either way, so prove the extrusion by its result: a
    # prism has the cap plus one side face per outline edge, never a lone face.
    raise "#{plan['id']}: pushpull did not extrude the profile" if ge.grep(Sketchup::Face).length < 3

    grp.transformation = Geom::Transformation.axes(
      Geom::Point3d.new(plan['origin'][0].mm, plan['origin'][1].mm, plan['origin'][2].mm),
      Geom::Vector3d.new(plan['x'][0], plan['x'][1], plan['x'][2]),
      Geom::Vector3d.new(plan['y'][0], plan['y'][1], plan['y'][2]),
      Geom::Vector3d.new(plan['z'][0], plan['z'][1], plan['z'][2]))

    grp.name = plan['name']
    grp.layer = layer
    grp.set_attribute(dict, k_src, source_id)
    grp.set_attribute(dict, k_rec, plan['id'])
    grp.set_attribute(dict, k_scn, scene_hash)
    grp.set_attribute(dict, k_mrk, marker)
    fresh << [plan, grp]
  end

  # Prove the replacement set before a single prior object is retired. A bake
  # that cannot verify its own ownership stamps must not delete anything.
  fresh.each do |plan, grp|
    active_id = plan['id']
    unless grp.valid? &&
           grp.get_attribute(dict, k_src).to_s == source_id &&
           grp.get_attribute(dict, k_rec).to_s == plan['id'] &&
           grp.get_attribute(dict, k_scn).to_s == scene_hash &&
           grp.get_attribute(dict, k_mrk).to_s == marker
      raise "#{plan['id']}: ownership attribute read-back failed"
    end
  end

  active_id = 'scene'
  retired = 0
  prior.each do |e|
    next unless e.valid?
    e.erase!
    retired += 1
  end
  prior.each do |e|
    raise 'a prior source-owned object survived retirement' if e.valid?
  end

  # Read every id the receipt will quote BEFORE committing, so that once the
  # commit lands nothing left in this block can raise. Anything that threw after
  # commit_operation would send the rescue below into abort_operation on a closed
  # operation, and the caller would be told a landed bake had rolled back.
  fresh.each do |plan, grp|
    r = row.call(plan['id'], plan['kind'], 'emitted', nil, nil)
    r['persistentId'] = grp.persistent_id.to_s if grp.respond_to?(:persistent_id)
    r['entityId'] = grp.entityID if grp.respond_to?(:entityID)
    r['profile'] = plan['profile'] unless plan['profile'].empty?
    emitted << r
  end
  model_title = (model.title.to_s rescue '')

  model.commit_operation
  committed = true

  status.call("#{emitted.length} member#{emitted.length == 1 ? '' : 's'} added.")

  {
    'ok' => true,
    'sourceId' => source_id,
    'sceneHash' => scene_hash,
    'materializationHash' => materialization_hash,
    'attemptId' => attempt_id,
    'scene_name' => scene_name,
    'model' => model_title,
    'tag' => tag_name,
    'created' => emitted.length,
    'retired' => retired,
    'emitted' => emitted,
    'failed' => [],
    'unsupported' => [],
    'warnings' => warnings,
  }
rescue StandardError => e
  # SketchUp DOES have a real rollback: abort_operation reverts every entity
  # created, modified or erased since start_operation. Unlike the Tekla sink
  # there is no "commit-state-uncertain" state to report -- either the whole
  # bake landed or the model is exactly as the user left it.
  # Only a still-open operation may be aborted. `committed` is the guard: if the
  # commit already landed there is nothing to roll back and saying otherwise
  # would be a lie about the state of the user's model.
  rolled_back = false
  if committed
    warnings << row.call('scene', 'scene', 'failed', 'commit-state-uncertain',
                         'The bake committed and then failed while reporting; re-run bake-scene to reconcile source ownership.')
  else
    rolled_back = true
    begin
      model.abort_operation
    rescue StandardError
      rolled_back = false
    end
    unless rolled_back
      warnings << row.call('scene', 'scene', 'failed', 'commit-state-uncertain',
                           'SketchUp refused to abort the bake operation; re-run bake-scene to reconcile source ownership.')
    end
  end
  status.call(rolled_back ? 'nothing was added.' : 'could not finish adding objects -- re-run to reconcile the model.')
  cause_taken = false
  plans.each do |plan|
    is_cause = !cause_taken && plan['id'] == active_id
    cause_taken = true if is_cause
    r = row.call(plan['id'], plan['kind'], 'failed',
                 is_cause ? 'materialization-failed' : 'batch-aborted',
                 is_cause ? e.message : 'Batch was aborted after another supported record failed.')
    r['rolledBack'] = rolled_back
    failed << r
  end
  unless cause_taken
    r = row.call(active_id, 'scene', 'failed', 'materialization-failed', e.message)
    r['rolledBack'] = rolled_back
    failed.unshift(r)
  end
  {
    'ok' => false,
    'sourceId' => source_id,
    'sceneHash' => scene_hash,
    'materializationHash' => materialization_hash,
    'attemptId' => attempt_id,
    'emitted' => [],
    'failed' => failed,
    'unsupported' => [],
    'warnings' => warnings,
  }
end
