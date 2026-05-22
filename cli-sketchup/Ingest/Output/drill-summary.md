# sketchup.exec drill - 2026-05-22 15:31:59

**Sidecar:** .\cli-sketchup\bin\Release\net10.0-windows\win-x64\aware-sketchup.exe
**Fixtures:** C:\Users\bimst\source\repos\aware\cli-sketchup\Ingest\Output
**Result:** 19 PASS, 1 FAIL of 20

| # | Status | Exit | Result snippet |
|---|---|---|---|
| prompt-01 | **PASS** | 0 | `{"ok":true,"result":{"total":1,"by_class":{"Sketchup::ComponentInstance":1}},"host":"sketchup","host_version":"26.1.189"` |
| prompt-02 | **PASS** | 0 | `{"ok":true,"result":{"count":0,"ids":[]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704` |
| prompt-03 | **PASS** | 0 | `{"ok":true,"result":{"valid":true,"min":[-83.63740047057117,-72.38874531912919,0.0],"max":[8.326772728431072,19.57542787` |
| prompt-04 | **PASS** | 0 | `{"ok":true,"result":{"count":1,"materials":[{"name":"Material","rgb":[159,153,146]}]},"host":"sketchup","host_version":"` |
| prompt-05 | **PASS** | 0 | `{"ok":true,"result":{"count":1,"layers":[{"name":"Layer0","visible":true}]},"host":"sketchup","host_version":"26.1.189",` |
| prompt-06 | **PASS** | 0 | `{"ok":true,"result":{"added":true,"id":"114992"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_i` |
| prompt-07 | **PASS** | 0 | `{"ok":true,"result":{"edge_count":24,"first_id":"114997"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"s` |
| prompt-08 | **PASS** | 0 | `{"ok":true,"result":{"added":true,"id":"115049","area":775.0015500031001},"host":"sketchup","host_version":"26.1.189","h` |
| prompt-09 | **PASS** | 0 | `{"ok":true,"result":{"group_id":"115051","point_count":5},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"s` |
| prompt-10 | **PASS** | 0 | `{"ok":true,"result":{"name":"AWARE","visible":true,"color":[116,80,25]},"host":"sketchup","host_version":"26.1.189","hos` |
| prompt-11 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":47` |
| prompt-12 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":47` |
| prompt-13 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":47` |
| prompt-14 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no selected groups or components to explode"},"host":"sketchup","host_version":` |
| prompt-15 | **PASS** | 0 | `{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":47` |
| prompt-16 | **PASS** | 0 | `{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.skp"},"host":"sketchup","ho` |
| prompt-17 | **PASS** | 0 | `{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.dae","entity_count":32},"ho` |
| prompt-18 | **PASS** | 0 | `{"ok":true,"result":{"count":0,"pages":[]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"47` |
| prompt-19 | **FAIL** | 0 | `{"ok":false,"error":"NoMethodError: undefined method \u0060rendering_options\u0027 for #\u003CSketchup::View:0x000002572` |
| prompt-20 | **PASS** | 0 | `{"ok":true,"result":{"guid":"3598ab80-6473-419a-be95-99a0e91cf0b6","title":"aware-drill","description":"","path":"C:\\Us` |

## Full receipts

### prompt-01 - PASS

```json
{"ok":true,"result":{"total":1,"by_class":{"Sketchup::ComponentInstance":1}},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:29:58.5657633Z"}
```

### prompt-02 - PASS

```json
{"ok":true,"result":{"count":0,"ids":[]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:02.7483203Z"}
```

### prompt-03 - PASS

```json
{"ok":true,"result":{"valid":true,"min":[-83.63740047057117,-72.38874531912919,0.0],"max":[8.326772728431072,19.57542787987306,72.68476217425663]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:08.6105314Z"}
```

### prompt-04 - PASS

```json
{"ok":true,"result":{"count":1,"materials":[{"name":"Material","rgb":[159,153,146]}]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:13.8506186Z"}
```

### prompt-05 - PASS

```json
{"ok":true,"result":{"count":1,"layers":[{"name":"Layer0","visible":true}]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:32.4274401Z"}
```

### prompt-06 - PASS

```json
{"ok":true,"result":{"added":true,"id":"114992"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:35.6594017Z"}
```

### prompt-07 - PASS

```json
{"ok":true,"result":{"edge_count":24,"first_id":"114997"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:43.4867663Z"}
```

### prompt-08 - PASS

```json
{"ok":true,"result":{"added":true,"id":"115049","area":775.0015500031001},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:47.2592182Z"}
```

### prompt-09 - PASS

```json
{"ok":true,"result":{"group_id":"115051","point_count":5},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:50.0590937Z"}
```

### prompt-10 - PASS

```json
{"ok":true,"result":{"name":"AWARE","visible":true,"color":[116,80,25]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:57.4457101Z"}
```

### prompt-11 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:30:58.8365583Z"}
```

### prompt-12 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:07.4647339Z"}
```

### prompt-13 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:12.3989081Z"}
```

### prompt-14 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no selected groups or components to explode"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:23.0716929Z"}
```

### prompt-15 - PASS

```json
{"ok":true,"result":{"ok":false,"error":"no objects selected"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:28.1581874Z"}
```

### prompt-16 - PASS

```json
{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.skp"},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:34.1421060Z"}
```

### prompt-17 - PASS

```json
{"ok":true,"result":{"ok":true,"path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1/aware-drill.dae","entity_count":32},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:45.7147413Z"}
```

### prompt-18 - PASS

```json
{"ok":true,"result":{"count":0,"pages":[]},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:49.3093949Z"}
```

### prompt-19 - FAIL

```json
{"ok":false,"error":"NoMethodError: undefined method \u0060rendering_options\u0027 for #\u003CSketchup::View:0x0000025723c1e3b0\u003E","stack":"(aware-exec):3:in \u0060block in run_user_script\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:247:in \u0060run_user_script\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:212:in \u0060process_one\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:202:in \u0060drain_inbox\u0027\nC:/Users/bimst/AppData/Roaming/SketchUp/SketchUp 2026/SketchUp/Plugins/aware_sketchup_bridge/core.rb:75:in \u0060block in start!\u0027","host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:52.1392719Z"}
```

### prompt-20 - PASS

```json
{"ok":true,"result":{"guid":"3598ab80-6473-419a-be95-99a0e91cf0b6","title":"aware-drill","description":"","path":"C:\\Users\\bimst\\AppData\\Local\\TEMP_~1\\aware-drill.skp","modified":false},"host":"sketchup","host_version":"26.1.189","host_pid":4704,"sketchup_id":"4704","verb":"exec","stdout_log":"","delivered_at":"2026-05-22T13:31:59.5490056Z"}
```

