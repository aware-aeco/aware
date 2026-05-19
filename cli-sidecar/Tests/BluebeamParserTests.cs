// Tests for the Bluebeam PageParser — Postman v2.1 collection → host-coverage IR.
//
// Fixture: bluebeam-mini-collection.json — a hand-crafted excerpt of the real
// Bluebeam Studio API collection that exercises each structural feature the
// parser handles:
//
//   • Top-level + subfolder nesting (Sessions / Files, Sessions / Single Session,
//     Sessions / (anonymous spacer)) — anonymous spacer + "Single ..." both fold
//     up into the top-level *Api class.
//   • POST with JSON body (synthesised Request DTO with property type inference).
//   • GET / DELETE without body (no DTO, no body param).
//   • Path-parameter token extraction with type mapping (sessionId → Guid,
//     fileId → int, id → Guid in the absence of stronger context).
//   • Idiomatic method name synthesis from verb + path (CreateSessionFile,
//     ListSessionFiles, CreateFileSnapshot, GetFile, DeleteFile, GetMe, etc.).
//   • Special-case action segments mapped to verbs (invite → Invite, cancel →
//     Cancel) AND mapped to nouns (snapshot stays as suffix).
//
// The parser must produce a stable, deterministic IR for the same input — this
// is the foundation tests pin.

