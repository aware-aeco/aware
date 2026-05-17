---
name: core-frontend-decorate-context
description: DecorateContext declarations from core-frontend
---

# DecorateContext

## Methods

- `create(args: DecorateContextCreateArgs)`
- `createGraphicBuilder(type: GraphicType, transform?: Transform, id?: Id64String)`
- `createGraphic(options: Omit<ViewportGraphicBuilderOptions, "viewport">)`
- `addFromDecorator(decorator: ViewportDecorator)`
- `addDecorationFromBuilder(builder: GraphicBuilder)`
- `addDecoration(type: GraphicType, decoration: RenderGraphic)`
- `addCanvasDecoration(decoration: CanvasDecoration, atFront?: boolean)`
- `addHtmlDecoration(decoration: HTMLElement)`
- `drawStandardGrid(gridOrigin: Point3d, rMatrix: Matrix3d, spacing: XAndY, gridsPerRef: number, _isoGrid?: boolean, _fixedRepetitions?: Point2d)`
- `setSkyBox(graphic: RenderGraphic)`
- `setViewBackground(graphic: RenderGraphic)`
- `create(args: DecorateContextCreateArgs)`
- `createGraphicBuilder(type: GraphicType, transform?: Transform, id?: Id64String)`
- `createGraphic(options: Omit<ViewportGraphicBuilderOptions, "viewport">)`
- `addFromDecorator(decorator: ViewportDecorator)`
- `addDecorationFromBuilder(builder: GraphicBuilder)`
- `addDecoration(type: GraphicType, decoration: RenderGraphic)`
- `addCanvasDecoration(decoration: CanvasDecoration, atFront?: boolean)`
- `addHtmlDecoration(decoration: HTMLElement)`
- `drawStandardGrid(gridOrigin: Point3d, rMatrix: Matrix3d, spacing: XAndY, gridsPerRef: number, _isoGrid?: boolean, _fixedRepetitions?: Point2d)`
- `setSkyBox(graphic: RenderGraphic)`
- `setViewBackground(graphic: RenderGraphic)`
