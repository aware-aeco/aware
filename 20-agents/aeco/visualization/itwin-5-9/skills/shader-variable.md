---
name: core-frontend-shader-variable
description: ShaderVariable declarations from core-frontend
---

# ShaderVariable

## Methods

- `create(name: string, type: VariableType, scope: VariableScope, addBinding?: AddVariableBinding, precision?: VariablePrecision)`
- `createArray(name: string, type: VariableType, length: number, scope: VariableScope, addBinding?: AddVariableBinding, precision?: VariablePrecision)`
- `createGlobal(name: string, type: VariableType, value?: string, isConst?: boolean)`
- `addBinding(prog: ShaderProgram)`
- `getScopeName(isVertexShader: boolean)`
- `buildDeclaration(isVertexShader: boolean)`
- `create(name: string, type: VariableType, scope: VariableScope, addBinding?: AddVariableBinding, precision?: VariablePrecision)`
- `createArray(name: string, type: VariableType, length: number, scope: VariableScope, addBinding?: AddVariableBinding, precision?: VariablePrecision)`
- `createGlobal(name: string, type: VariableType, value?: string, isConst?: boolean)`
- `addBinding(prog: ShaderProgram)`
- `getScopeName(isVertexShader: boolean)`
- `buildDeclaration(isVertexShader: boolean)`
