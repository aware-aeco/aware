---
name: three-node-builder
description: NodeBuilder declarations from three
---

# NodeBuilder

## Methods

- `setHashNode(node: Node, hash: string)`
- `addNode(node: Node)`
- `isFilteredTexture(texture: Texture)`
- `getMethod(method: string)`
- `getNodeFromHash(hash: string)`
- `addFlow(shaderStage: NodeShaderStage, node: Node)`
- `setContext(context: NodeBuilderContext)`
- `getContext()`
- `setCache(cache: NodeCache)`
- `getCache()`
- `getCacheFromNode(node: Node, parent?: boolean)`
- `isAvailable(name: string)`
- `getInstanceIndex()`
- `getDrawIndex()`
- `getFrontFacing()`
- `getFragCoord()`
- `isFlipY()`
- `getConst(type: string, value?: unknown)`
- `getType(type: string)`
- `generateMethod(method: string)`
- `getAttribute(name: string, type: string)`
- `getPropertyName(node: NodeVar | NodeUniform<unknown, unknown>, shaderStage: NodeShaderStage)`
- `isVector(type: string)`
- `isMatrix(type: string)`
- `isReference(type: string)`
- `getElementType(type: string)`
- `getComponentType(type: string)`
- `getVectorType(type: string)`
- `getTypeFromLength(length: number)`
- `getTypeLength(type: string)`
- `getVectorFromMatrix(type: string)`
- `getDataFromNode(node: Node, shaderStage?: NodeShaderStage)`
- `getNodeProperties(node: Node, shaderStage?: NodeShaderStage)`
- `getUniformFromNode()`
- `getVarFromNode(node: Node, type: string, shaderStage?: NodeShaderStage)`
- `getVaryFromNode(node: Node, type: string)`
- `getCodeFromNode(node: Node, type: string, shaderStage?: NodeShaderStage)`
- `addFlowCode(code: string)`
- `getFlowData(node: Node, shaderStage: NodeShaderStage)`
- `flowNode(node: Node)`
- `flowChildNode(node: Node, output?: string | null)`
- `flowNodeFromShaderStage()`
- `hasGeometryAttribute(name: string)`
- `getAttributes(shaderStage: NodeShaderStage)`
- `getVaryings(shaderStage: NodeShaderStage)`
- `getVars(shaderStage: NodeShaderStage)`
- `getUniforms(stage: NodeShaderStage)`
- `getCodes(shaderStage: NodeShaderStage)`
- `getHash()`
- `setShaderStage(shaderStage: NodeShaderStage)`
- `getShaderStage()`
- `setBuildStage(buildStage: BuildStageOption)`
- `getBuildStage()`
- `buildCode()`
- `build()`
- `format(snippet: string, fromType: string, toType: string)`
- `getSignature()`
