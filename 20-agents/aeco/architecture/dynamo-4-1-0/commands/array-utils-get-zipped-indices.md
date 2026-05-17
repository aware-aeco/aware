# array-utils-get-zipped-indices

Lifecycle: single

For an array we supporting zipped replicaiton for array indexing as              well. I.e., for the following expression:                              a[1..3][2..4] = x;                          It will be expanded to                               a[1][2] = x;                 a[2][3] = x;                 a[3][4] = x;                              So here we need to calculate zipped indices. The length of returned              indices is decided by the shortest length of index that used in              array indexing. E.g.,                          For array indexing                              [{1, 2, 3}][{"x", "y"}][{6, 7, 8}], i.e.,                                   1 -> "x" -> 6                 2 -> "y" -> 7                 3 ->     -> 8                          The shortest length of index is 2 ({"x", "y"}), so function will              returns:                              {{1, "x", 6}, {2, "y", 7}}
