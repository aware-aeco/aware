# package-loader-load-custom-nodes-and-packages

Lifecycle: single

LoadCustomNodesAndPackages can be used to load custom nodes and packages             from temporary paths that do not need to be added to preference settings.              To load from temporary custom paths, initialize a local PreferenceSettings object              and add the paths to its CustomPackageFolders property, then initialize a new              LoadPackageParams with this preferences object and use as input to this method.             To load from custom paths that need to be persisted to the preferences,              initialize a LoadPackageParams from an existing preferences object.
