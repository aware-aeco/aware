# binary-archive-writer-write3dm-chunk-version

Lifecycle: single

A chunk version is a single byte that encodes a major.minor             version number.  Useful when creating I/O code for 3dm chunks             that may change in the future.  Increment the minor version              number if new information is added to the end of the chunk.              Increment the major version if the format of the chunk changes             in some other way.
