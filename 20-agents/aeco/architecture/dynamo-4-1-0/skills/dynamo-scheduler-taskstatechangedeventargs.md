---
name: dynamo-dynamo-scheduler-taskstatechangedeventargs
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Scheduler.TaskStateChangedEventArgs namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: State.
---

# Dynamo.Scheduler.TaskStateChangedEventArgs

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## State (enum)

Type State

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/SyncDataManager.cs)

### Values
- `CompletionHandled` = `5` — Post-execute action of task is completed and registered event handlers of task completion are notified.
- `Discarded` = `1` — Task is dropped from the scheduler due to compacting process
- `ExecutionCompleted` = `4` — Task execution is completed successfully
- `ExecutionFailed` = `3` — Task execution is completed with errors
- `ExecutionStarting` = `2` — Task is about to be executed
- `Scheduled` = `0` — Task is added to the scheduler

