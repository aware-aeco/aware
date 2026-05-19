---
name: dynamo-dynamo-scheduler
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Scheduler namespace — 12 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AsyncTaskCompletedHandler, AsyncTask, AsyncTaskExtensions, DelegateBasedAsyncTask, TaskStateChangedEventArgs, TaskStateChangedEventHandler, IScheduler, TaskProcessMode, and 4 more types.
---

# Dynamo.Scheduler

Auto-generated from vendor docs for dynamo 4.1.1. 12 types in this namespace.

## AsyncTask (class)

This is the base class for async tasks, that are running on Dynamo Scheduler.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

### Constructors
- `void AsyncTask(Dynamo.Scheduler.IScheduler scheduler)` — Constructs an instance of AsyncTask object.

### Methods
#### `Dynamo.Scheduler.AsyncTask.TaskMergeInstruction CanMergeWithCore(Dynamo.Scheduler.AsyncTask otherTask)`

AsyncTask.CanMergeWithCore

**Parameters:**
- `otherTask` (Dynamo.Scheduler.AsyncTask)

**Returns:** `Dynamo.Scheduler.AsyncTask.TaskMergeInstruction` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

#### `int CompareCore(Dynamo.Scheduler.AsyncTask otherTask)`

AsyncTask.CompareCore

**Parameters:**
- `otherTask` (Dynamo.Scheduler.AsyncTask)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

#### `void HandleTaskCompletionCore()`

AsyncTask.HandleTaskCompletionCore

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

#### `void HandleTaskExecutionCore()`

AsyncTask.HandleTaskExecutionCore

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

### Properties
- `Priority` (Dynamo.Scheduler.AsyncTask.TaskPriority, get) — DynamoScheduler sorts tasks based on two key factors: the priority of a task, and the relative importance between two tasks that have the same priority. During task re-prioritizing process, DynamoScheduler first sorts the tasks in accordance to their AsyncTask.Priority property. The resulting ordered list is then sorted again by calling AsyncTask.Compare method to determine the relative importance among tasks having the same priority.

### Events
#### `Completed` (`Dynamo.Scheduler.AsyncTaskCompletedHandler`)

**Signature:** `public event Dynamo.Scheduler.AsyncTaskCompletedHandler Completed`

This event is raised when the AsyncTask is completed, in the context of ISchedulerThread, any UI element access that is needed should be dispatched onto the UI dispatcher.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

## AsyncTaskCompletedHandler (delegate)

This delegate is used in AsyncTask events: Completed and Discarded.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

