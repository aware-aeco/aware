//! YARD-rendered Ruby docs → AWARE agent.
//!
//! YARD (Yay! A Ruby Documentation tool) is the de-facto Ruby docs generator.
//! YARD-rendered HTML follows a stable template — class index at
//! `_index.html`, per-class pages at `<Namespace>/<Class>.html`, methods
//! inside each class page under `<h3 class="signature">`.
//!
//! Use this when a Ruby project publishes its API as YARD HTML but doesn't
//! ship a gem on rubygems.org. Canonical example: SketchUp's Ruby API at
//! <https://ruby.sketchup.com/>.
//!
//! Accepts HTTP(S) URL (e.g. `https://ruby.sketchup.com/`) or a local
//! directory (a YARD output dir on disk).

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use scraper::{Html, Selector};

use crate::builder::{
    GeneratedAgent, GeneratedCommand, GeneratedSkill, Provenance, kebab_ascii, now_iso,
};
use crate::error::AwareError;

pub fn build_from_url_or_dir(
    input: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    let source = YardSource::resolve(input)?;
    let index_html = source.fetch("_index.html")?;
    let class_paths = parse_index_classes(&index_html);

    if class_paths.is_empty() {
        return Err(AwareError::Validation(format!(
            "no classes found in YARD index at {input} \u{2014} not a YARD docs site?"
        )));
    }

    let mut commands: BTreeMap<String, GeneratedCommand> = BTreeMap::new();
    let mut skills: Vec<GeneratedSkill> = Vec::new();

    for class_path in &class_paths {
        let class_html = match source.fetch(class_path) {
            Ok(h) => h,
            Err(_) => continue, // skip broken refs rather than fail the whole build
        };
        let parsed = parse_class_page(&class_html);
        for m in &parsed.methods {
            let cmd_name = kebab_ascii(&format!("{}-{}", parsed.class_name, m.name));
            commands.insert(
                cmd_name,
                GeneratedCommand {
                    lifecycle: "single".into(),
                    description: m.description.clone(),
                    inputs_yaml: String::new(),
                    outputs_yaml: String::new(),
                    ..Default::default()
                },
            );
        }
        // One skill per class — captures the class-level docstring + a roster
        // of its methods so the AI can find the right command.
        let pkg_lower = "yard";
        let skill_stem = kebab_ascii(&parsed.class_name);
        let method_list = parsed
            .methods
            .iter()
            .map(|m| format!("- `{}` \u{2014} {}", m.name, m.description))
            .collect::<Vec<_>>()
            .join("\n");
        let body = format!(
            "---\nname: {pkg_lower}-{skill_stem}\ndescription: {} API reference (YARD)\n---\n\n# {} API reference\n\n{}\n\n## Methods\n\n{}\n",
            parsed.class_name, parsed.class_name, parsed.class_doc, method_list
        );
        skills.push(GeneratedSkill {
            filename: format!("{skill_stem}.md"),
            body,
        });
    }
    skills.sort_by(|a, b| a.filename.cmp(&b.filename));

    let id = agent_id.map(String::from).unwrap_or_else(|| {
        let derived = derive_id_from_input(input);
        if derived.is_empty() {
            "yard-agent".into()
        } else {
            derived
        }
    });

    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({
            "type": "yard",
            "input": input,
            "classes": class_paths.len(),
        }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: None,
        description: format!(
            "Generated from YARD docs at {input} ({} classes, {} methods)",
            class_paths.len(),
            commands.len()
        ),
        commands,
        skills,
        provenance,
        stateful: false,
        license: "see-source".into(),
        rest: None,
    })
}

/// `YardSource` abstracts the difference between fetching pages over HTTP
/// vs reading them from a local YARD output directory.
enum YardSource {
    Url(String),
    Dir(PathBuf),
}

impl YardSource {
    fn resolve(input: &str) -> Result<Self, AwareError> {
        if input.starts_with("http://") || input.starts_with("https://") {
            Ok(YardSource::Url(input.trim_end_matches('/').to_string()))
        } else {
            let path = PathBuf::from(input);
            if !path.is_dir() {
                return Err(AwareError::Validation(format!(
                    "--from-yard expects a URL or a directory; {input} is neither"
                )));
            }
            Ok(YardSource::Dir(path))
        }
    }

