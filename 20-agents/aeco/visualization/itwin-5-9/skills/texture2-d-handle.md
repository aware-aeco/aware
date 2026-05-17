---
name: core-frontend-texture2-d-handle
description: Texture2DHandle declarations from core-frontend
---

# Texture2DHandle

## Methods

- `bindTexture(texUnit: TextureUnit, glTex: WebGLTexture | undefined)`
- `bindSampler(uniform: UniformHandle, tex: WebGLTexture, unit: TextureUnit)`
- `bind(texUnit: TextureUnit)`
- `bindSampler(uniform: UniformHandle, unit: TextureUnit)`
- `update(updater: Texture2DDataUpdater)`
- `replaceTextureData(data: Texture2DData)`
- `createForAttachment(width: number, height: number, format: GL.Texture.Format, dataType: GL.Texture.DataType)`
- `createForData(width: number, height: number, data: Texture2DData, wantPreserveData?: boolean, wrapMode?: GL.Texture.WrapMode, format?: GL.Texture.Format)`
- `createForImageBuffer(image: ImageBuffer, type: RenderTexture.Type)`
- `createForImage(image: HTMLImageElement, type: RenderTexture.Type)`
- `createForImageBitmap(image: ImageBitmap, type: RenderTexture.Type)`
- `createForElement(id: Id64String, imodel: IModelConnection, type: RenderTexture.Type, format: ImageSourceFormat, onLoaded: ExternalTextureLoadCallback)`
- `reload(params: Texture2DCreateParams)`
- `bindTexture(texUnit: TextureUnit, glTex: WebGLTexture | undefined)`
- `bindSampler(uniform: UniformHandle, tex: WebGLTexture, unit: TextureUnit)`
- `bind(texUnit: TextureUnit)`
- `bindSampler(uniform: UniformHandle, unit: TextureUnit)`
- `update(updater: Texture2DDataUpdater)`
- `replaceTextureData(data: Texture2DData)`
- `createForAttachment(width: number, height: number, format: GL.Texture.Format, dataType: GL.Texture.DataType)`
- `createForData(width: number, height: number, data: Texture2DData, wantPreserveData?: boolean, wrapMode?: GL.Texture.WrapMode, format?: GL.Texture.Format)`
- `createForImageBuffer(image: ImageBuffer, type: RenderTexture.Type)`
- `createForImage(image: HTMLImageElement, type: RenderTexture.Type)`
- `createForImageBitmap(image: ImageBitmap, type: RenderTexture.Type)`
- `createForElement(id: Id64String, imodel: IModelConnection, type: RenderTexture.Type, format: ImageSourceFormat, onLoaded: ExternalTextureLoadCallback)`
- `reload(params: Texture2DCreateParams)`
