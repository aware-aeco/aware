# catalog-item-from-file

Lifecycle: single

Finds the catalog item contained in the specified XML file, then creates and returns an appropriate catalog item object. If the file does not contain a valid catalog item this method returns null. If load is true, this method loads the file into the item after creating it.  Returns the newly created catalog item if the file contains a catalog item. Returns null if the file does not contain a catalog item or if the function failed.
