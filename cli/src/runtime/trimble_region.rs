//! Region-aware routing for Trimble Connect's Core API (floless.app#2184).
//!
//! Every Trimble Connect project lives in one region, fixed when it is
//! created, and Trimble's Core docs say its data "must be accessed through the
//! corresponding regional endpoints". The agent's manifest base is the North
//! America master (`app.connect.trimble.com`); a Europe project's folders,
//! files and ToDos are served by `app21`, the UK's by `app22`, and so on.
//!
//! The region is chosen per command, in this order:
//!
//! 1. the `region` input — the project's `location` as TC names it
//!    (`europe`, `unitedKingdom`, …; `get-project` and `list-projects` return
//!    it), so a command that takes only a folder, file or ToDo id can reach a
//!    non-NA project;
//! 2. otherwise, for a command that names a project, that project's
//!    `location` (`GET {master}/projects/{id}`);
//! 3. otherwise the master, as before this module existed.
//!
//! A location becomes a host only through the public `GET {master}/regions`
//! (its `tc-api`), and only an exact published Core host ([`CORE_HOSTS`]) is
//! ever sent the token. The `/regions` lookup follows no redirect.

use std::time::Duration;

use serde_json::Value;

use crate::error::AwareError;
use crate::runtime::invoker::percent_encode_path;
use crate::runtime::trimble_files::{get_json, json_body};

/// The Core API hosts Trimble publishes (`tc-api` of each production region in
/// `GET /regions`, and the `servers` of the tcps/2.0 definition). The bearer
/// token goes to no other Core host.
pub(super) const CORE_HOSTS: &[&str] = &[
    "app.connect.trimble.com",
    "app21.connect.trimble.com",
    "app22.connect.trimble.com",
    "app31.connect.trimble.com",
    "app32.connect.trimble.com",
];

/// The longest a TC location name may be; the five published ones are at most
/// 13 characters.
const MAX_REGION_LEN: usize = 64;

/// `https://<one of CORE_HOSTS>` on the default port, with no credentials in
/// the URL.
pub(super) fn prod_core_host(url: &url::Url) -> bool {
    url.scheme() == "https"
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
        && url.host_str().is_some_and(|h| CORE_HOSTS.contains(&h))
}

/// Whether a discovered Core API may receive the token. Test builds also admit
/// the 127.0.0.1 wire mock; there is no runtime switch to widen it.
pub(super) fn core_host_allowed(url: &url::Url) -> bool {
    prod_core_host(url)
        || (cfg!(test) && url.scheme() == "http" && url.host_str() == Some("127.0.0.1"))
}

/// The optional `region` input: absent, null or empty means "not given". A
/// given one is a TC location name — ASCII letters and digits only, so it can
/// never smuggle a path, query or host into a URL.
pub(super) fn region_arg(args: &Value) -> Result<Option<&str>, AwareError> {
    let region = match args.get("region") {
        None | Some(Value::Null) => return Ok(None),
        Some(Value::String(s)) if s.is_empty() => return Ok(None),
        Some(Value::String(s)) => s.as_str(),
        Some(other) => {
            return Err(AwareError::Validation(format!(
                "trimble-connect: `region` must be a string, got {other}"
            )));
        }
    };
    if region.len() > MAX_REGION_LEN || !region.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AwareError::Validation(format!(
            "trimble-connect: `region` must be a Trimble Connect location such as `europe` \
             or `unitedKingdom` (the project's `location`), got {region:?}"
        )));
    }
    Ok(Some(region))
}

/// The `field` API (`tc-api`, `topic-api`) that a `GET /regions` answer lists
/// for `location`.
pub(super) fn api_for<'a>(regions: &'a Value, location: &str, field: &str) -> Option<&'a str> {
    regions
        .as_array()?
        .iter()
        .find(|r| r.get("location").and_then(Value::as_str) == Some(location))?
        .get(field)?
        .as_str()
}

