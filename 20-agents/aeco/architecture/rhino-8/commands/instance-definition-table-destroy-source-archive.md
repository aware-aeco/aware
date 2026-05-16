# instance-definition-table-destroy-source-archive

Lifecycle: single

Destroys all source archive information.             Specifically:             *  is set to the empty string.             * SourceRelativePath is set to false             * The alternative source archive path is set to the empty string.             * Checksum.Zero() is used to private destroy all checksum information.             *  is set to .
