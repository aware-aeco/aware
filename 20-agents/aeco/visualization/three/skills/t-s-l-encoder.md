---
name: three-t-s-l-encoder
description: TSLEncoder declarations from three
---

# TSLEncoder

## Methods

- `addImport(name: string)`
- `emitUniform(node: Uniform)`
- `emitExpression(node: Expression, output?: string | null)`
- `emitBody(body: Statement[])`
- `emitTernary(node: Ternary)`
- `emitConditional(node: Conditional)`
- `emitLoop(node: For)`
- `emitSwitch(switchNode: Switch)`
- `emitFor(node: For)`
- `emitForWhile(node: For)`
- `emitWhile(node: While)`
- `emitVariables(node: VariableDeclaration, isRoot?: boolean)`
- `emitVarying(node: Varying)`
- `emitStructDefinition(node: StructDefinition)`
- `emitOverloadingFunction(nodes: FunctionDeclaration[])`
- `emitFunction(node: FunctionDeclaration)`
- `setLastStatement(statement: Statement | null)`
- `emitComment(statement: Comment, body: Statement[])`
- `emitExtraLine(statement: Statement, body: Statement[])`
- `emit(ast: Program)`
