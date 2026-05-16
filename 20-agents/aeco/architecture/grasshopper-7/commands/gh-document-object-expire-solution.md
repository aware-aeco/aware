# gh-document-object-expire-solution

Lifecycle: single

Call this function whenever you do something which expires the current solution.   This will make sure all caches are erased, all downstream objects are expired and that   the event is raised.   The default implementation merely places a call to OnSolutionExpired(), override this function   in derived classes to make sure you clear local data caches and expire downstream objects.
