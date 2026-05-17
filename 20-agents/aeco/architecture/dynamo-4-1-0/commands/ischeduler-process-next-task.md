# ischeduler-process-next-task

Lifecycle: single

An ISchedulerThread implementation calls this method so scheduler              starts to process the next task in the queue, if there is any. Note              that this method is meant to process only one task in queue. The              implementation of ISchedulerThread is free to call this method again             in a fashion that matches its task fetching behavior.