/// No redirects (a 3xx is an error, never followed): a discovered host is
/// used only as TC named it.
fn lookup_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
}

/// The Core base (no trailing `/`) of `location`: the public `GET
/// {master}/regions`, asked without the token, mapped to that region's
/// `tc-api`, which must be a published Core host.
pub(super) fn regional_core(master: &str, location: &str) -> Result<String, AwareError> {
    let what = "Core region";
    let regions = get_json(
        &lookup_agent(),
        &format!("{master}/regions"),
        None,
        &format!("{what}: regions"),
    )?;
    let tc_api = api_for(&regions, location, "tc-api").ok_or_else(|| {
        AwareError::Validation(format!(
            "{what}: Trimble Connect lists no region {location:?} — use a project's `location` \
             as `get-project` or `list-projects` return it"
        ))
    })?;
    let parsed = url::Url::parse(tc_api)
        .map_err(|e| AwareError::Network(format!("{what}: Core API URL {tc_api:?}: {e}")))?;
    if !core_host_allowed(&parsed) {
        return Err(AwareError::Network(format!(
            "{what}: TC named {tc_api:?} as the Core API for {location:?}; that is not a \
             known Trimble Connect Core host, so the token is not sent there"
        )));
    }
    Ok(tc_api.trim_end_matches('/').to_string())
}

/// Fetches one JSON document for [`locate_project`]: `(url, with_token)` →
/// `Ok(None)` on a 404, the body on a 2xx, an error otherwise.
pub(super) type Fetch<'a> = dyn FnMut(&str, bool) -> Result<Option<Value>, AwareError> + 'a;

/// Where `project` lives. The master knows only its own region's projects —
/// live, it answers 404 for a Europe project's `GET projects/{id}` — so on a
/// 404 each other region the public `GET {master}/regions` lists is asked in
/// turn, and only at a published Core host. The first that knows the project
/// gives its `location`.
pub(super) fn locate_project(
    master: &str,
    project: &str,
    fetch: &mut Fetch<'_>,
) -> Result<String, AwareError> {
    let what = "region: project";
    let path = format!("projects/{}", percent_encode_path(project));
    let location_of = |info: &Value| {
        info.get("location")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .ok_or_else(|| {
                AwareError::Network(format!("{what}: project {project} has no `location`"))
            })
    };
    if let Some(info) = fetch(&format!("{master}/{path}"), true)? {
        return location_of(&info);
    }
    let regions = fetch(&format!("{master}/regions"), false)?
        .ok_or_else(|| AwareError::Network(format!("{what}: {master}/regions answered 404")))?;
    for tc_api in regions
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| r.get("tc-api").and_then(Value::as_str))
    {
        let base = tc_api.trim_end_matches('/');
        if base == master || !url::Url::parse(tc_api).is_ok_and(|u| core_host_allowed(&u)) {
            continue;
        }
        if let Some(info) = fetch(&format!("{base}/{path}"), true)? {
            return location_of(&info);
        }
    }
    Err(AwareError::Validation(format!(
        "{what}: no Trimble Connect region knows project {project} — check the id and that \
         this account can open it"
    )))
}

/// [`locate_project`] over plain requests that follow no redirect.
pub(super) fn project_location(
    master: &str,
    token: &str,
    project: &str,
) -> Result<String, AwareError> {
    let agent = lookup_agent();
    locate_project(master, project, &mut |url, with_token| {
        let mut req = agent.get(url);
        if with_token {
            req = req.set("Authorization", &format!("Bearer {token}"));
        }
        match req.call() {
            Err(ureq::Error::Status(404, _)) => Ok(None),
            res => json_body(res, "region: project").map(Some),
        }
    })
}

