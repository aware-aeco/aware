---
name: dynamo-applications-dynamo-scheduler
description: API reference for namespace Dynamo.Scheduler from DynamoCore.dll
---

# Dynamo.Scheduler

- **AsyncTask**
  - This is the base class for async tasks, that are running on Dynamo Scheduler.
- **AsyncTaskCompletedHandler**
  - This delegate is used in AsyncTask events: Completed and Discarded.
- **AsyncTaskExtensions**
  - Tools for working productively with AsyncTask's
- **DelegateBasedAsyncTask**
  - DelegateBasedAsyncTask allows for a delegate or System.Action object              to be scheduled for asynchronous execution on the ISchedulerThread.
- **DynamoScheduler**
  - This class represents Dynamo scheduler. All the tasks are scheduled on the scheduler. Also, these tasks runs async.
- **IScheduler**
  - This interface provides methods and properties used for Dynamo Scheduler.
- **ISchedulerThread**
  - Interface provides method for DynamoSchedulerThread.
- **TaskProcessMode**
  - Describes the way a scheduled task will be processed.
- **TaskStateChangedEventArgs**
  - Provides data for DynamoScheduler.TaskStateChanged events.
- **TaskStateChangedEventHandler**
  - Represents the method that will handle  events.
- **TimeStamp**
  - This class is used to set creation time of async task in Dynamo Scheduler.
