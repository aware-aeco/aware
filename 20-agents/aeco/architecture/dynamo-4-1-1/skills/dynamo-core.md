---
name: dynamo-applications-dynamo-core
description: API reference for namespace Dynamo.Core from DynamoCore.dll
---

# Dynamo.Core

- **AuthenticationManager**
  - This is a wrapper for  functionality.                 It is used for oxygen authentication.
- **CrashErrorReportArgs**
  - Event argument for CER (crash error reporting) tool.             It contains options on what to send out with the crash report.
- **CrashPromptArgs**
  - Event argument for CrashPrompt. It contains display options, details, overriding text and file path.
- **CustomNodeManager**
  - Manages instantiation of custom nodes.  All custom nodes known to Dynamo should be stored                 with this type.  This object implements late initialization of custom nodes by providing a                  single interface to initialize custom nodes.
- **IDSDKManager**
  - The class to provide auth APIs for IDSDK related methods.
- **NotificationObject**
  - This class notifies the View when there is a change.