/// The Core base one command uses, and the location it resolved (`None` when
/// it fell back to the master without knowing one): the `region` input, else
/// the named project's location, else the master.
pub(super) fn resolve_core(
    master: &str,
    token: &str,
    region: Option<&str>,
    project: Option<&str>,
) -> Result<(String, Option<String>), AwareError> {
    let location = match (region, project) {
        (Some(r), _) => r.to_string(),
        (None, Some(p)) => project_location(master, token, p)?,
        (None, None) => return Ok((master.to_string(), None)),
    };
    Ok((regional_core(master, &location)?, Some(location)))
}

/// For a declarative Core read (the generic REST renderer builds the request):
/// take `region` out of `args` — it picks the host and is not a TC parameter —
/// and return `(master, regional)` bases for [`rebase`], or `None` to stay on
/// the master. Blocking HTTP runs off the reactor.
pub(crate) async fn take_declarative_region(
    master: Option<String>,
    args: &mut Value,
) -> Result<Option<(String, String)>, AwareError> {
    let region = region_arg(args)?.map(str::to_string);
    if let Some(obj) = args.as_object_mut() {
        obj.remove("region");
    }
    let (Some(location), Some(master)) = (region, master) else {
        return Ok(None);
    };
    let master = master.trim_end_matches('/').to_string();
    tokio::task::spawn_blocking(move || {
        regional_core(&master, &location).map(|regional| Some((master, regional)))
    })
    .await
    .map_err(|e| AwareError::Internal(format!("trimble-connect region task join: {e}")))?
}

/// `url` (built on the `master` base) moved onto the `regional` base.
pub(crate) fn rebase(url: &str, master: &str, regional: &str) -> Result<String, AwareError> {
    url.strip_prefix(master)
        .filter(|rest| rest.is_empty() || rest.starts_with('/') || rest.starts_with('?'))
        .map(|rest| format!("{regional}{rest}"))
        .ok_or_else(|| {
            AwareError::Internal(format!(
                "trimble-connect: {url:?} is not under the Core base {master:?}"
            ))
        })
}

