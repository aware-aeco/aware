---
name: three-stack-node
description: StackNode declarations from three
---

# StackNode

## Methods

- `If(boolNode: Node, method: () => void)`
- `ElseIf(boolNode: Node, method: () => void)`
- `Else(method: () => void)`
- `Switch(expression: Node)`
- `Case(...params: [...Node[], () => void])`
- `Default(method: () => void)`
