# sketchup.exec drill - 2026-05-20 00:17:29

**Sidecar:** C:\Users\bimst\source\repos\aware\.claude\worktrees\agent-adcdcaaf9ca4f1881\cli-sketchup\bin\Release\net10.0-windows\win-x64\aware-sketchup.exe
**Fixtures:** C:\Users\bimst\source\repos\aware\.claude\worktrees\agent-adcdcaaf9ca4f1881\cli-sketchup\Ingest\Output
**Result:** 19 PASS, 1 FAIL of 20

| # | Status | Exit | Result snippet |
|---|---|---|---|
| prompt-01 | **PASS** | 0 | `{"ok":true,"result":{"total":1,"by_class":{"Sketchup::ComponentInstance":1}},"host":"sketchup","host_version":"26.1.189"` |
| prompt-02 | **PASS** | 0 | `{"ok":true,"result":{"count":0,"ids":[]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"231` |
| prompt-03 | **PASS** | 0 | `{"ok":true,"result":{"valid":true,"min":[-83.63740047057117,-72.38874531912919,0.0],"max":[8.326772728431072,19.57542787` |
| prompt-04 | **PASS** | 0 | `{"ok":true,"result":{"count":1,"materials":[{"name":"Material","rgb":[159,153,146]}]},"host":"sketchup","host_version":"` |
| prompt-05 | **PASS** | 0 | `{"ok":true,"result":{"count":1,"layers":[{"name":"Layer0","visible":true}]},"host":"sketchup","host_version":"26.1.189",` |
| prompt-06 | **PASS** | 0 | `{"ok":true,"result":{"added":true,"id":"114992"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_` |
| prompt-07 | **PASS** | 0 | `{"ok":true,"result":{"edge_count":24,"first_id":"114997"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"` |
| prompt-08 | **PASS** | 0 | `{"ok":true,"result":{"added":true,"id":"115049","area":775.0015500031001},"host":"sketchup","host_version":"26.1.189","h` |
| prompt-09 | **PASS** | 0 | `{"ok":true,"result":{"group_id":"115051","point_count":5},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"` |
| prompt-10 | **PASS** | 0 | `{"ok":true,"result":{"name":"AWARE","visible":true,"color":[37,238,94]},"host":"sketchup","host_version":"26.1.189","hos` |
| prompt-11 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23` |
| prompt-12 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23` |
| prompt-13 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23` |
| prompt-14 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no selected groups or components to explode"},"host":"sketchup","host_version":` |
| prompt-15 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23` |
| prompt-16 | **PASS** | 0 | `{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.skp"},"host":"sketchup","ho` |
| prompt-17 | **PASS** | 0 | `{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.dae","entity_count":32},"ho` |
| prompt-18 | **PASS** | 0 | `{"ok":true,"result":{"count":0,"pages":[]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"2` |
| prompt-19 | **FAIL** | 0 | `{"ok":false,"error":"NoMethodError: undefined method \u0060rendering_options\u0027 for #\u003CSketchup::View:0x000001af7` |
| prompt-20 | **PASS** | 0 | `{"ok":true,"result":{"guid":"9b4a8a6b-29d4-4803-994a-5ac639c1bb94","title":"aware-drill","description":"","path":"C:\\Us` |

## Full receipts

### prompt-01 - PASS

```json
{"ok":true,"result":{"total":1,"by_class":{"Sketchup::ComponentInstance":1}},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:14.3637289Z"}
```

### prompt-02 - PASS

```json
{"ok":true,"result":{"count":0,"ids":[]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:18.3815653Z"}
```

### prompt-03 - PASS

```json
{"ok":true,"result":{"valid":true,"min":[-83.63740047057117,-72.38874531912919,0.0],"max":[8.326772728431072,19.57542787987306,72.68476217425663]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:24.2319678Z"}
```

### prompt-04 - PASS

```json
{"ok":true,"result":{"count":1,"materials":[{"name":"Material","rgb":[159,153,146]}]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:31.4832529Z"}
```

### prompt-05 - PASS

```json
{"ok":true,"result":{"count":1,"layers":[{"name":"Layer0","visible":true}]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:41.3745279Z"}
```

### prompt-06 - PASS

```json
{"ok":true,"result":{"added":true,"id":"114992"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:46.4825762Z"}
```

### prompt-07 - PASS

```json
{"ok":true,"result":{"edge_count":24,"first_id":"114997"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:15:56.4575133Z"}
```

### prompt-08 - PASS

```json
{"ok":true,"result":{"added":true,"id":"115049","area":775.0015500031001},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:10.1216790Z"}
```

### prompt-09 - PASS

```json
{"ok":true,"result":{"group_id":"115051","point_count":5},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:16.0759730Z"}
```

### prompt-10 - PASS

```json
{"ok":true,"result":{"name":"AWARE","visible":true,"color":[37,238,94]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:22.4689349Z"}
```

### prompt-11 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:27.1119780Z"}
```

### prompt-12 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:33.3076789Z"}
```

### prompt-13 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:40.5218196Z"}
```

### prompt-14 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no selected groups or components to explode"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:42.1495749Z"}
```

### prompt-15 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:50.2939314Z"}
```

### prompt-16 - PASS

```json
{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.skp"},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:16:59.2535740Z"}
```

### prompt-17 - PASS

```json
{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.dae","entity_count":32},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:17:08.9023862Z"}
```

### prompt-18 - PASS

```json
{"ok":true,"result":{"count":0,"pages":[]},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:17:17.1547228Z"}
```

### prompt-19 - FAIL

```json
{"ok":false,"error":"NoMethodError: undefined method \u0060rendering_options\u0027 for #\u003CSketchup::View:0x000001af7bb1f450\u003E","stack":"(aware-exec):3:in \u0060block in run_user_script\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:247:in \u0060run_user_script\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:212:in \u0060process_one\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:202:in \u0060drain_inbox\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:75:in \u0060block in start!\u0027","host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:17:24.1869034Z"}
```

### prompt-20 - PASS

```json
{"ok":true,"result":{"guid":"9b4a8a6b-29d4-4803-994a-5ac639c1bb94","title":"aware-drill","description":"","path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1\\aware-drill.skp","modified":false},"host":"sketchup","host_version":"26.1.189","host_pid":23176,"sketchup_id":"23176","verb":"exec","stdout_log":"","delivered_at":"2026-05-19T22:17:29.3552194Z"}
```