/// `out` with the resolved `region` added, so a downstream node can pass it on
/// (`region: "{{ prev.region }}"`). Nothing is added when the master was used
/// without knowing a location; an empty value downstream means "not given".
pub(super) fn with_region(mut out: Value, location: Option<&str>) -> Value {
    if let (Some(loc), Value::Object(m)) = (location, &mut out) {
        m.insert("region".into(), Value::String(loc.to_string()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::trimble_files::test_support::{mock_routed, next_request};
    use serde_json::json;

    fn url(s: &str) -> url::Url {
        url::Url::parse(s).unwrap()
    }

    #[test]
    fn only_the_published_https_core_hosts_get_the_token() {
        for h in CORE_HOSTS {
            assert!(
                prod_core_host(&url(&format!("https://{h}/tc/api/2.0/"))),
                "{h}"
            );
        }
        for bad in [
            "http://app21.connect.trimble.com/tc/api/2.0/",
            "https://app21.connect.trimble.com:8443/tc/api/2.0/",
            "https://user:pw@app21.connect.trimble.com/tc/api/2.0/",
            "https://app21.connect.trimble.com.evil.example/tc/api/2.0/",
            "https://evil.example/app21.connect.trimble.com/",
            "https://app99.connect.trimble.com/tc/api/2.0/",
            "https://open21.connect.trimble.com/",
            "https://APP21.connect.trimble.com.evil/",
        ] {
            assert!(!prod_core_host(&url(bad)), "{bad}");
        }
    }

    #[test]
    fn region_input_is_a_bare_location_name() {
        assert_eq!(region_arg(&json!({})).unwrap(), None);
        assert_eq!(region_arg(&json!({ "region": null })).unwrap(), None);
        assert_eq!(region_arg(&json!({ "region": "" })).unwrap(), None);
        assert_eq!(
            region_arg(&json!({ "region": "unitedKingdom" })).unwrap(),
            Some("unitedKingdom")
        );
        for bad in [
            json!({ "region": "europe/../x" }),
            json!({ "region": "evil.example" }),
            json!({ "region": "europe?x=1" }),
            json!({ "region": " europe" }),
            json!({ "region": 21 }),
            json!({ "region": "a".repeat(65) }),
        ] {
            let err = region_arg(&bad).unwrap_err().to_string();
            assert!(err.contains("`region`"), "{bad}: {err}");
        }
    }

    #[test]
    fn api_for_reads_the_named_field_of_the_matching_location() {
        let regions = json!([
            { "location": "northAmerica", "tc-api": "https://app.connect.trimble.com/tc/api/2.0/" },
            { "location": "europe", "tc-api": "https://app21.connect.trimble.com/tc/api/2.0/",
              "topic-api": "https://open21.connect.trimble.com/" }
        ]);
        assert_eq!(
            api_for(&regions, "europe", "tc-api"),
            Some("https://app21.connect.trimble.com/tc/api/2.0/")
        );
        assert_eq!(
            api_for(&regions, "europe", "topic-api"),
            Some("https://open21.connect.trimble.com/")
        );
        assert_eq!(api_for(&regions, "asia", "tc-api"), None);
        assert_eq!(api_for(&json!({}), "europe", "tc-api"), None);
    }

    #[test]
    fn with_region_adds_only_a_known_location() {
        assert_eq!(
            with_region(json!({ "a": 1 }), Some("europe")),
            json!({ "a": 1, "region": "europe" })
        );
        assert_eq!(with_region(json!({ "a": 1 }), None), json!({ "a": 1 }));
    }

    #[test]
    fn no_region_and_no_project_is_the_master_without_a_request() {
        // A dead base: any request would fail, so success proves none was made.
        let master = crate::runtime::trimble_files::test_support::dead_base();
        assert_eq!(
            resolve_core(&master, "t", None, None).unwrap(),
            (master.clone(), None)
        );
    }

    #[test]
    fn a_region_maps_through_public_regions_to_its_tc_api() {
        let (base, rx) = mock_routed(2, |b| {
            vec![(
                "GET /regions",
                200,
                format!(
                    r#"[{{"location":"northAmerica","tc-api":"https://app.connect.trimble.com/tc/api/2.0/"}},
                        {{"location":"europe","tc-api":"{b}/eu/"}}]"#
                ),
            )]
        });
        let (core, loc) = resolve_core(&base, "tok", Some("europe"), Some("ignored")).unwrap();
        assert_eq!(core, format!("{base}/eu"));
        assert_eq!(loc.as_deref(), Some("europe"));
        let req = next_request(&rx);
        assert!(!req.contains("Authorization"), "regions is public: {req}");
    }

    #[test]
    fn a_project_is_located_first_then_mapped() {
        let (base, rx) = mock_routed(3, |b| {
            vec![
                (
                    "GET /projects/P%201",
                    200,
                    r#"{"location":"europe"}"#.into(),
                ),
                (
                    "GET /regions",
                    200,
                    format!(r#"[{{"location":"europe","tc-api":"{b}/eu"}}]"#),
                ),
            ]
        });
        let (core, loc) = resolve_core(&base, "tok", None, Some("P 1")).unwrap();
        assert_eq!(core, format!("{base}/eu"));
        assert_eq!(loc.as_deref(), Some("europe"));
        let project = next_request(&rx);
        assert!(project.contains("Bearer tok"), "{project}");
    }

    #[test]
    fn a_project_the_master_does_not_know_is_found_in_its_own_region() {
        // As live: the master 404s a Europe project; Europe's host knows it.
        let (base, rx) = mock_routed(6, |b| {
            vec![
                (
                    "GET /eu/projects/P ",
                    200,
                    r#"{"location":"europe"}"#.into(),
                ),
                ("GET /projects/P ", 404, "{}".into()),
                (
                    "GET /regions",
                    200,
                    format!(
                        r#"[{{"location":"northAmerica","tc-api":"{b}/"}},
                            {{"location":"evil","tc-api":"https://evil.example/tc/api/2.0/"}},
                            {{"location":"europe","tc-api":"{b}/eu/"}}]"#
                    ),
                ),
            ]
        });
        let (core, loc) = resolve_core(&base, "tok", None, Some("P")).unwrap();
        assert_eq!(
            (core, loc.as_deref()),
            (format!("{base}/eu"), Some("europe"))
        );
        let seen: Vec<String> = [0; 4].iter().map(|_| next_request(&rx)).collect();
        let lines: Vec<&str> = seen.iter().map(|r| line(r)).collect();
        assert!(lines[0].starts_with("GET /projects/P "), "{lines:?}");
        assert!(lines[1].starts_with("GET /regions"), "{lines:?}");
        // The master is not asked twice and the unpublished host never at all.
        assert!(lines[2].starts_with("GET /eu/projects/P "), "{lines:?}");
        assert!(seen[2].contains("Bearer tok"), "{}", seen[2]);
        assert!(lines[3].starts_with("GET /regions"), "{lines:?}");
    }

    #[test]
    fn a_project_no_region_knows_is_an_error_naming_it() {
        let (base, _rx) = mock_routed(4, |b| {
            vec![
                ("GET /eu/projects/P ", 404, "{}".into()),
                ("GET /projects/P ", 404, "{}".into()),
                (
                    "GET /regions",
                    200,
                    format!(r#"[{{"location":"europe","tc-api":"{b}/eu/"}}]"#),
                ),
            ]
        });
        let err = resolve_core(&base, "tok", None, Some("P"))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("no Trimble Connect region knows project P"),
            "{err}"
        );
    }

    /// `GET /regions` naming the mock's `/eu` path as Europe's Core host.
    fn europe(b: &str) -> (&'static str, u16, String) {
        (
            "GET /regions",
            200,
            format!(
                r#"[{{"location":"northAmerica","tc-api":"{b}/"}},
                    {{"location":"europe","tc-api":"{b}/eu/"}}]"#
            ),
        )
    }

    fn line(req: &str) -> &str {
        req.lines().next().unwrap_or_default()
    }

    #[tokio::test]
    async fn an_id_only_command_with_a_region_goes_to_that_regions_host() {
        use crate::runtime::trimble_files::test_support::mock_agents;
        let (base, rx) = mock_routed(3, |b| {
            vec![europe(b), ("DELETE /eu/folders/F1 ", 204, String::new())]
        });
        let agents = mock_agents(&base);
        let out = crate::runtime::trimble_ops::invoke(
            agents.path().join("agents"),
            "delete-folder",
            json!({"folder-id": "F1", "region": "europe"}),
        )
        .await
        .unwrap();
        assert_eq!(
            out,
            json!({"folder-id": "F1", "deleted": true, "region": "europe"})
        );
        let regions = next_request(&rx);
        assert!(line(&regions).starts_with("GET /regions"), "{regions}");
        assert!(!regions.contains("Authorization"), "{regions}");
        let delete = next_request(&rx);
        assert!(
            line(&delete).starts_with("DELETE /eu/folders/F1 "),
            "{delete}"
        );
        assert!(delete.contains("Bearer TESTTOKEN"), "{delete}");
    }

    #[tokio::test]
    async fn upload_and_download_follow_the_region_too() {
        use crate::runtime::trimble_files::test_support::mock_agents;
        let (base, rx) = mock_routed(4, |b| {
            vec![
                europe(b),
                (
                    "GET /eu/files/fs/D1/downloadurl",
                    200,
                    format!(r#"{{"url":"{b}/s3/D1"}}"#),
                ),
                ("GET /s3/D1", 200, "hi".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let out = crate::runtime::trimble_files::download(
            agents.path().join("agents"),
            json!({"file-id": "D1", "region": "europe"}),
        )
        .await
        .unwrap();
        assert_eq!(out["region"], "europe");
        assert!(line(&next_request(&rx)).starts_with("GET /regions"));
        assert!(line(&next_request(&rx)).starts_with("GET /eu/files/fs/D1/downloadurl"));

        // A bad region is refused before the credential or any request.
        let err = crate::runtime::trimble_files::upload(
            agents.path().join("agents"),
            json!({"folder-id": "F", "filename": "a", "bytes": "aGk=", "region": "eu/x"}),
        )
        .await
        .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "{err}");
    }

    #[tokio::test]
    async fn a_declarative_read_with_a_region_is_rebased_and_sends_no_region_param() {
        use crate::runtime::invoker::{AgentInvoker as _, RestInvoker};
        use crate::runtime::trimble_files::test_support::agents_with;
        let (base, rx) = mock_routed(4, |b| {
            vec![
                europe(b),
                ("GET /eu/folders/F", 200, r#"{"id":"F"}"#.to_string()),
                ("GET /folders/F", 200, r#"{"id":"F"}"#.to_string()),
            ]
        });
        let agents = agents_with(
            &format!(
                "agent: trimble-connect\nversion: 0.1.0\ndescription: x\nstateful: false\n\
                 license: MIT\ntransport:\n  rest:\n    base: {base}/\nauth:\n  scheme: oauth2\n  \
                 secret: mock-tc-tok\ncommands:\n  get-folder:\n    lifecycle: single\n    \
                 description: x\n    method: GET\n    path: /folders/{{folder-id}}\n    inputs:\n      \
                 folder-id:\n        type: string\n        in: path\n      region:\n        \
                 type: string\n        required: false\n"
            ),
            Some("mock-tc-tok"),
        );
        let inv = RestInvoker {
            catalogue: agents.path().join("agents").into(),
        };
        let out = inv
            .invoke_single(
                "trimble-connect",
                "get-folder",
                json!({"folder-id": "F", "region": "europe"}),
            )
            .await
            .unwrap();
        assert_eq!(out["body"]["id"], "F");
        assert!(line(&next_request(&rx)).starts_with("GET /regions"));
        let get = next_request(&rx);
        assert_eq!(line(&get), "GET /eu/folders/F HTTP/1.1", "{get}");
        assert!(get.contains("Bearer TESTTOKEN"), "{get}");

        // No region: the master, unchanged — and no lookup.
        inv.invoke_single("trimble-connect", "get-folder", json!({"folder-id": "F"}))
            .await
            .unwrap();
        assert_eq!(line(&next_request(&rx)), "GET /folders/F HTTP/1.1");
    }

    #[test]
    fn rebase_moves_only_a_url_under_the_master() {
        let m = "https://app.connect.trimble.com/tc/api/2.0";
        let r = "https://app21.connect.trimble.com/tc/api/2.0";
        assert_eq!(
            rebase(&format!("{m}/folders/F"), m, r).unwrap(),
            format!("{r}/folders/F")
        );
        assert_eq!(
            rebase(&format!("{m}?x=1"), m, r).unwrap(),
            format!("{r}?x=1")
        );
        assert!(rebase(&format!("{m}x/folders"), m, r).is_err());
        assert!(rebase("https://elsewhere.example/folders", m, r).is_err());
    }

    #[test]
    fn an_unlisted_region_or_an_unknown_host_is_refused() {
        let (base, _rx) = mock_routed(2, |_| {
            vec![(
                "GET /regions",
                200,
                r#"[{"location":"europe","tc-api":"https://app21.connect.trimble.com/tc/api/2.0/"}]"#
                    .into(),
            )]
        });
        let err = regional_core(&base, "mars").unwrap_err().to_string();
        assert!(err.contains("lists no region \"mars\""), "{err}");

        let (base, _rx) = mock_routed(2, |_| {
            vec![(
                "GET /regions",
                200,
                r#"[{"location":"europe","tc-api":"https://evil.example/tc/api/2.0/"}]"#.into(),
            )]
        });
        let err = regional_core(&base, "europe").unwrap_err().to_string();
        assert!(err.contains("token is not sent there"), "{err}");
    }
}
