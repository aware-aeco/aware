# mesh-set-cached-texture-coordinates-from-material

Lifecycle: single

Sets up cached texture coordinate set for each texture in the material.              Textures in the Material define which mapping channels are used and              the RhinoObject defines what texture mapping is used for each mapping channel.              After this method is called all necessary texture coordinate sets are cached              and correct texture coordinates for each texture can be fetched using               GetCachedTextureCoordinates(RhinoObject rhinoObject, Rhino.DocObjects.Texture texture)                           If any texture coordinates are already cached they will not be re-computed.