### Constructors
- `void AsyncTaskCompletedHandler(object object, IntPtr method)` — AsyncTaskCompletedHandler.AsyncTaskCompletedHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Scheduler.AsyncTask asyncTask, System.AsyncCallback callback, object object)`

AsyncTaskCompletedHandler.BeginInvoke

**Parameters:**
- `asyncTask` (Dynamo.Scheduler.AsyncTask)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

#### `void EndInvoke(System.IAsyncResult result)`

AsyncTaskCompletedHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

#### `void Invoke(Dynamo.Scheduler.AsyncTask asyncTask)`

AsyncTaskCompletedHandler.Invoke

**Parameters:**
- `asyncTask` (Dynamo.Scheduler.AsyncTask)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTask.cs)

## AsyncTaskExtensions (static-class)

Tools for working productively with AsyncTask's

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTaskExtensions.cs)

### Methods
#### `System.IDisposable AllComplete(System.Collections.Generic.IEnumerable<Dynamo.Scheduler.AsyncTask> tasks, System.Action<System.Collections.Generic.IEnumerable<Dynamo.Scheduler.AsyncTask>> action)`

Await the completion of a collection of scheduled tasks. The given action will only be executed after all events are complete or discarded. The tasks must already be scheduled and not yet completed or this action will never be executed.

**Parameters:**
- `tasks` (System.Collections.Generic.IEnumerable<Dynamo.Scheduler.AsyncTask>)
- `action` (System.Action<System.Collections.Generic.IEnumerable<Dynamo.Scheduler.AsyncTask>>)

**Returns:** `System.IDisposable` — An IDisposable representing all of the event subscription

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTaskExtensions.cs)

#### `System.IDisposable Then(Dynamo.Scheduler.AsyncTask task, Dynamo.Scheduler.AsyncTaskCompletedHandler action)`

Upon completion of the task, invoke the specified action

**Parameters:**
- `task` (Dynamo.Scheduler.AsyncTask)
- `action` (Dynamo.Scheduler.AsyncTaskCompletedHandler)

**Returns:** `System.IDisposable` — An IDisposable representing the event subscription

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTaskExtensions.cs)

#### `System.IDisposable ThenPost(Dynamo.Scheduler.AsyncTask task, Dynamo.Scheduler.AsyncTaskCompletedHandler action, System.Threading.SynchronizationContext context)`

Upon completion of the task, invoke the action asynchronously in the specified SynchronizationContext

**Parameters:**
- `task` (Dynamo.Scheduler.AsyncTask)
- `action` (Dynamo.Scheduler.AsyncTaskCompletedHandler)
- `context` (System.Threading.SynchronizationContext)

**Returns:** `System.IDisposable` — An IDisposable representing the event subscription

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTaskExtensions.cs)

#### `System.IDisposable ThenSend(Dynamo.Scheduler.AsyncTask task, Dynamo.Scheduler.AsyncTaskCompletedHandler action, System.Threading.SynchronizationContext context)`

Upon completion of the task, invoke the action synchronously in the specified SynchronizationContext

**Parameters:**
- `task` (Dynamo.Scheduler.AsyncTask)
- `action` (Dynamo.Scheduler.AsyncTaskCompletedHandler)
- `context` (System.Threading.SynchronizationContext)

**Returns:** `System.IDisposable` — An IDisposable representing the event subscription

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/AsyncTaskExtensions.cs)

## DelegateBasedAsyncTask (class)

DelegateBasedAsyncTask allows for a delegate or System.Action object to be scheduled for asynchronous execution on the ISchedulerThread.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DelegateBasedAsyncTask.cs)

### Constructors
- `void DelegateBasedAsyncTask(Dynamo.Scheduler.IScheduler scheduler)` — construct a new empty DelegateBasedAsyncTask
- `void DelegateBasedAsyncTask(Dynamo.Scheduler.IScheduler scheduler, System.Action action)` — construct a new DelegateBasedAsyncTask by supplying an action delegate that will run on the scheduler specified

### Methods
#### `void HandleTaskCompletionCore()`

DelegateBasedAsyncTask.HandleTaskCompletionCore

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DelegateBasedAsyncTask.cs)

#### `void HandleTaskExecutionCore()`

DelegateBasedAsyncTask.HandleTaskExecutionCore

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DelegateBasedAsyncTask.cs)

### Properties
- `Priority` (Dynamo.Scheduler.AsyncTask.TaskPriority, get) — Returns priority of the Dynamo.Scheduler.DelegateBasedAsyncTask.

## Disposable (static-class)

Implements IDisposable functionality.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Disposable.cs)

### Methods
#### `System.IDisposable Create(System.Action disposeAction)`

Constructs a new disposable that calls the delegate when disposed

**Parameters:**
- `disposeAction` (System.Action) — An action that runs when this object is disposed

**Returns:** `System.IDisposable` — New disposable object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Disposable.cs)

## DynamoScheduler (class)

This class represents Dynamo scheduler. All the tasks are scheduled on the scheduler. Also, these tasks runs async.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

### Methods
#### `bool ProcessNextTask(bool waitIfTaskQueueIsEmpty)`

An ISchedulerThread implementation calls this method so scheduler starts to process the next task in the queue, if there is any. Note that this method is meant to process only one task in queue. The implementation of ISchedulerThread is free to call this method again in a fashion that matches its task fetching behavior.

**Parameters:**
- `waitIfTaskQueueIsEmpty` (bool) — This parameter is only used if the task queue is empty at the time this method is invoked. When the task queue becomes empty, setting this to true will cause this call to block until either the next task becomes available, or when the scheduler is requested to shutdown.

**Returns:** `bool` — This method returns true if the task queue is not empty, or false otherwise. Note that this method returns false when scheduler begins to shutdown, even when the task queue is not empty.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

#### `void ScheduleForExecution(Dynamo.Scheduler.AsyncTask asyncTask)`

Callers of this method create an instance of AsyncTask derived class and call this method to schedule the task for execution.

**Parameters:**
- `asyncTask` (Dynamo.Scheduler.AsyncTask) — The task to execute asynchronously.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

### Properties
- `HasPendingTasks` (bool, get) — Returns true if task queue is not empty.
- `HasTaskInProgress` (bool, get) — Returns true if a task is curently executing
- `NextTimeStamp` (Dynamo.Scheduler.TimeStamp, get) — AsyncTask base class calls this to obtain the new time-stamp value.
- `ProcessMode` (Dynamo.Scheduler.TaskProcessMode, get/set) — Flag determining whether or not the scheduled task will be processed immediately or not.
- `Tasks` (System.Collections.Generic.IEnumerable<Dynamo.Scheduler.AsyncTask>, get) — DynamoScheduler.Tasks

## IScheduler (interface)

This interface provides methods and properties used for Dynamo Scheduler.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

### Methods
#### `bool ProcessNextTask(bool waitIfTaskQueueIsEmpty)`

An ISchedulerThread implementation calls this method so scheduler starts to process the next task in the queue, if there is any. Note that this method is meant to process only one task in queue. The implementation of ISchedulerThread is free to call this method again in a fashion that matches its task fetching behavior.

**Parameters:**
- `waitIfTaskQueueIsEmpty` (bool) — This parameter is only used if the task queue is empty at the time this method is invoked. When the task queue becomes empty, setting this to true will cause this call to block until either the next task becomes available, or when the scheduler is requested to shutdown.

**Returns:** `bool` — This method returns true if the task queue is not empty, or false otherwise. Note that this method returns false when scheduler begins to shutdown, even when the task queue is not empty.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

#### `void ScheduleForExecution(Dynamo.Scheduler.AsyncTask asyncTask)`

Callers of this method create an instance of AsyncTask derived class and call this method to schedule the task for execution.

**Parameters:**
- `asyncTask` (Dynamo.Scheduler.AsyncTask) — The task to execute asynchronously.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

### Properties
- `NextTimeStamp` (Dynamo.Scheduler.TimeStamp, get) — AsyncTask base class calls this to obtain the new time-stamp value.
- `Tasks` (System.Collections.Generic.IEnumerable<Dynamo.Scheduler.AsyncTask>, get) — The complete collection of all of the currently scheduled tasks

## ISchedulerThread (interface)

Interface provides method for DynamoSchedulerThread.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/ISchedulerThread.cs)

### Methods
#### `void Initialize(Dynamo.Scheduler.IScheduler owningScheduler)`

DynamoScheduler calls this method to initialize and start this instance of scheduler thread. This call marks the point from which it is safe to call into DynamoScheduler.

**Parameters:**
- `owningScheduler` (Dynamo.Scheduler.IScheduler) — A reference to the DynamoScheduler object which owns this instance of scheduler thread.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/ISchedulerThread.cs)

#### `void Shutdown()`

DynamoScheduler calls this method to shutdown the scheduler thread.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/ISchedulerThread.cs)

## TaskProcessMode (enum)

Describes the way a scheduled task will be processed.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

### Values
- `Asynchronous` = `1` — Scheduled task will be processed by schedule
- `Synchronous` = `0` — Scheduled task will be processed immediately

## TaskStateChangedEventArgs (class)

Provides data for DynamoScheduler.TaskStateChanged events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

## TaskStateChangedEventHandler (delegate)

Represents the method that will handle Dynamo.Scheduler.DynamoScheduler.TaskStateChanged events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

### Constructors
- `void TaskStateChangedEventHandler(object object, IntPtr method)` — TaskStateChangedEventHandler.TaskStateChangedEventHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Scheduler.DynamoScheduler sender, Dynamo.Scheduler.TaskStateChangedEventArgs e, System.AsyncCallback callback, object object)`

