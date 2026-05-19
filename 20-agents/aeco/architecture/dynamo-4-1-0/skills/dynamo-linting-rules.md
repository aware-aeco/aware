---
name: dynamo-dynamo-linting-rules
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Linting.Rules namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GraphLinterRule, LinterRule, NodeLinterRule.
---

# Dynamo.Linting.Rules

Auto-generated from vendor docs for dynamo 4.1.0. 3 types in this namespace.

## GraphLinterRule (class)

Base class for creating Graph related linter rules

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/GraphLinterRule.cs)

### Constructors
- `void GraphLinterRule()` — GraphLinterRule.GraphLinterRule

### Methods
#### `System.Tuple<Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum, System.Collections.Generic.HashSet<string>> EvaluateFunction(Dynamo.Graph.Workspaces.WorkspaceModel workspaceModel, string changedEvent, Dynamo.Graph.Nodes.NodeModel modifiedNode)`

Function used to evaluate this rule

**Parameters:**
- `workspaceModel` (Dynamo.Graph.Workspaces.WorkspaceModel) — 
- `changedEvent` (string) — 
- `modifiedNode` (Dynamo.Graph.Nodes.NodeModel) — 

**Returns:** `System.Tuple<Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum, System.Collections.Generic.HashSet<string>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/GraphLinterRule.cs)

#### `System.Collections.Generic.List<System.Tuple<Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum, System.Collections.Generic.HashSet<string>>> InitFunction(Dynamo.Graph.Workspaces.WorkspaceModel workspaceModel)`

The init function is used when the Linter extension implementing this Rule is initialized.

**Parameters:**
- `workspaceModel` (Dynamo.Graph.Workspaces.WorkspaceModel) — 

**Returns:** `System.Collections.Generic.List<System.Tuple<Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum, System.Collections.Generic.HashSet<string>>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/GraphLinterRule.cs)

## LinterRule (class)

Base class for all linting rules

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/LinterRule.cs)

### Constructors
- `void LinterRule()` — LinterRule.LinterRule

### Methods
#### `void CleanupRule(Dynamo.Graph.Workspaces.WorkspaceModel muribundWorkspaceModel)`

Uses the cleanup function to allow a rule to get rid of any transient data it might have stored before the workspace model is disposed

**Parameters:**
- `muribundWorkspaceModel` (Dynamo.Graph.Workspaces.WorkspaceModel) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/LinterRule.cs)

### Properties
- `CallToAction` (string, get) — Description of how to solve issues related to this rule
- `Description` (string, get) — Description of the rule
- `EvaluationTriggerEvents` (System.Collections.Generic.List<string>, get) — List of Property names that this rule should react to changes on
- `Id` (string, get) — Unique id of this rule
- `SeverityCode` (Dynamo.Linting.Interfaces.SeverityCodesEnum, get) — Severity code of this rule. This code will define how the rule is displayed in the UI

## NodeLinterRule (class)

Base class for creating Node related linter rules

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/NodeLinterRule.cs)

### Constructors
- `void NodeLinterRule()` — NodeLinterRule.NodeLinterRule

### Methods
#### `Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum EvaluateFunction(Dynamo.Graph.Nodes.NodeModel nodeModel, string changedEvent)`

Function used to evaluate this rule

**Parameters:**
- `nodeModel` (Dynamo.Graph.Nodes.NodeModel) — Node to evaluate
- `changedEvent` (string) — 

**Returns:** `Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/NodeLinterRule.cs)

#### `System.Collections.Generic.List<System.Tuple<Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum, string>> InitFunction(Dynamo.Graph.Workspaces.WorkspaceModel workspaceModel)`

The init function is used when the Linter extension implementing this Rule is initialized.

**Parameters:**
- `workspaceModel` (Dynamo.Graph.Workspaces.WorkspaceModel) — 

**Returns:** `System.Collections.Generic.List<System.Tuple<Dynamo.Linting.Interfaces.RuleEvaluationStatusEnum, string>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Linting/Rules/NodeLinterRule.cs)

