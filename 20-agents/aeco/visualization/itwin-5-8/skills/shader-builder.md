---
name: core-frontend-shader-builder
description: ShaderBuilder declarations from core-frontend
---

# ShaderBuilder

## Methods

- `addInitializer(initializer: string)`
- `addComponent(index: number, component: string)`
- `removeComponent(index: number)`
- `getComponent(index: number)`
- `addFunction(declarationOrFull: string, implementation?: string)`
- `replaceFunction(existing: string, replacement: string)`
- `findFunction(func: string)`
- `addExtension(extName: string)`
- `addMacro(macro: string)`
- `addDefine(name: string, value: string)`
- `clearFragOutput()`
- `addFragOutput(name: string, value: number)`
- `buildPreludeCommon(attrMap: Map<string, AttributeDetails> | undefined, isVertexShader: boolean)`
- `copyCommon(src: ShaderBuilder)`
- `addInitializer(initializer: string)`
- `addComponent(index: number, component: string)`
- `removeComponent(index: number)`
- `getComponent(index: number)`
- `addFunction(declarationOrFull: string, implementation?: string)`
- `replaceFunction(existing: string, replacement: string)`
- `findFunction(func: string)`
- `addExtension(extName: string)`
- `addMacro(macro: string)`
- `addDefine(name: string, value: string)`
- `clearFragOutput()`
- `addFragOutput(name: string, value: number)`
- `buildPreludeCommon(attrMap: Map<string, AttributeDetails> | undefined, isVertexShader: boolean)`
- `copyCommon(src: ShaderBuilder)`
