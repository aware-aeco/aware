---
name: core-frontend-techniques
description: Techniques declarations from core-frontend
---

# Techniques

## Methods

- `create(gl: WebGL2RenderingContext)`
- `getTechnique(id: TechniqueId)`
- `addDynamicTechnique(technique: Technique, name: string)`
- `getDynamicTechniqueId(name: string)`
- `execute(target: Target, commands: DrawCommands, renderPass: RenderPass)`
- `executeForIndexedClassifier(target: Target, cmdsByIndex: DrawCommands, renderPass: RenderPass)`
- `draw(params: DrawParams)`
- `dispose()`
- `compileShaders()`
- `idleCompileNextShader()`
- `forEachVariedProgram(func: (program: ShaderProgram) => void)`
- `create(gl: WebGL2RenderingContext)`
- `getTechnique(id: TechniqueId)`
- `addDynamicTechnique(technique: Technique, name: string)`
- `getDynamicTechniqueId(name: string)`
- `execute(target: Target, commands: DrawCommands, renderPass: RenderPass)`
- `executeForIndexedClassifier(target: Target, cmdsByIndex: DrawCommands, renderPass: RenderPass)`
- `draw(params: DrawParams)`
- `dispose()`
- `compileShaders()`
- `idleCompileNextShader()`
- `forEachVariedProgram(func: (program: ShaderProgram) => void)`
