---
name: core-frontend-shader-variables
description: ShaderVariables declarations from core-frontend
---

# ShaderVariables

## Methods

- `find(name: string)`
- `addVariable(v: ShaderVariable)`
- `addUniform(name: string, type: VariableType, binding: AddVariableBinding, precision?: VariablePrecision)`
- `addUniformArray(name: string, type: VariableType, length: number, binding: AddVariableBinding)`
- `addVarying(name: string, type: VariableType)`
- `addGlobal(name: string, type: VariableType, value?: string, isConst?: boolean)`
- `addConstant(name: string, type: VariableType, value: string)`
- `addBitFlagConstant(name: string, value: number)`
- `buildScopeDeclarations(isVertexShader: boolean, scope: VariableScope, constants?: boolean)`
- `buildDeclarations(isVertexShader: boolean)`
- `addBindings(prog: ShaderProgram, predefined?: ShaderVariables)`
- `checkMaxVaryingVectors(fragSource: string)`
- `computeNumVaryingVectors(fragSource: string)`
- `find(name: string)`
- `addVariable(v: ShaderVariable)`
- `addUniform(name: string, type: VariableType, binding: AddVariableBinding, precision?: VariablePrecision)`
- `addUniformArray(name: string, type: VariableType, length: number, binding: AddVariableBinding)`
- `addVarying(name: string, type: VariableType)`
- `addGlobal(name: string, type: VariableType, value?: string, isConst?: boolean)`
- `addConstant(name: string, type: VariableType, value: string)`
- `addBitFlagConstant(name: string, value: number)`
- `buildScopeDeclarations(isVertexShader: boolean, scope: VariableScope, constants?: boolean)`
- `buildDeclarations(isVertexShader: boolean)`
- `addBindings(prog: ShaderProgram, predefined?: ShaderVariables)`
- `checkMaxVaryingVectors(fragSource: string)`
- `computeNumVaryingVectors(fragSource: string)`
