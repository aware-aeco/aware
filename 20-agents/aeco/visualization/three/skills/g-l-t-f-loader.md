---
name: three-g-l-t-f-loader
description: GLTFLoader declarations from three
---

# GLTFLoader

## Methods

- `setDRACOLoader(dracoLoader: DRACOLoader)`
- `setKTX2Loader(ktx2Loader: KTX2Loader | null)`
- `setMeshoptDecoder(meshoptDecoder: typeof MeshoptDecoder | null)`
- `register(callback: (parser: GLTFParser) => GLTFLoaderPlugin)`
- `unregister(callback: (parser: GLTFParser) => GLTFLoaderPlugin)`
- `parse()`
- `parseAsync(data: ArrayBuffer | string, path: string)`
