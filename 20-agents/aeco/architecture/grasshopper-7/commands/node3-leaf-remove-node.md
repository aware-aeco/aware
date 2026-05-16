# node3-leaf-remove-node

Lifecycle: single

Remove the node with the specified tag from this leaf or any subleafs.   This method assumes that a node with a given tag occurs at most once within this subtree.   Removing nodes will not close leafs, you need to call TrimExcess() in order to remove   empty subleafs or empty nodelists.
