---
name: three-g-p-u-computation-renderer
description: GPUComputationRenderer declarations from three
---

# GPUComputationRenderer

## Methods

- `setDataType(type: TextureDataType)`
- `addVariable(variableName: string, computeFragmentShader: string, initialValueTexture: Texture)`
- `setVariableDependencies(variable: Variable, dependencies: Variable[] | null)`
- `init()`
- `compute()`
- `getCurrentRenderTarget(variable: Variable)`
- `getAlternateRenderTarget(variable: Variable)`
- `addResolutionDefine(materialShader: ShaderMaterial)`
- `createShaderMaterial(computeFragmentShader: string, uniforms?: { [uniform: string]: IUniform })`
- `createRenderTarget()`
- `createTexture()`
- `renderTexture(input: Texture, output: WebGLRenderTarget)`
- `doRenderTarget(material: Material, output: WebGLRenderTarget)`
- `dispose()`
