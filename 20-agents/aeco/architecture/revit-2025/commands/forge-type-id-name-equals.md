# forge-type-id-name-equals

Lifecycle: single

Returns true if the typeid string held by this ForgeTypeId is equal to that held by the given ForgeTypeId,    excluding the version number. The version number of a typeid string follows a hyphen character. This function    compares the typeid strings up to the first hyphen. This is the default equality comparison method for the    ForgeTypeId class, used by the equality operator (==).
