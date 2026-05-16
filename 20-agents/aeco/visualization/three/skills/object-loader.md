---
name: three-object-loader
description: ObjectLoader declarations from three
---

# ObjectLoader

## Methods

- `load()`
- `parse(json: unknown, onLoad?: (object: Object3D) => void)`
- `parseAsync(json: unknown)`
- `parseGeometries(json: unknown)`
- `parseMaterials(json: unknown, textures: { [key: string]: Texture })`
- `parseAnimations(json: unknown)`
- `parseImages(json: unknown, onLoad?: () => void)`
- `parseImagesAsync(json: unknown)`
- `parseTextures(json: unknown, images: { [key: string]: Source<unknown> })`
- `parseObject()`
