---
name: rhino-common-rhino-runtime-notifications
description: API reference for namespace Rhino.Runtime.Notifications from RhinoCommon.dll
---

# Rhino.Runtime.Notifications

- **ButtonType**
  - The type of button in a notification.
- **IAssemblyRestrictedObject**
  - A class that implements this interface signals its clients that its instances can             only be modified by certain assemblies. This is useful in cases where only             certain assemblies should be able to modify an object. The actual members of an              instance that are restricted are left to the discretion of the instance's class,             and should be documented.
- **Notification**
  - A Notification instance can be used to inform the user about various events. For             a Notification instance to be displayed in Rhino, it must be added to the             . When added, it will be displayed in the             Notifications panel in Rhino. A Notification contains 1 to 3 buttons that are              automatically wired to its  Action if it is not null.              The buttons are displayed when the Notification is shown modally by either the user              clicking on a particular notification in the Notifications panel, or by programatically              showing it using .                          Currently, only process-wide notifications are             supported; document specific notifications are not possible.                          Notification instances contain metadata that can be added, modified, or removed during             its life. The metadata is important for LINQ queries and other patterns.             For example, a particular action may require that multiple notifications be modified.              Thus, a LINQ query can be performed on the  using metadata             to retrieve related Notification objects and modify them as a batch.                          Notification objects implement . By default, a              Notification can be editedby any assembly, but explicitly specifing allowed assemblies              in the constructor changes this behavior.                          Notification objects are not thread-safe and should only be manipulated in UI thread.
- **NotificationButtonClickedArgs**
  - Used when a button is clicked for a notification.
- **NotificationCenter**
  - The NotificationCenter holds all  objects that are displayed in the Notifications panel by Rhino.                          The NotificationCenter is not thread-safe and should only be used in the UI thread.
- **TrulyObservableOrderedSet`1**
  - An ordered set that notifies its subscribers whenever one of its INotifyPropertyChanged elements raises its PropertyChanged event.