TaskStateChangedEventHandler.BeginInvoke

**Parameters:**
- `sender` (Dynamo.Scheduler.DynamoScheduler)
- `e` (Dynamo.Scheduler.TaskStateChangedEventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

#### `void EndInvoke(System.IAsyncResult result)`

TaskStateChangedEventHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

#### `void Invoke(Dynamo.Scheduler.DynamoScheduler sender, Dynamo.Scheduler.TaskStateChangedEventArgs e)`

TaskStateChangedEventHandler.Invoke

**Parameters:**
- `sender` (Dynamo.Scheduler.DynamoScheduler)
- `e` (Dynamo.Scheduler.TaskStateChangedEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/DynamoScheduler.cs)

## TimeStamp (struct)

This class is used to set creation time of async task in Dynamo Scheduler.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/TimeStampGenerator.cs)

### Methods
#### `bool Equals(object other)`

TimeStamp.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/TimeStampGenerator.cs)

#### `int GetHashCode()`

TimeStamp.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/TimeStampGenerator.cs)

#### `bool op_GreaterThan(Dynamo.Scheduler.TimeStamp timeStamp0, Dynamo.Scheduler.TimeStamp timeStamp1)`

The public usage of time stamps should be restricted to these methods which are used to ensure an ordering on timestamps

**Parameters:**
- `timeStamp0` (Dynamo.Scheduler.TimeStamp) — The first time stamp in comparison.
- `timeStamp1` (Dynamo.Scheduler.TimeStamp) — The second time stamp in comparison.

**Returns:** `bool` — Return true if the first time stamp was created later than the second time stamp, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/TimeStampGenerator.cs)

#### `bool op_LessThan(Dynamo.Scheduler.TimeStamp timeStamp0, Dynamo.Scheduler.TimeStamp timeStamp1)`

The public usage of time stamps should be restricted to these methods which are used to ensure an ordering on timestamps

**Parameters:**
- `timeStamp0` (Dynamo.Scheduler.TimeStamp) — The first time stamp in comparison.
- `timeStamp1` (Dynamo.Scheduler.TimeStamp) — The second time stamp in comparison.

**Returns:** `bool` — Return true if the first time stamp was created earlier than the second time stamp, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Scheduler/TimeStampGenerator.cs)

