---
name: core-frontend-gltf-reader
description: GltfReader declarations from core-frontend
---

# GltfReader

## Methods

- `read()`
- `traverseNodes(nodeIds: Iterable<GltfId>)`
- `traverseScene()`
- `readGltfAndCreateGraphics(isLeaf: boolean, featureTable: FeatureTable | undefined, contentRange: ElementAlignedBox3d | undefined, transformToRoot?: Transform, pseudoRtcBias?: Vector3d, instances?: InstancedGraphicParams)`
- `readGltfAndCreateTemplate(isLeaf: boolean, featureTable: FeatureTable | undefined, contentRange: ElementAlignedBox3d | undefined, noDispose: boolean, transformToRoot?: Transform, pseudoRtcBias?: Vector3d, instances?: InstancedGraphicParams)`
- `readGltfAndCreateGeometry(transformToRoot?: Transform, needNormals?: boolean, needParams?: boolean)`
- `getBufferView()`
- `readBufferData32()`
- `readBufferData16()`
- `readBufferData8()`
- `readBufferDataFloat()`
- `readBufferData()`
- `readFeatureIndices(_json: any)`
- `createDisplayParams(material: GltfMaterial, hasBakedLighting: boolean, isPointPrimitive?: boolean, lineStyle?: MaterialLineStyle)`
- `readMeshPrimitive(primitive: GltfMeshPrimitive, featureTable?: FeatureTable, pseudoRtcBias?: Vector3d)`
- `readIndices()`
- `readBatchTable(_mesh: Mesh, _json: GltfMeshPrimitive)`
- `readPrimitiveFeatures(primitive: GltfMeshPrimitive)`
- `readMeshIndices()`
- `readMeshNormals()`
- `readAndOctEncodeNormals()`
- `readColors()`
- `readPolylines()`
- `resolveResources()`
- `resolveUrl(uri: string)`
- `getTextureType(sampler?: GltfSampler)`
- `findTextureMapping(id: string | undefined, isTransparent: boolean, normalMapId: string | undefined, constantLodParamProps: TextureMapping.ConstantLodParamProps | undefined, normalMapUseConstantLod?: boolean)`
- `read()`
- `traverseNodes(nodeIds: Iterable<GltfId>)`
- `traverseScene()`
- `readGltfAndCreateGraphics(isLeaf: boolean, featureTable: FeatureTable | undefined, contentRange: ElementAlignedBox3d | undefined, transformToRoot?: Transform, pseudoRtcBias?: Vector3d, instances?: InstancedGraphicParams)`
- `readGltfAndCreateTemplate(isLeaf: boolean, featureTable: FeatureTable | undefined, contentRange: ElementAlignedBox3d | undefined, noDispose: boolean, transformToRoot?: Transform, pseudoRtcBias?: Vector3d, instances?: InstancedGraphicParams)`
- `readGltfAndCreateGeometry(transformToRoot?: Transform, needNormals?: boolean, needParams?: boolean)`
- `getBufferView()`
- `readBufferData32()`
- `readBufferData16()`
- `readBufferData8()`
- `readBufferDataFloat()`
- `readBufferData()`
- `readFeatureIndices(_json: any)`
- `createDisplayParams(material: GltfMaterial, hasBakedLighting: boolean, isPointPrimitive?: boolean, lineStyle?: MaterialLineStyle)`
- `readMeshPrimitive(primitive: GltfMeshPrimitive, featureTable?: FeatureTable, pseudoRtcBias?: Vector3d)`
- `readIndices()`
- `readBatchTable(_mesh: Mesh, _json: GltfMeshPrimitive)`
- `readPrimitiveFeatures(primitive: GltfMeshPrimitive)`
- `readMeshIndices()`
- `readMeshNormals()`
- `readAndOctEncodeNormals()`
- `readColors()`
- `readPolylines()`
- `resolveResources()`
- `resolveUrl(uri: string)`
- `getTextureType(sampler?: GltfSampler)`
- `findTextureMapping(id: string | undefined, isTransparent: boolean, normalMapId: string | undefined, constantLodParamProps: TextureMapping.ConstantLodParamProps | undefined, normalMapUseConstantLod?: boolean)`
