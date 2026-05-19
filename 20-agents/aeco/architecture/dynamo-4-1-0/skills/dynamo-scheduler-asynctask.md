---
name: dynamo-dynamo-scheduler-asynctask
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Scheduler.AsyncTask namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: TaskPriority, TaskMergeInstruction.
---

# Dynamo.Scheduler.AsyncTask

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## TaskMergeInstruction (enum)

Type TaskMergeInstruction

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Scheduler/AsyncTask.cs)

### Values
- `KeepBoth` = `0` — Both the AsyncTask objects in comparison should be kept.
- `KeepOther` = `2` — The current instance of AsyncTask should be discarded while keeping the other AsyncTask in comparison.
- `KeepThis` = `1` — The current instance of AsyncTask should be kept while discarding the other AsyncTask in comparison.

## TaskPriority (enum)

Type TaskPriority

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Scheduler/AsyncTask.cs)

### Values
- `AboveNormal` = `2`
- `BelowNormal` = `4`
- `Critical` = `0`
- `Highest` = `1`
- `Idle` = `6`
- `Lowest` = `5`
- `Normal` = `3`

