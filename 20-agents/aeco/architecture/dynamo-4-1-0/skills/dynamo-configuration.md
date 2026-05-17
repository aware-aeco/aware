---
name: dynamo-applications-dynamo-configuration
description: API reference for namespace Dynamo.Configuration from DynamoCore.dll
---

# Dynamo.Configuration

- **Configurations**
  - This class contains properties that are used in Dynamo.
- **Context**
  - This specifies the Dynamo Context (RunTime). ex : Revit 2014, Revit 2015 etc.
- **DebugSettings**
  - This class is used for setting debug settings through Dynamo UI.             E.g. turn on/off logging; show/hide compiled node values.
- **DynamoPlayerFolder**
  - This class describes a folder (usually containing Dynamo graphs) added to the Dynamo Player or Generative Design
- **DynamoPlayerFolderGroup**
  - This class defines a group of folders associated with a Dynamo Player or Generative Design entry point.
- **GraphChecksumItem**
  - Represents the stringified version of the nodes connections from a graph
- **GroupStyleItem**
  - Group specific style item             Note: This class does not contain special property yet comparing to base class
- **PreferenceSettings**
  - PreferenceSettings is a class for GUI to persist certain settings.             Upon running of the GUI, those settings that are persistent will be loaded             from a XML file from DYNAMO_SETTINGS_FILE.             When GUI is closed, the settings are saved back into the XML file.
- **StyleItem**
  - This class stores the group styles added by the user
- **ViewExtensionDisplayMode**
  - Possible display modes for an extension UI control.
- **ViewExtensionSettings**
  - Settings that apply to a view extension specifically.
- **WindowSettings**
  - Settings that define how to display an extension control in floating window mode.
- **WindowStatus**
  - Possible status of a floating window.
