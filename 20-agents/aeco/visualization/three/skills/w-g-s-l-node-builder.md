---
name: three-w-g-s-l-node-builder
description: WGSLNodeBuilder declarations from three
---

# WGSLNodeBuilder

## Methods

- `_generateTextureSample()`
- `_generateVideoSample(textureProperty: string, uvSnippet: string, shaderStage: NodeShaderStage)`
- `_generateTextureSampleLevel()`
- `generateTextureLod(texture: Texture, textureProperty: string, uvSnippet: string, levelSnippet: string)`
- `generateTextureLoad()`
- `generateTextureStore()`
- `isUnfilterable(texture: Texture)`
- `generateTexture()`
- `generateTextureGrad()`
- `generateTextureCompare()`
- `generateTextureLevel()`
- `getPropertyName(node: NodeVar | NodeUniform<unknown, unknown>, shaderStage: NodeShaderStage)`
- `getOutputStructName()`
- `_getUniformGroupCount(shaderStage: NodeShaderStage)`
- `getFunctionOperator(op: string)`
- `getBuiltin(name: string, property: string, type: string, shaderStage: BuiltinStage)`
- `getVertexIndex()`
- `getInstanceIndex()`
- `getDrawIndex()`
- `buildFunctionCode(shaderNode: ShaderNode)`
- `getFragDepth()`
- `getFragCoord()`
- `getFrontFacing()`
- `getBuiltins(shaderStage: BuiltinStage)`
- `getAttributes(shaderStage: NodeShaderStage)`
- `getStructMembers(struct: StructTypeNode)`
- `getStructs(shaderStage: NodeShaderStage)`
- `getVar(type: string, name: string)`
- `getVars(shaderStage: NodeShaderStage)`
- `getVaryings(shaderStage: NodeShaderStage)`
- `getUniforms(shaderStage: NodeShaderStage)`
- `buildCode()`
- `getMethod(method: string, output?: string | null)`
- `_getWGSLMethod(method: string)`
- `_include(name: string)`
- `_getWGSLVertexCode(shaderData: { [key: string]: string })`
- `_getWGSLFragmentCode(shaderData: { [key: string]: string })`
- `_getWGSLComputeCode(shaderData: { [key: string]: string }, workgroupSize: string)`
- `_getWGSLStruct(name: string, vars: string)`
- `_getWGSLStructBinding(name: string, vars: string, access: string, binding: number, group: number)`
