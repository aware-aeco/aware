---
name: core-frontend-program-builder
description: ProgramBuilder declarations from core-frontend
---

# ProgramBuilder

## Methods

- `addUniform(name: string, type: VariableType, binding: AddVariableBinding, which?: ShaderType)`
- `addUniformArray(name: string, type: VariableType, length: number, binding: AddVariableBinding, which?: ShaderType)`
- `addVarying(name: string, type: VariableType)`
- `addGlobal(name: string, type: VariableType, which?: ShaderType, value?: string, isConst?: boolean)`
- `addInlineComputedVarying(name: string, type: VariableType, inlineComputation: string)`
- `addFunctionComputedVarying(name: string, type: VariableType, funcName: string, funcBody: string)`
- `addFunctionComputedVaryingWithArgs(name: string, type: VariableType, funcCall: string, funcDef: string)`
- `buildProgram(gl: WebGL2RenderingContext)`
- `setDebugDescription(description: string)`
- `clone()`
- `addUniform(name: string, type: VariableType, binding: AddVariableBinding, which?: ShaderType)`
- `addUniformArray(name: string, type: VariableType, length: number, binding: AddVariableBinding, which?: ShaderType)`
- `addVarying(name: string, type: VariableType)`
- `addGlobal(name: string, type: VariableType, which?: ShaderType, value?: string, isConst?: boolean)`
- `addInlineComputedVarying(name: string, type: VariableType, inlineComputation: string)`
- `addFunctionComputedVarying(name: string, type: VariableType, funcName: string, funcBody: string)`
- `addFunctionComputedVaryingWithArgs(name: string, type: VariableType, funcCall: string, funcDef: string)`
- `buildProgram(gl: WebGL2RenderingContext)`
- `setDebugDescription(description: string)`
- `clone()`
