---
name: grasshopper-grasshopper-gui-remote-panel
description: API reference for namespace Grasshopper.GUI.RemotePanel from Grasshopper.dll
---

# Grasshopper.GUI.RemotePanel

- **IRcpItem**
  - Implement this interface if you want to be part of a Remote-Control-Panel.
- **RcpDeadItem**
  - Item implementation for missing targets.
- **RcpDocumentObjectItem`1**
  - Base implementation of RcpItem meant for document object proxies.
- **RcpGroup**
  - Represents a container for GH_Object instances.   Containers are top-level objects in the RCP.
- **RcpIndex**
  - Contains the indices needed to identify an item on a Remote Control Panel.
- **RcpItem**
  - Base implementation of IRcpItem
- **RcpLabelItem**
  - Label item implementation.
- **RcpLayout**
  - Represents a collection of RcpGroup objects.
- **RcpLayoutControl**
- **RcpLayoutEventArgs**
  - Event arguments used in the LayoutChanged event for RcpGroupLayout.
- **RcpLayoutKind**
  - Enumerates the possible types of layout changes.
- **RcpPlaceHolder**
  - Placeholder class during node drags.
- **RcpSeparatorItem**
  - Separator item implementation.
- **RemoteControlPanel**
  - Remote Control Panel control. This control hosts the Rcp toolbar and also the group content panel.
