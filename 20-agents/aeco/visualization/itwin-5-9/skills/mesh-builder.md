---
name: core-frontend-mesh-builder
description: MeshBuilder declarations from core-frontend
---

# MeshBuilder

## Methods

- `create(props: MeshBuilder.Props)`
- `addStrokePointLists(strokes: StrokesPrimitivePointLists, isDisjoint: boolean, fillColor: number, feature: Feature | undefined)`
- `addFromPolyface(polyface: IndexedPolyface, props: MeshBuilder.PolyfaceOptions, feature: Feature | undefined)`
- `addFromPolyfaceVisitor(visitor: PolyfaceVisitor, options: MeshBuilder.PolyfaceOptions, feature: Feature | undefined)`
- `createTriangleVertices(triangleIndex: number, visitor: PolyfaceVisitor, options: MeshBuilder.PolyfaceVisitorOptions, feature: Feature | undefined)`
- `createTriangle(triangleIndex: number, visitor: PolyfaceVisitor, options: MeshBuilder.PolyfaceVisitorOptions, feature: Feature | undefined)`
- `addPolyline(points: Point3d[], fillColor: number, feature: Feature | undefined)`
- `addPointString(points: Point3d[], fillColor: number, feature: Feature | undefined)`
- `beginPolyface(polyface: Polyface, options: MeshEdgeCreationOptions)`
- `endPolyface()`
- `addVertex(vertex: VertexKeyProps, addToMeshOnInsert?: boolean)`
- `addTriangle(triangle: Triangle)`
- `create(props: MeshBuilder.Props)`
- `addStrokePointLists(strokes: StrokesPrimitivePointLists, isDisjoint: boolean, fillColor: number, feature: Feature | undefined)`
- `addFromPolyface(polyface: IndexedPolyface, props: MeshBuilder.PolyfaceOptions, feature: Feature | undefined)`
- `addFromPolyfaceVisitor(visitor: PolyfaceVisitor, options: MeshBuilder.PolyfaceOptions, feature: Feature | undefined)`
- `createTriangleVertices(triangleIndex: number, visitor: PolyfaceVisitor, options: MeshBuilder.PolyfaceVisitorOptions, feature: Feature | undefined)`
- `createTriangle(triangleIndex: number, visitor: PolyfaceVisitor, options: MeshBuilder.PolyfaceVisitorOptions, feature: Feature | undefined)`
- `addPolyline(points: Point3d[], fillColor: number, feature: Feature | undefined)`
- `addPointString(points: Point3d[], fillColor: number, feature: Feature | undefined)`
- `beginPolyface(polyface: Polyface, options: MeshEdgeCreationOptions)`
- `endPolyface()`
- `addVertex(vertex: VertexKeyProps, addToMeshOnInsert?: boolean)`
- `addTriangle(triangle: Triangle)`
