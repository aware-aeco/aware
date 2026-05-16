---
name: rhino-common-rhino-ui-controls
description: API reference for namespace Rhino.UI.Controls from RhinoCommon.dll
---

# Rhino.UI.Controls

- **CollapsibleSectionHolderImpl**
- **CollapsibleSectionImpl**
- **CollapsibleSectionViewModel**
  - Derive from this class to implement your own view model that wraps around a built             in implementation of IRdkViewModel.  Use GetData etc to implement your properties.
- **ContentUI**
  - This class represents the user interface (UI) of a content. A content's UI appears inside             an editor which is represented by IRhRdkContentEditor.The UI is the part of the editor which             allows the user to view and modify the persistent state of the selected content(s).             This interface is implemented by RDK internally in the core content class, CRhRdkCoreContent.             If you do not override CRhRdkCoreContent::CreateUI() you do not have to concern yourself with             the details.If however, you do want to override CreateUI() to create your own custom UI, then             you must derive your UI class from CRhRdkCustomContentUI and implement this interface to support             your own UI design. \note CRhRdkCustomContentUI implements some of this interface for you.
- **Delegates**
- **ExpandableContentUI**
  - This class extends ContentUI to represent the default user interface which             the RDK provides for a content.This is the UI created by the default implementation             of CRhRdkCoreContent::CreateUI(). The main feature of this implementation is a 'holder'             containing one or more expandable 'sections' (AKA 'roll-ups').
- **FactoryBase**
  - Base class for CollapsibleSection and ViewModel factories used by the RDK UI
- **ICollapsibleSection**
- **ICollapsibleSection2**
- **ICollapsibleSection3**
- **ICollapsibleSectionHolder**
- **ICollapsibleSectionHolder2**
- **IHasCppImplementation**
- **IHeaderButtonHandler**
- **IRdkViewModel**
- **IWindow**
- **InternalRdkViewModelFactory**
- **UndoRecord**
  - Undo Record