using AwareSidecar.Ingest.Extractors.Bluebeam;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class BluebeamParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(BluebeamParserTests).Assembly.Location)!;
        var path = Path.Combine(here, "fixtures", name);
        if (!File.Exists(path))
        {
            // Walk up to find the source tree (CI vs local-dev layouts).
            var walk = here;
            for (int i = 0; i < 6 && walk is not null; i++)
            {
                var probe = Path.Combine(walk, "Tests", "fixtures", name);
                if (File.Exists(probe)) { path = probe; break; }
                walk = Path.GetDirectoryName(walk);
            }
        }
        return File.ReadAllText(path);
    }

    // ── PageParser.ParseCollection end-to-end ──────────────────────────────

    [Fact]
    public void ParseCollection_EmitsExpectedClasses()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);

        // Three top-level folders in the fixture: Sessions / Jobs / Misc.
        // SessionsApi (rolls up the anonymous spacer's POST/GET + Single Session's three ops)
        // SessionsFilesApi (Files subfolder — 5 ops)
        // JobsApi (Job subfolder folds up — 2 ops)
        // ProjectFileJobsApi (Project File Jobs subfolder — 1 op; class name strips redundant "Jobs" prefix)
        // MiscApi (Misc — 2 direct ops)
        Assert.Contains(types, t => t.name == "SessionsApi" && t.@namespace == "Bluebeam.StudioApi.Sessions" && t.kind == "class");
        Assert.Contains(types, t => t.name == "SessionsFilesApi" && t.@namespace == "Bluebeam.StudioApi.Sessions");
        Assert.Contains(types, t => t.name == "JobsApi" && t.@namespace == "Bluebeam.StudioApi.Jobs");
        Assert.Contains(types, t => t.name == "ProjectFileJobsApi" && t.@namespace == "Bluebeam.StudioApi.Jobs");
        Assert.Contains(types, t => t.name == "MiscApi" && t.@namespace == "Bluebeam.StudioApi.Misc");
    }

    [Fact]
    public void ParseCollection_SessionsApi_HasSingleSessionOpsAndRoot()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var sessions = types.First(t => t.name == "SessionsApi");

        // SessionsApi rolls up: top-level POST + top-level GET + Single Session
        // (GetSession / DeleteSession / InviteSession). 5 total.
        var names = sessions.methods.Select(m => m.name).OrderBy(s => s).ToList();
        Assert.Contains("CreateSession", names);
        Assert.Contains("ListSessions", names);
        Assert.Contains("GetSession", names);
        Assert.Contains("DeleteSession", names);
        Assert.Contains("InviteSession", names);
        Assert.Equal(5, sessions.methods.Count);
    }

    [Fact]
    public void ParseCollection_SessionsFilesApi_HasCorrectMethodNames()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var files = types.First(t => t.name == "SessionsFilesApi");

        var names = files.methods.Select(m => m.name).OrderBy(s => s).ToList();
        // POST /sessions/{sessionId}/files       → CreateSessionFile
        Assert.Contains("CreateSessionFile", names);
        // GET  /sessions/{sessionId}/files       → ListSessionFiles
        Assert.Contains("ListSessionFiles", names);
        // GET  /sessions/{sessionId}/files/{id}  → GetFile
        Assert.Contains("GetFile", names);
        // POST /sessions/{sessionId}/files/{id}/snapshot → CreateFileSnapshot
        Assert.Contains("CreateFileSnapshot", names);
        // DELETE /sessions/{sessionId}/files/{id}        → DeleteFile
        Assert.Contains("DeleteFile", names);
    }

    [Fact]
    public void ParseCollection_PathParams_TypedAsGuidOrInt()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var files = types.First(t => t.name == "SessionsFilesApi");
        var snapshot = files.methods.First(m => m.name == "CreateFileSnapshot");
        // Path params: sessionId (Guid) + id (Guid by default mapping)
        Assert.Equal(2, snapshot.@params.Count);
        Assert.Equal("sessionId", snapshot.@params[0].name);
        Assert.Equal("Guid", snapshot.@params[0].type);
        Assert.Equal("id", snapshot.@params[1].name);
        Assert.Equal("Guid", snapshot.@params[1].type);

        // Project file jobs use fileId (int) — Project file ids are integers in Bluebeam.
        var projectFileJobs = types.First(t => t.name == "ProjectFileJobsApi");
        var dwg = projectFileJobs.methods.First();
        var projectIdParam = dwg.@params.First(p => p.name == "projectId");
        var fileIdParam = dwg.@params.First(p => p.name == "fileId");
        Assert.Equal("Guid", projectIdParam.type);
        Assert.Equal("int", fileIdParam.type);
    }

    [Fact]
    public void ParseCollection_RequestBody_SynthesizesDtoWithInferredTypes()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);

        // CreateSession body: { Name (string), Notification (bool), Restricted (bool), SessionEndDate (string) }
        // The DTO is named "CreateSessionRequest" and lives in the .Models sub-namespace.
        var dto = types.First(t => t.name == "CreateSessionRequest" && t.@namespace == "Bluebeam.StudioApi.Sessions.Models");
        Assert.Equal("class", dto.kind);
        Assert.Equal(4, dto.properties.Count);
        Assert.Equal("string", dto.properties.First(p => p.name == "Name").type);
        Assert.Equal("bool", dto.properties.First(p => p.name == "Notification").type);
        Assert.Equal("bool", dto.properties.First(p => p.name == "Restricted").type);
        Assert.Equal("string", dto.properties.First(p => p.name == "SessionEndDate").type);

        // The CreateSession METHOD references CreateSessionRequest as its `request` param.
        var sessions = types.First(t => t.name == "SessionsApi");
        var createSession = sessions.methods.First(m => m.name == "CreateSession");
        Assert.Contains(createSession.@params, p => p.name == "request" && p.type == "CreateSessionRequest");
    }

    [Fact]
    public void ParseCollection_GetEndpoints_HaveNoBodyParam()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var sessions = types.First(t => t.name == "SessionsApi");
        var list = sessions.methods.First(m => m.name == "ListSessions");
        // GET /sessions has no path params and no body.
        Assert.Empty(list.@params);
    }

    [Fact]
    public void ParseCollection_ActionAsImperative_ChangesVerbPrefix()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var sessions = types.First(t => t.name == "SessionsApi");

        // POST /sessions/{sessionId}/invite → InviteSession (action "invite" treated as a verb,
        // resource "sessions" singular → Session).
        Assert.Contains(sessions.methods, m => m.name == "InviteSession");

        // PUT /jobs/{id}/cancel → CancelJob (action "cancel" treated as a verb).
        var jobs = types.First(t => t.name == "JobsApi");
        Assert.Contains(jobs.methods, m => m.name == "CancelJob");
    }

    [Fact]
    public void ParseCollection_MiscApi_GetUserMe_IsHandled()
    {
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var misc = types.First(t => t.name == "MiscApi");
        Assert.Contains(misc.methods, m => m.name == "GetUserMe");
        Assert.Contains(misc.methods, m => m.name == "ListVersion");
    }

    [Fact]
    public void ParseCollection_AllTypesHaveSchemaRequiredFields()
    {
        // Every TypeInfo emitted must satisfy the schema's REQUIRED fields:
        // name, namespace, kind, summary, doc_url, interfaces, constructors,
        // methods, properties, events, enum_values. Missing-field IRs cause
        // the catalog writer + schema validator to reject the entire IR.
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);

        Assert.NotEmpty(types);
        foreach (var t in types)
        {
            Assert.False(string.IsNullOrEmpty(t.name), $"type missing name");
            Assert.False(string.IsNullOrEmpty(t.@namespace), $"{t.name}: namespace missing");
            Assert.False(string.IsNullOrEmpty(t.kind), $"{t.name}: kind missing");
            Assert.False(string.IsNullOrEmpty(t.summary), $"{t.name}: summary missing");
            Assert.False(string.IsNullOrEmpty(t.doc_url), $"{t.name}: doc_url missing");
            Assert.NotNull(t.interfaces);
            Assert.NotNull(t.constructors);
            Assert.NotNull(t.methods);
            Assert.NotNull(t.properties);
            Assert.NotNull(t.events);
            Assert.NotNull(t.enum_values);

            // Per-method schema requirements.
            foreach (var m in t.methods)
            {
                Assert.False(string.IsNullOrEmpty(m.name), $"{t.name}.{m.name}: method name missing");
                Assert.False(string.IsNullOrEmpty(m.signature), $"{t.name}.{m.name}: signature missing");
                Assert.False(string.IsNullOrEmpty(m.summary), $"{t.name}.{m.name}: summary missing");
                Assert.False(string.IsNullOrEmpty(m.doc_url), $"{t.name}.{m.name}: doc_url missing");
                // returns may be null (void) — that's schema-legal.
                Assert.NotNull(m.@params);
                Assert.NotNull(m.throws);
                Assert.NotNull(m.events_related);
            }

            // Per-property schema requirements.
            foreach (var p in t.properties)
            {
                Assert.False(string.IsNullOrEmpty(p.name), $"{t.name}.{p.name}: property name missing");
                Assert.False(string.IsNullOrEmpty(p.type), $"{t.name}.{p.name}: property type missing");
                Assert.False(string.IsNullOrEmpty(p.summary), $"{t.name}.{p.name}: property summary missing");
                Assert.False(string.IsNullOrEmpty(p.doc_url), $"{t.name}.{p.name}: property doc_url missing");
            }
        }
    }

    [Fact]
    public void ParseCollection_RequestDtoNamesAreUnique()
    {
        // Synthesised Request DTOs must have unique (namespace, name) — duplicate types
        // would produce duplicate catalog files (Windows filesystem would overwrite, no error,
        // silent loss of one of the type definitions).
        var json = LoadFixture("bluebeam-mini-collection.json");
        var types = PageParser.ParseCollection(json);
        var keys = types.Select(t => $"{t.@namespace}.{t.name}").ToList();
        Assert.Equal(keys.Count, keys.Distinct().Count());
    }

    // ── TryParseBodySchema (the body-schema synthesis primitive) ───────────

    [Fact]
    public void TryParseBodySchema_StringIntBool_InfersTypes()
    {
        var raw = """{"Name":"","Count":0,"Enabled":true,"Ratio":1.5}""";
        var (dto, names) = PageParser.TryParseBodySchema(raw, "FooRequest", "Bluebeam.X");
        Assert.NotNull(dto);
        Assert.Equal(new[] { "Name", "Count", "Enabled", "Ratio" }, names);
        var byName = dto!.properties.ToDictionary(p => p.name, p => p.type);
        Assert.Equal("string", byName["Name"]);
        Assert.Equal("int", byName["Count"]);
        Assert.Equal("bool", byName["Enabled"]);
        Assert.Equal("double", byName["Ratio"]);
    }

    [Fact]
    public void TryParseBodySchema_Array_InfersListType()
    {
        var raw = """{"FileIds":[1,2,3]}""";
        var (dto, _) = PageParser.TryParseBodySchema(raw, "FooRequest", "Bluebeam.X");
        Assert.NotNull(dto);
        var prop = dto!.properties[0];
        Assert.Equal("FileIds", prop.name);
        Assert.Equal("List<int>", prop.type);
    }

    [Fact]
    public void TryParseBodySchema_InvalidJson_ReturnsNull()
    {
        var (dto, names) = PageParser.TryParseBodySchema("not-json-at-all", "FooRequest", "Bluebeam.X");
        Assert.Null(dto);
        Assert.Empty(names);
    }

    [Fact]
    public void TryParseBodySchema_EmptyObject_ProducesDtoWithNoProperties()
    {
        var (dto, names) = PageParser.TryParseBodySchema("{}", "FooRequest", "Bluebeam.X");
        Assert.NotNull(dto);
        Assert.Empty(names);
        Assert.Empty(dto!.properties);
    }
}
