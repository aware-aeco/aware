# igh-active-object-compute-data

Lifecycle: single

This function is called whenever the object is required to generate new data.   This step is only performed by some objects and only when the Phase flag is Collected.  Upon completion, the Phase will be Computed. If this object throws exceptions,   it is the responsibility of the caller to set the Phase flag to Failed.