    fn fetch(&self, relative_path: &str) -> Result<String, AwareError> {
        match self {
            YardSource::Url(base) => {
                let url = format!("{base}/{relative_path}");
                let resp = ureq::get(&url)
                    .call()
                    .map_err(|e| AwareError::Network(format!("GET {url}: {e}")))?;
                let mut body = String::new();
                resp.into_reader()
                    .read_to_string(&mut body)
                    .map_err(|e| AwareError::Network(format!("read {url}: {e}")))?;
                Ok(body)
            }
            YardSource::Dir(root) => {
                let file = root.join(relative_path);
                std::fs::read_to_string(&file)
                    .map_err(|e| AwareError::NotFound(format!("read {}: {e}", file.display())))
            }
        }
    }
}

/// Parse `_index.html` for the YARD class list. Returns the relative paths
/// to each class page (e.g. `Sketchup/Animation.html`).
fn parse_index_classes(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    // The selector is a constant, so `Err` would mean a malformed literal, not
    // bad input. An empty class list is the honest degradation — the caller
    // already handles a page that yields no classes.
    let Ok(sel) = Selector::parse("a.object_link, span.object_link a") else {
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for a in doc.select(&sel) {
        if let Some(href) = a.value().attr("href")
            && href.ends_with(".html")
            && !href.starts_with("http")
            && !href.starts_with("#")
            && !href.starts_with("file.")
        {
            out.push(href.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

struct ParsedClass {
    class_name: String,
    class_doc: String,
    methods: Vec<ParsedMethod>,
}

struct ParsedMethod {
    name: String,
    description: String,
}

/// Parse a single class page. Class name comes from `<h1>`, class docstring
/// from the first `<div class="docstring">`, methods from `<h3 class="signature">`.
fn parse_class_page(html: &str) -> ParsedClass {
    let doc = Html::parse_document(html);

    let class_name = Selector::parse("h1")
        .ok()
        .and_then(|sel| {
            doc.select(&sel)
                .next()
                .map(|h| text_of(&h).trim().to_string())
        })
        .unwrap_or_else(|| "Unknown".into());
    // YARD <h1> is always "Class: <Name>" or "Module: <Name>"; strip the prefix.
    let class_name = class_name
        .strip_prefix("Class: ")
        .or_else(|| class_name.strip_prefix("Module: "))
        .map(|s| s.to_string())
        .unwrap_or(class_name);

    let class_doc =
        Selector::parse("#description .docstring .discussion p, .docstring .discussion p")
            .ok()
            .and_then(|sel| doc.select(&sel).next().map(|p| text_of(&p)))
            .unwrap_or_default()
            .trim()
            .to_string();

    let mut methods = Vec::new();
    // Both selectors are constants — hoisted out of the loop so `strong` is
    // compiled once rather than per `<h3>`, and so a malformed literal yields
    // an empty method list instead of a panic.
    let (Ok(sig_sel), Ok(strong_sel)) =
        (Selector::parse("h3.signature"), Selector::parse("strong"))
    else {
        return ParsedClass {
            class_name,
            class_doc,
            methods,
        };
    };
    for h3 in doc.select(&sig_sel) {
        // Method name is inside <strong>...</strong>
        let name = h3
            .select(&strong_sel)
            .next()
            .map(|s| text_of(&s).trim().to_string())
            .unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        // Docstring lives in the sibling <div class="docstring"> following <h3>.
        // scraper doesn't expose sibling-of-element traversal cleanly, so we
        // pull the first <p> inside the parent's docstring block.
        let desc = h3
            .parent()
            .and_then(scraper::ElementRef::wrap)
            .and_then(|parent| {
                let p_sel = Selector::parse(".docstring .discussion p").ok()?;
                parent.select(&p_sel).next().map(|p| text_of(&p))
            })
            .unwrap_or_default()
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .to_string();
        methods.push(ParsedMethod {
            name,
            description: desc,
        });
    }

    ParsedClass {
        class_name,
        class_doc,
        methods,
    }
}

fn text_of(el: &scraper::ElementRef) -> String {
    el.text().collect::<Vec<_>>().join("").replace('\n', " ")
}

fn derive_id_from_input(input: &str) -> String {
    let trimmed = input.trim_end_matches('/');
    if let Some(host_pos) = trimmed.find("://") {
        let host = &trimmed[host_pos + 3..];
        let host = host.split('/').next().unwrap_or(host);
        // strip leading 'www.' and trailing TLDs ad-hoc
        let host = host.trim_start_matches("www.");
        return kebab_ascii(host.split('.').next().unwrap_or(host));
    }
    let basename = Path::new(trimmed)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(trimmed);
    kebab_ascii(basename)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INDEX: &str = r#"<!DOCTYPE html>
<html><body>
  <ul class="toplevel">
    <li><span class='object_link'><a href="top-level-namespace.html" title="Top Level Namespace (root)">Top Level Namespace</a></span></li>
  </ul>
  <ul>
    <li><span class='object_link'><a href="Sketchup/Animation.html" title="Sketchup::Animation (class)">Animation</a></span></li>
    <li><span class='object_link'><a href="Sketchup/ArcCurve.html" title="Sketchup::ArcCurve (class)">ArcCurve</a></span></li>
    <li><a class="object_link" href="Layout/Document.html">Layout::Document</a></li>
    <!-- Both of these carry object_link so the SELECTOR matches them and the
         href filters are what exclude them. Without the class they were
         discarded before the filters ran, and the two exclusion assertions
         below held whether the filters existed or not. -->
    <li><span class='object_link'><a href="https://example.com/external.html">External</a></span></li>
    <li><span class='object_link'><a href="file.README.html">Readme</a></span></li>
  </ul>
</body></html>"#;

    const SAMPLE_CLASS_PAGE: &str = r#"<!DOCTYPE html>
<html><body>
<h1>Class: Sketchup::Animation</h1>
<div id="description">
<div class="docstring"><div class="discussion"><p>An Animation is a callback interface used by SketchUp to drive its scene transitions.</p></div></div>
</div>
<div id="instance_method_details" class="method_details_list">
  <div class="method_details first">
    <h3 class="signature first" id="nextFrame-instance_method">
      #<strong>nextFrame</strong>(view)  &#x21d2; <tt>Boolean</tt>
    </h3>
    <div class="docstring"><div class="discussion"><p>Display the next animation frame.</p></div></div>
  </div>
  <div class="method_details">
    <h3 class="signature" id="pause-instance_method">
      #<strong>pause</strong>  &#x21d2; <tt>Object</tt>
    </h3>
    <div class="docstring"><div class="discussion"><p>Pause the animation.</p></div></div>
  </div>
</div>
</body></html>"#;

    #[test]
    fn parse_index_extracts_relative_class_paths_only() {
        // Exact, not `contains`: external URLs and `file.*` overview pages are
        // excluded, and nothing else is.
        assert_eq!(
            parse_index_classes(SAMPLE_INDEX),
            vec![
                "Layout/Document.html".to_string(),
                "Sketchup/Animation.html".to_string(),
                "Sketchup/ArcCurve.html".to_string(),
                "top-level-namespace.html".to_string(),
            ]
        );
    }

    #[test]
    fn parse_class_page_extracts_name_doc_methods() {
        let parsed = parse_class_page(SAMPLE_CLASS_PAGE);
        // `assert_eq`, not `contains`: the `Class: ` prefix has to be gone, and
        // a `contains` check passes with it still attached.
        assert_eq!(parsed.class_name, "Sketchup::Animation");
        assert_eq!(
            parsed.class_doc,
            "An Animation is a callback interface used by SketchUp to drive its scene transitions."
        );
        let names: Vec<&str> = parsed.methods.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["nextFrame", "pause"]);
        assert_eq!(
            parsed.methods[0].description,
            "Display the next animation frame."
        );
        assert_eq!(parsed.methods[1].description, "Pause the animation.");
    }

    #[test]
    fn build_from_local_dir_end_to_end() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("_index.html"), SAMPLE_INDEX).unwrap();
        std::fs::create_dir(root.join("Sketchup")).unwrap();
        std::fs::write(root.join("Sketchup/Animation.html"), SAMPLE_CLASS_PAGE).unwrap();
        // The other classes referenced in SAMPLE_INDEX aren't on disk — that's
        // fine, build_from_url_or_dir skips missing pages rather than failing.
        let agent = build_from_url_or_dir(root.to_str().unwrap(), Some("test")).unwrap();
        assert_eq!(agent.id, "test");
        assert!(agent.commands.contains_key("sketchup-animation-next-frame"));
        assert!(agent.commands.contains_key("sketchup-animation-pause"));
    }

    #[test]
    fn build_rejects_non_yard_input() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("_index.html"),
            "<html><body>not a YARD index</body></html>",
        )
        .unwrap();
        let err = build_from_url_or_dir(tmp.path().to_str().unwrap(), None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn build_rejects_missing_directory() {
        let err = build_from_url_or_dir("/no/such/path", None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn derive_id_from_url_uses_host_first_label() {
        assert_eq!(derive_id_from_input("https://ruby.sketchup.com/"), "ruby");
        assert_eq!(
            derive_id_from_input("https://docs.example.org/api/"),
            "docs"
        );
        assert_eq!(
            derive_id_from_input("/local/path/sketchup-api"),
            "sketchup-api"
        );
    }

    /// A `www.` host has no useful first label — `www` names every site there
    /// has ever been. Stripping it is what makes `https://www.example.com/`
    /// derive `example` rather than an agent id shared with the whole web.
    #[test]
    fn derive_id_strips_a_www_prefix_before_taking_the_first_label() {
        assert_eq!(derive_id_from_input("https://www.example.com/"), "example");
        assert_eq!(
            derive_id_from_input("http://www.ruby.sketchup.com/api"),
            "ruby"
        );
    }

    /// The base is joined to each page as `{base}/{relative_path}`, so a
    /// trailing slash on the input would produce `…//_index.html`. Trimming it
    /// at resolve time is the only place that is fixed.
    #[test]
    fn resolve_trims_trailing_slashes_from_a_url_base() {
        let YardSource::Url(base) = YardSource::resolve("https://ruby.sketchup.com//").unwrap()
        else {
            panic!("an http(s) input must resolve to a Url source, not a Dir");
        };
        assert_eq!(base, "https://ruby.sketchup.com");
    }

    /// A path that exists but is a *file* is not a YARD output dir. It has to be
    /// rejected here, before any page fetch reports a confusing read error.
    #[test]
    fn resolve_rejects_a_path_that_is_not_a_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("_index.html");
        std::fs::write(&file, "<html></html>").unwrap();
        match YardSource::resolve(file.to_str().unwrap()) {
            Err(AwareError::Validation(_)) => {}
            Err(other) => panic!("expected a Validation error, got {other:?}"),
            Ok(_) => panic!("a file is not a YARD output dir and must not resolve"),
        }
    }

    /// Every class page is fetched once per entry, so a duplicate href would
    /// fetch and re-emit the same class twice; and the order feeds the
    /// provenance class count and the skill list, so it has to be stable.
    /// An `href` that is a same-page anchor names no page at all.
    #[test]
    fn parse_index_sorts_dedups_and_drops_same_page_anchors() {
        let html = r##"<html><body>
          <a class="object_link" href="Zeta.html">Z</a>
          <a class="object_link" href="Alpha.html">A</a>
          <a class="object_link" href="Zeta.html">Z again</a>
          <a class="object_link" href="#anchor.html">same page</a>
        </body></html>"##;
        assert_eq!(
            parse_index_classes(html),
            vec!["Alpha.html".to_string(), "Zeta.html".to_string()]
        );
    }

    /// YARD titles a module page `Module: <Name>`, not `Class: <Name>`. Leaving
    /// the prefix on puts it in the command names and the skill filename.
    #[test]
    fn parse_class_page_strips_the_module_prefix_too() {
        let parsed =
            parse_class_page(r#"<html><body><h1>Module: Sketchup::Layout</h1></body></html>"#);
        assert_eq!(parsed.class_name, "Sketchup::Layout");
    }

    /// A page with no `<h1>` still has to yield a usable name rather than an
    /// empty one, which would collapse every command onto a bare method name.
    #[test]
    fn parse_class_page_names_an_h1_less_page_unknown() {
        let parsed = parse_class_page(r#"<html><body><p>no heading here</p></body></html>"#);
        assert_eq!(parsed.class_name, "Unknown");
        assert!(parsed.methods.is_empty());
    }

    /// A `<h3 class="signature">` with no `<strong>` is a signature YARD did not
    /// render a name for. Emitting it would put a command with an empty name in
    /// the manifest, so it is skipped — and the signatures around it are not.
    #[test]
    fn parse_class_page_skips_a_signature_that_names_no_method() {
        let html = r#"<html><body>
          <h1>Class: C</h1>
          <div class="method_details"><h3 class="signature"><strong>first</strong></h3></div>
          <div class="method_details"><h3 class="signature">#(anonymous)</h3></div>
          <div class="method_details"><h3 class="signature"><strong>last</strong></h3></div>
        </body></html>"#;
        let parsed = parse_class_page(html);
        let names: Vec<&str> = parsed.methods.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["first", "last"]);
    }

    /// A docstring wrapped across source lines is one sentence, not two. The
    /// description becomes a single manifest line, so the newline has to become
    /// a space — truncating at it would silently drop half the sentence.
    #[test]
    fn parse_class_page_joins_a_wrapped_docstring_onto_one_line() {
        let html = "<html><body><h1>Class: C</h1>\
          <div class=\"method_details\"><h3 class=\"signature\"><strong>m</strong></h3>\
          <div class=\"docstring\"><div class=\"discussion\"><p>opens a\nnew scene</p></div></div>\
          </div></body></html>";
        let parsed = parse_class_page(html);
        assert_eq!(parsed.methods[0].description, "opens a new scene");
    }

    /// Not every YARD template wraps the class docstring in `#description` — the
    /// bare `.docstring .discussion p` fallback is what keeps the class-level
    /// prose (and therefore the generated skill) from coming out empty.
    #[test]
    fn parse_class_page_reads_a_class_docstring_with_no_description_wrapper() {
        let html = r#"<html><body>
          <h1>Class: Sketchup::View</h1>
          <div class="docstring"><div class="discussion"><p>The drawing surface.</p></div></div>
          <div class="method_details"><h3 class="signature"><strong>invalidate</strong></h3>
            <div class="docstring"><div class="discussion"><p>Redraw it.</p></div></div>
          </div>
        </body></html>"#;
        let parsed = parse_class_page(html);
        assert_eq!(parsed.class_doc, "The drawing surface.");
        assert_eq!(parsed.methods[0].description, "Redraw it.");
    }

    /// Write a two-class YARD dir under `root`.
    ///
    /// `_index.html` advertises three classes; only two pages exist, so the
    /// missing-page skip is exercised by every caller. The two live pages carry
    /// four methods between them, so the advertised class count and the built
    /// command count are never the same number — a test that confuses the two
    /// cannot pass by coincidence.
    fn write_two_class_site(root: &Path) {
        std::fs::write(
            root.join("_index.html"),
            r#"<html><body>
              <a class="object_link" href="Sketchup/Animation.html">Animation</a>
              <a class="object_link" href="Sketchup/View.html">View</a>
              <a class="object_link" href="Sketchup/Gone.html">Gone</a>
            </body></html>"#,
        )
        .unwrap();
        std::fs::create_dir_all(root.join("Sketchup")).unwrap();
        std::fs::write(root.join("Sketchup/Animation.html"), SAMPLE_CLASS_PAGE).unwrap();
        std::fs::write(
            root.join("Sketchup/View.html"),
            r#"<html><body>
              <h1>Class: Sketchup::View</h1>
              <div id="description"><div class="docstring"><div class="discussion">
                <p>The drawing surface of a model.</p>
              </div></div></div>
              <div class="method_details"><h3 class="signature"><strong>invalidate</strong></h3>
                <div class="docstring"><div class="discussion"><p>Redraw the view.</p></div></div>
              </div>
              <div class="method_details"><h3 class="signature"><strong>zoom</strong></h3>
                <div class="docstring"><div class="discussion"><p>Zoom the camera.</p></div></div>
              </div>
            </body></html>"#,
        )
        .unwrap();
    }

    /// The skills are the half of the payload nothing asserted on: one per class,
    /// carrying the class docstring and a roster of its methods so the runtime
    /// can pick a command. Sorted, because `write_agent` lists them in the
    /// manifest in the order they arrive.
    #[test]
    fn build_emits_one_sorted_skill_per_class_listing_its_methods() {
        let tmp = tempfile::tempdir().unwrap();
        write_two_class_site(tmp.path());
        let agent = build_from_url_or_dir(tmp.path().to_str().unwrap(), Some("test")).unwrap();

        let filenames: Vec<&str> = agent.skills.iter().map(|s| s.filename.as_str()).collect();
        assert_eq!(
            filenames,
            vec!["sketchup-animation.md", "sketchup-view.md"],
            "one skill per fetched class, sorted by filename"
        );

        let view = &agent.skills[1].body;
        assert!(
            view.starts_with("---\nname: yard-sketchup-view\n"),
            "skill frontmatter must name the skill; got:\n{view}"
        );
        assert!(
            view.contains("description: Sketchup::View API reference (YARD)"),
            "got:\n{view}"
        );
        assert!(
            view.contains("The drawing surface of a model."),
            "the class docstring belongs in the skill body; got:\n{view}"
        );
        assert!(
            view.contains("- `invalidate` \u{2014} Redraw the view."),
            "the method roster belongs in the skill body; got:\n{view}"
        );
    }

    /// Skills are ordered by their own filename, not by the order the class
    /// pages happened to appear in `_index.html`. The two orders agree on a
    /// conventional YARD site, so this fixture deliberately disagrees: the
    /// first page listed declares the last class alphabetically.
    #[test]
    fn build_sorts_skills_by_filename_not_by_page_order() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(
            root.join("_index.html"),
            r#"<html><body>
              <a class="object_link" href="a-page.html">first listed</a>
              <a class="object_link" href="z-page.html">second listed</a>
            </body></html>"#,
        )
        .unwrap();
        std::fs::write(
            root.join("a-page.html"),
            r#"<html><body><h1>Class: Zebra</h1></body></html>"#,
        )
        .unwrap();
        std::fs::write(
            root.join("z-page.html"),
            r#"<html><body><h1>Class: Alpha</h1></body></html>"#,
        )
        .unwrap();

        let agent = build_from_url_or_dir(root.to_str().unwrap(), Some("test")).unwrap();
        let filenames: Vec<&str> = agent.skills.iter().map(|s| s.filename.as_str()).collect();
        assert_eq!(filenames, vec!["alpha.md", "zebra.md"]);
    }

    /// A method becomes a `single`-lifecycle command carrying its own docstring —
    /// not the class's, and not an empty string.
    #[test]
    fn build_carries_each_method_docstring_onto_its_command() {
        let tmp = tempfile::tempdir().unwrap();
        write_two_class_site(tmp.path());
        let agent = build_from_url_or_dir(tmp.path().to_str().unwrap(), Some("test")).unwrap();

        let cmd = agent
            .commands
            .get("sketchup-view-invalidate")
            .expect("a command per method");
        assert_eq!(cmd.lifecycle, "single");
        assert_eq!(cmd.description, "Redraw the view.");
        assert_eq!(
            agent.commands.len(),
            4,
            "two methods on Animation plus two on View — the unfetchable class contributes none"
        );
    }

    /// The provenance records what the index advertised (three classes) while the
    /// description reports the commands actually built, so a page that 404s is
    /// visible as the gap between the two rather than silently erasing itself.
    #[test]
    fn build_records_the_advertised_class_count_in_its_provenance() {
        let tmp = tempfile::tempdir().unwrap();
        write_two_class_site(tmp.path());
        let input = tmp.path().to_str().unwrap().to_string();
        let agent = build_from_url_or_dir(&input, Some("test")).unwrap();

        assert_eq!(agent.provenance.source["type"], "yard");
        assert_eq!(agent.provenance.source["input"], input);
        assert_eq!(agent.provenance.source["classes"], 3);
        assert!(
            agent.description.ends_with("(3 classes, 4 methods)"),
            "got: {}",
            agent.description
        );
    }

    /// With no `--output`, the id comes from the input. A directory input has no
    /// host to take a label from, so it falls to the directory's own name.
    #[test]
    fn build_derives_the_agent_id_from_the_directory_when_none_is_given() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("sketchup-api");
        std::fs::create_dir(&root).unwrap();
        write_two_class_site(&root);
        let agent = build_from_url_or_dir(root.to_str().unwrap(), None).unwrap();
        assert_eq!(agent.id, "sketchup-api");
    }

    /// A directory whose name is all separators slugifies to nothing. An empty
    /// agent id would write the agent over `agents/` itself, so the fallback
    /// name is what has to come out.
    #[test]
    fn build_falls_back_to_yard_agent_when_the_input_slugifies_to_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("___");
        std::fs::create_dir(&root).unwrap();
        write_two_class_site(&root);
        let agent = build_from_url_or_dir(root.to_str().unwrap(), None).unwrap();
        assert_eq!(agent.id, "yard-agent");
    }
}
