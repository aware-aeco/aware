# exporter-ifcutils-create-sub-element-guid

Lifecycle: single

Creates a consistent GUID for an IFC entity related to a Revit element.    A "related" sub-element is one that is unique for a given type of element, and can    therefore by identified by a simple index value (e.g. PSet_Wall_Common property set for a wall.)    The index value 0 is reserved, as this would generate the GUID of the element itself.    A listing of known sub-elements is contained in IFCSubElementEnums.cs; it is    expected that this list would be maintained up-to-date, instead of passing arbitrary values    into this function.
