# scene.export-to-images

Render each named scene (page) to a PNG. The designer "Friday client shot pack" primitive.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `out-dir` | string | yes | — | Output directory. PNG filenames derived from scene names. |
| `width` | number | no | 1920 | Pixel width. |
| `height` | number | no | 1080 | Pixel height. |
| `antialias` | boolean | no | true | Render with antialiasing. |
| `scene-name-filter` | string | no | `""` | Optional Ruby regex; only matching scenes are exported. |

## Outputs

```json
{ "exported-count": 7, "files": ["C:/tmp/scenes/Front.png", ...] }
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
require 'fileutils'

model        = Sketchup.active_model
out_dir      = args["out-dir"].to_s
width        = args.fetch("width",  1920).to_i
height       = args.fetch("height", 1080).to_i
antialias    = args.fetch("antialias", true)
name_filter  = args.fetch("scene-name-filter", "").to_s
rx           = name_filter.empty? ? nil : Regexp.new(name_filter)

FileUtils.mkdir_p(out_dir)

scenes = model.pages.select { |p| rx.nil? || rx.match?(p.name) }
if scenes.empty?
  return { "ok" => false, "error" => "no scenes matched filter '#{name_filter}'" }
end

written = []
view = model.active_view
scenes.each do |page|
  page.use_axes = page.use_axes              # touch a property to commit any pending camera changes
  page.parent.selected_page = page if page.respond_to?(:parent)
  view.show_frame                            # advance to the scene

  safe_name = page.name.gsub(/[\\\/:\*\?"<>\|]+/, "_")
  out_path  = File.join(out_dir, "#{safe_name}.png")
  options = {
    :filename    => out_path,
    :width       => width,
    :height      => height,
    :antialias   => antialias,
    :transparent => false,
  }
  ok = view.write_image(options)
  written << out_path if ok
end

{ "ok" => true, "exported_count" => written.size, "files" => written }
```

## Gotchas

- `view.show_frame` advances the view to the selected scene. Without it, exports capture whichever scene was last active.
- Animation transitions are skipped (we use `show_frame`, not `view.animate`).
- The filename sanitizer replaces `\/:*?"<>|` with `_`. Pre-process your scene names if you want different behavior.
- Style/shadow/fog settings come from the scene's saved state. Edit the scene first if you want different style on export.
- `write_image` returns `true` on success, `false` on file-system error. The script counts only successes.

## See also

- `view.write_image` (Ruby API) — underlying primitive
- Compare to Rhino's [`view.capture-bitmap`](../../../architecture/rhino-8/commands/view.capture-bitmap.md) for the equivalent primitive
