---
name: rhino-rhino-display
description: This skill encodes the rhino 7.0 surface of the Rhino.Display namespace — 59 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CalculateBoundingBoxEventArgs, ColorGradient, CullObjectEventArgs, CustomDisplay, DisplayBitmap, DisplayBitmapDrawList, DisplayConduit, DisplayEngine, and 51 more types.
---

# Rhino.Display

Auto-generated from vendor docs for rhino 7.0. 59 types in this namespace.

## BackgroundStyle (enum)

Constants that define how the background of a viewport should be filled.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_BackgroundStyle.htm)

### Values
- `SolidColor` = `0` — Single solid color fill.
- `WallpaperImage` = `1` — Simple image background wallpaper.
- `Gradient` = `2` — Two color top/bottom color gradient.
- `Environment` = `3` — Using a special environment.

## BlendMode (enum)

Defines enumerated constants for display blend modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_BlendMode.htm)

### Values
- `Zero` = `0` — Blends using 0.
- `One` = `1` — Blends using 1.
- `SourceColor` = `768` — Blends using source color.
- `OneMinusSourceColor` = `769` — Blends using 1-source color.
- `SourceAlpha` = `770` — Blends using the source alpha channel.
- `OneMinusSourceAlpha` = `771` — Blends using 1-the source alpha channel.
- `DestinationAlpha` = `772` — Blends using the destination alpha channel.
- `OneMinusDestinationAlpha` = `773` — Blends using 1-the destination alpha channel.
- `DestinationColor` = `774` — Blends using the destination color.
- `OneMinusDestinationColor` = `775` — Blends using 1-the destination color.
- `SourceAlphaSaturate` = `776` — Blends using the source alpha saturation.

## CalculateBoundingBoxEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.CalculateBoundingBoxEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_CalculateBoundingBoxEventArgs.htm)

### Methods
#### `public void IncludeBoundingBox(BoundingBox box)`

Unites a bounding box with the current display bounding box in order to ensure dynamic objects in "box" are drawn.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — The box to unite.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CalculateBoundingBoxEventArgs_IncludeBoundingBox.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — Gets the current bounding box.
- `Display` (DisplayPipeline, get) — (Inherited from DrawEventArgs.)
- `RhinoDoc` (RhinoDoc, get) — (Inherited from DrawEventArgs.)
- `Viewport` (RhinoViewport, get) — (Inherited from DrawEventArgs.)

## Color4f (struct)

Color defined by 4 floating point values.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_Color4f.htm)

### Constructors
- `public Color4f(Color color)` — Initializes a new instance of the Color4f class
- `public Color4f(int argb)` — Initializes a new instance of the Color4f class
- `public Color4f(Color4f color)` — Initializes a new instance of the Color4f class
- `public Color4f(float red, float green, float blue, float alpha)` — Initializes a new instance of the Color4f class

### Methods
#### `public static Color4f ApplyGamma(Color4f col, float gamma)`



**Parameters:**
- `col` (Rhino.Display.Color4f) — [Missing <param name="col"/> documentation for "M:Rhino.Display.Color4f.ApplyGamma(Rhino.Display.Color4f,System.Single)"]
- `gamma` (System.Single) — [Missing <param name="gamma"/> documentation for "M:Rhino.Display.Color4f.ApplyGamma(Rhino.Display.Color4f,System.Single)"]

**Returns:** `Color4f` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.ApplyGamma(Rhino.Display.Color4f,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_ApplyGamma.htm)

#### `public Color AsSystemColor()`



**Returns:** `Color` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.AsSystemColor"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_AsSystemColor.htm)

#### `public Color4f BlendTo(float t, Color4f col)`



**Parameters:**
- `t` (System.Single) — [Missing <param name="t"/> documentation for "M:Rhino.Display.Color4f.BlendTo(System.Single,Rhino.Display.Color4f)"]
- `col` (Rhino.Display.Color4f) — [Missing <param name="col"/> documentation for "M:Rhino.Display.Color4f.BlendTo(System.Single,Rhino.Display.Color4f)"]

**Returns:** `Color4f` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.BlendTo(System.Single,Rhino.Display.Color4f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_BlendTo.htm)

#### `public static bool operator ==(Color4f a, Color4f b)`



**Parameters:**
- `a` (Rhino.Display.Color4f) — [Missing <param name="a"/> documentation for "M:Rhino.Display.Color4f.op_Equality(Rhino.Display.Color4f,Rhino.Display.Color4f)"]
- `b` (Rhino.Display.Color4f) — [Missing <param name="b"/> documentation for "M:Rhino.Display.Color4f.op_Equality(Rhino.Display.Color4f,Rhino.Display.Color4f)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.op_Equality(Rhino.Display.Color4f,Rhino.Display.Color4f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_op_Equality.htm)

#### `public override bool Equals(Object obj)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `obj` (System.Object) — [Missing <param name="obj"/> documentation for "M:Rhino.Display.Color4f.Equals(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.Equals(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_Equals.htm)

#### `public static Color4f FromArgb(float a, Color4f color)`



**Parameters:**
- `a` (System.Single) — [Missing <param name="a"/> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,Rhino.Display.Color4f)"]
- `color` (Rhino.Display.Color4f) — [Missing <param name="color"/> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,Rhino.Display.Color4f)"]

**Returns:** `Color4f` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,Rhino.Display.Color4f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_FromArgb.htm)

#### `public static Color4f FromArgb(float a, float r, float g, float b)`



**Parameters:**
- `a` (System.Single) — [Missing <param name="a"/> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,System.Single,System.Single,System.Single)"]
- `r` (System.Single) — [Missing <param name="r"/> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,System.Single,System.Single,System.Single)"]
- `g` (System.Single) — [Missing <param name="g"/> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,System.Single,System.Single,System.Single)"]
- `b` (System.Single) — [Missing <param name="b"/> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,System.Single,System.Single,System.Single)"]

**Returns:** `Color4f` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.FromArgb(System.Single,System.Single,System.Single,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_FromArgb_1.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_GetHashCode.htm)

#### `public static bool operator !=(Color4f a, Color4f b)`



**Parameters:**
- `a` (Rhino.Display.Color4f) — [Missing <param name="a"/> documentation for "M:Rhino.Display.Color4f.op_Inequality(Rhino.Display.Color4f,Rhino.Display.Color4f)"]
- `b` (Rhino.Display.Color4f) — [Missing <param name="b"/> documentation for "M:Rhino.Display.Color4f.op_Inequality(Rhino.Display.Color4f,Rhino.Display.Color4f)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.Color4f.op_Inequality(Rhino.Display.Color4f,Rhino.Display.Color4f)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Color4f_op_Inequality.htm)

### Properties
- `A` (Single, get) — 
- `B` (Single, get) — 
- `Black` (Color4f, get) — 
- `Empty` (Color4f, get) — 
- `G` (Single, get) — 
- `L` (Single, get) — 
- `R` (Single, get) — 
- `White` (Color4f, get) — 

## ColorCMYK (struct)

Represents a CMYK (Cyan, Magenta, Yellow, Key) color with double precision floating point channels. CMYK colors are used primarily in printing environments as they provide a good simulation of physical ink.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorCMYK.htm)

### Constructors
- `public ColorCMYK(Color rgb)` — Initializes a new instance of ColorCMYK that is equivalent to an ARGB color.
- `public ColorCMYK(double cyan, double magenta, double yellow)` — Initializes a new instance of ColorCMYK with custom channel values. The cyan, magenta and yellow values will be adjusted based on their combined darkness.
- `public ColorCMYK(double cyan, double magenta, double yellow, double key)` — Initializes a new instance of ColorCMYK with custom channel values.
- `public ColorCMYK(double alpha, double cyan, double magenta, double yellow, double key)` — Initializes a new instance of ColorCMYK with custom channel values.

### Methods
#### `public static ColorCMYK CreateFromHSL(ColorHSL hsl)`

Constructs the nearest CMYK equivalent of an HSL color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — Target color in HSL space.

**Returns:** `ColorCMYK` — The CMYK equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_CreateFromHSL.htm)

#### `public static ColorCMYK CreateFromHSV(ColorHSV hsv)`

Constructs the nearest CMYK equivalent of an HSV color.

**Parameters:**
- `hsv` (Rhino.Display.ColorHSV) — Target color in HSV space.

**Returns:** `ColorCMYK` — The CMYK equivalent of the HSV color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_CreateFromHSV.htm)

#### `public static ColorCMYK CreateFromLAB(ColorLAB lab)`

Constructs the nearest CMYK equivalent of a LAB color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — Target color in LAB space.

**Returns:** `ColorCMYK` — The CMYK equivalent of the LAB color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_CreateFromLAB.htm)

#### `public static ColorCMYK CreateFromLCH(ColorLCH lch)`

Constructs the nearest CMYK equivalent of a LCH color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — Target color in LCH space.

**Returns:** `ColorCMYK` — The CMYK equivalent of the LCH color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_CreateFromLCH.htm)

#### `public static ColorCMYK CreateFromRGBA(ColorRGBA rgba)`

Create the nearest CMYK equivalent of a RGBA color.

**Parameters:**
- `rgba` (Rhino.Display.ColorRGBA) — Target color in RGBA space.

**Returns:** `ColorCMYK` — The CMYK equivalent of the RGBA color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_CreateFromRGBA.htm)

#### `public static ColorCMYK CreateFromXYZ(ColorXYZ xyz)`

Constructs the nearest CMYK equivalent of an XYZ color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — Target color in XYZ space.

**Returns:** `ColorCMYK` — The CMYK equivalent of the XYZ color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_CreateFromXYZ.htm)

#### `public static implicit operator Color (ColorCMYK cmyk)`

Implicitly converts a CMYK color into a .Net library color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — A CMYK color.

**Returns:** `Color` — A ARGB .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorCMYK_op_Implicit.htm)

### Properties
- `A` (Double, get/set) — Gets or sets the Alpha channel value. Alpha channels are limited to the 0~1 range.
- `C` (Double, get/set) — Gets or sets the Cyan channel value. Cyan channels are limited to the 0~1 range.
- `K` (Double, get/set) — Gets or sets the Key channel value. Key channels are limited to the 0~1 range.
- `M` (Double, get/set) — Gets or sets the Magenta channel value. Magenta channels are limited to the 0~1 range.
- `Y` (Double, get/set) — Gets or sets the Yellow channel value. Yellow channels are limited to the 0~1 range.

## ColorGradient (class)

[Missing <summary> documentation for "T:Rhino.Display.ColorGradient"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorGradient.htm)

### Constructors
- `public ColorGradient()` — Initializes a new instance of the ColorGradient class

### Methods
#### `public ColorStop[] GetColorStops()`

Get sorted list of colors / positions that a gradient is defined over

**Returns:** `ColorStop[]` — [Missing <returns> documentation for "M:Rhino.Display.ColorGradient.GetColorStops"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorGradient_GetColorStops.htm)

#### `public void SetColorStops(IEnumerable<ColorStop> stops)`

Set color stops for the gradient

**Parameters:**
- `stops` (System.Collections.Generic.IEnumerable<ColorStop>) — [Missing <param name="stops"/> documentation for "M:Rhino.Display.ColorGradient.SetColorStops(System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorGradient_SetColorStops.htm)

### Properties
- `EndPoint` (Point3d, get/set) — End point of gradient
- `GradientType` (GradientType, get/set) — Gradient fill type associated with this hatch
- `Repeat` (Double, get/set) — Repeat factor for gradient. Factors greater than 1 define a reflected repeat factor while values less than -1 define a wrapped repeat factor.
- `StartPoint` (Point3d, get/set) — Start point of gradient

## ColorHSL (struct)

Represents an HSL (Hue, Saturation, Luminance) color with double precision floating point channels. HSL colors are used primarily in Graphical User Interface environments as they provide a very natural approach to picking colors.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorHSL.htm)

### Constructors
- `public ColorHSL(Color rgb)` — Constructs a new instance of ColorHSL that is equivalent to an ARGB color.
- `public ColorHSL(double hue, double saturation, double luminance)` — Constructs a new instance of ColorHSL with custom channel values.
- `public ColorHSL(double alpha, double hue, double saturation, double luminance)` — Constructs a new instance of ColorHSL with custom channel values.

### Methods
#### `public static ColorHSL CreateFromCMYK(ColorCMYK cmyk)`

Create the nearest HSL equivalent of a CMYK color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — Target color in CMYK space.

**Returns:** `ColorHSL` — The HSL equivalent of the CMYK color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_CreateFromCMYK.htm)

#### `public static ColorHSL CreateFromHSV(ColorHSV hsv)`

Constructs the nearest HSL equivalent of an HSV color.

**Parameters:**
- `hsv` (Rhino.Display.ColorHSV) — Target color in HSV space.

**Returns:** `ColorHSL` — The HSL equivalent of the HSV color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_CreateFromHSV.htm)

#### `public static ColorHSL CreateFromLAB(ColorLAB lab)`

Create the nearest HSL equivalent of a LAB color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — Target color in LAB space.

**Returns:** `ColorHSL` — The HSL equivalent of the LAB color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_CreateFromLAB.htm)

#### `public static ColorHSL CreateFromLCH(ColorLCH lch)`

Create the nearest HSL equivalent of a LCH color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — Target color in LCH space.

**Returns:** `ColorHSL` — The HSL equivalent of the LCH color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_CreateFromLCH.htm)

#### `public static ColorHSL CreateFromRGBA(ColorRGBA rgba)`

Create the nearest HSL equivalent of a RGBA color.

**Parameters:**
- `rgba` (Rhino.Display.ColorRGBA) — Target color in RGBA space.

**Returns:** `ColorHSL` — The HSL equivalent of the RGBA color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_CreateFromRGBA.htm)

#### `public static ColorHSL CreateFromXYZ(ColorXYZ xyz)`

Create the nearest HSL equivalent of an XYZ color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — Target color in XYZ space.

**Returns:** `ColorHSL` — The HSL equivalent of the XYZ color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_CreateFromXYZ.htm)

#### `public static implicit operator Color (ColorHSL hsl)`

Implicitly converts a ColorHSL in a .Net library color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — A HSL color.

**Returns:** `Color` — A ARGB .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_op_Implicit.htm)

#### `public Color ToArgbColor()`

Convert HSL color to an equivalent System.Drawing.Color.

**Returns:** `Color` — A .Net framework library color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSL_ToArgbColor.htm)

### Properties
- `A` (Double, get/set) — Gets or sets the alpha channel value. Alpha channels are limited to a 0~1 range.
- `H` (Double, get/set) — Gets or sets the hue channel value. Hue channels rotate between 0.0 and 1.0.
- `L` (Double, get/set) — Gets or sets the luminance channel value. Luminance channels are limited to a 0~1 range.
- `S` (Double, get/set) — Gets or sets the saturation channel value. Saturation channels are limited to a 0~1 range.

## ColorHSV (struct)

Represents an HSV (Hue, Saturation, Value) color with double precision floating point channels. HSV colors (also sometimes called HSB, where B means Brightness) are similar to HSL colors in that they represent colors in a cylindrical color space, and are intended to provide intuitive means to edit the brightness of a particular color over RGB color space where each color channel would need to be modified to affect the color brightness.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorHSV.htm)

### Constructors
- `public ColorHSV(Color rgb)` — Constructs a new instance of ColorHSV that is equivalent to an ARGB color.
- `public ColorHSV(double hue, double saturation, double value)` — Constructs a new instance of ColorHSV with custom channel values.
- `public ColorHSV(double alpha, double hue, double saturation, double value)` — Constructs a new instance of ColorHSV with custom channel values.

### Methods
#### `public static ColorHSV CreateFromCMYK(ColorCMYK cmyk)`

Create the nearest HSV equivalent of a CMYK color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — Target color in CMYK space.

**Returns:** `ColorHSV` — The HSV equivalent of the CMYK color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_CreateFromCMYK.htm)

#### `public static ColorHSV CreateFromHSL(ColorHSL hsl)`

Constructs the nearest HSV equivalent of an HSL color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — Target color in HSL space.

**Returns:** `ColorHSV` — The HSV equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_CreateFromHSL.htm)

#### `public static ColorHSV CreateFromLAB(ColorLAB lab)`

Create the nearest HSV equivalent of a LAB color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — Target color in LAB space.

**Returns:** `ColorHSV` — The HSV equivalent of the LAB color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_CreateFromLAB.htm)

#### `public static ColorHSV CreateFromLCH(ColorLCH lch)`

Create the nearest HSV equivalent of a LCH color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — Target color in LCH space.

**Returns:** `ColorHSV` — The HSV equivalent of the LCH color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_CreateFromLCH.htm)

#### `public static ColorHSV CreateFromRGBA(ColorRGBA rgba)`

Create the nearest HSV equivalent of a RGBA color.

**Parameters:**
- `rgba` (Rhino.Display.ColorRGBA) — Target color in RGBA space.

**Returns:** `ColorHSV` — The HSV equivalent of the RGBA color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_CreateFromRGBA.htm)

#### `public static ColorHSV CreateFromXYZ(ColorXYZ xyz)`

Create the nearest HSV equivalent of an XYZ color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — Target color in XYZ space.

**Returns:** `ColorHSV` — The HSV equivalent of the XYZ color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_CreateFromXYZ.htm)

#### `public static implicit operator Color (ColorHSV hsv)`

Implicitly converts a ColorHSV in a .Net library color.

**Parameters:**
- `hsv` (Rhino.Display.ColorHSV) — A HSV color.

**Returns:** `Color` — A ARGB .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_op_Implicit.htm)

#### `public Color ToArgbColor()`

Convert HSV color to an equivalent System.Drawing.Color.

**Returns:** `Color` — A .Net framework library color value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorHSV_ToArgbColor.htm)

### Properties
- `A` (Double, get/set) — Gets or sets the alpha channel value. Alpha channels are limited to a 0~1 range.
- `H` (Double, get/set) — Gets or sets the hue channel value. Hue channels rotate between 0.0 and 1.0.
- `S` (Double, get/set) — Gets or sets the saturation channel value. Saturation channels are limited to a 0~1 range.
- `V` (Double, get/set) — Gets or sets the value (brightness) channel value. Value channels are limited to a 0~1 range.

## ColorLAB (struct)

Represents a LAB (Lightness, A, B) color with double precision floating point channels. LAB colors are based on nonlinearly compressed CIE XYZ color space coordinates. The A and B parameters of a LAB color represent the opponents.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorLAB.htm)

### Constructors
- `public ColorLAB(Color rgb)` — Constructs a new instance of ColorLAB that is equivalent to an ARGB color.
- `public ColorLAB(double lightness, double a, double b)` — Constructs a new instance of ColorLAB with custom channel values.
- `public ColorLAB(double alpha, double lightness, double a, double b)` — Constructs a new instance of ColorLAB with custom channel values.

### Methods
#### `public static ColorLAB CreateFromCMYK(ColorCMYK cmyk)`

Create the nearest LAB equivalent of a CMYK color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — Target color in CMYK space.

**Returns:** `ColorLAB` — The LAB equivalent of the CMYK color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_CreateFromCMYK.htm)

#### `public static ColorLAB CreateFromHSL(ColorHSL hsl)`

Create the nearest LAB equivalent of an HSL color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — Target color in HSL space.

**Returns:** `ColorLAB` — The LAB equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_CreateFromHSL.htm)

#### `public static ColorLAB CreateFromHSV(ColorHSV hsv)`

Constructs the nearest LAB equivalent of an HSV color.

**Parameters:**
- `hsv` (Rhino.Display.ColorHSV) — Target color in HSV space.

**Returns:** `ColorLAB` — The LAB equivalent of the HSV color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_CreateFromHSV.htm)

#### `public static ColorLAB CreateFromLCH(ColorLCH lch)`

Create the nearest LAB equivalent of an LCH color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — Target color in LCH space.

**Returns:** `ColorLAB` — The LAB equivalent of the LCH color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_CreateFromLCH.htm)

#### `public static ColorLAB CreateFromRGBA(ColorRGBA rgb)`

Create the nearest LAB equivalent of an RGBA color.

**Parameters:**
- `rgb` (Rhino.Display.ColorRGBA) — Target color in RGBA space.

**Returns:** `ColorLAB` — The LAB equivalent of the RGBA color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_CreateFromRGBA.htm)

#### `public static ColorLAB CreateFromXYZ(ColorXYZ xyz)`

Create the nearest LAB equivalent of an XYZ color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — Target color in XYZ space.

**Returns:** `ColorLAB` — The LAB equivalent of the XYZ color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_CreateFromXYZ.htm)

#### `public static implicit operator Color (ColorLAB lab)`

Implicitly converts a LAB color into a .Net library color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — A LAB color.

**Returns:** `Color` — A ARGB .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLAB_op_Implicit.htm)

### Properties
- `A` (Double, get/set) — Gets or sets the Base channel. The channel is limited to 0~1.
- `Alpha` (Double, get/set) — Gets or sets the Alpha channel. The channel is limited to 0~1.
- `B` (Double, get/set) — Gets or sets the Opponent channel. The channel is limited to 0~1.
- `L` (Double, get/set) — Gets or sets the lightness channel. The channel is limited to 0~1.

## ColorLCH (struct)

Represents an LCH (Lightness, A, B) color with double precision floating point channels. LCH colors (also sometimes called CIELUV) are transformation of the 1931 CIE XYZ color space, in order to approach perceptual uniformity. They are primarily used in computer graphics which deal with colored lights.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorLCH.htm)

### Constructors
- `public ColorLCH(Color rgb)` — Constructs a new instance of ColorLCH that is equivalent to an ARGB color.
- `public ColorLCH(double lightness, double chroma, double hue)` — Constructs a new instance of ColorLCH with custom channel values.
- `public ColorLCH(double alpha, double lightness, double chroma, double hue)` — Constructs a new instance of ColorLCH with custom channel values.

### Methods
#### `public static ColorLCH CreateFromCMYK(ColorCMYK cmyk)`

Create the nearest LCH equivalent of a CMYK color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — Target color in CMYK space.

**Returns:** `ColorLCH` — The LCH equivalent of the CMYK color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_CreateFromCMYK.htm)

#### `public static ColorLCH CreateFromHSL(ColorHSL hsl)`

Create the nearest LCH equivalent of an HSL color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — Target color in HSL space.

**Returns:** `ColorLCH` — The LCH equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_CreateFromHSL.htm)

#### `public static ColorLCH CreateFromLAB(ColorLAB lab)`

Create the nearest LCH equivalent of a LAB color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — Target color in LAB space.

**Returns:** `ColorLCH` — The LCH equivalent of the LAB color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_CreateFromLAB.htm)

#### `public static ColorLCH CreateFromRGBA(ColorRGBA rgb)`

Create the nearest LCH equivalent of an RGBA color.

**Parameters:**
- `rgb` (Rhino.Display.ColorRGBA) — Target color in RGBA space.

**Returns:** `ColorLCH` — The LCH equivalent of the RGBA color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_CreateFromRGBA.htm)

#### `public static ColorLCH CreateFromXYZ(ColorXYZ xyz)`

Create the nearest LCH equivalent of an XYZ color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — Target color in XYZ space.

**Returns:** `ColorLCH` — The LCH equivalent of the XYZ color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_CreateFromXYZ.htm)

#### `public static implicit operator Color (ColorLCH lch)`

Implicitly converts a LCH color into a .Net library color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — A LCH color.

**Returns:** `Color` — A ARGB .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_op_Implicit.htm)

#### `public void MakePositive()`

Ensure the Chromaticity of this color is positive.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorLCH_MakePositive.htm)

### Properties
- `A` (Double, get/set) — Gets or sets the Alpha channel. The Alpha channel is limited to the 0~1 range.
- `C` (Double, get/set) — Gets or sets the Chroma channel. Chroma is defined from -1.0 to +1.0.
- `H` (Double, get/set) — Gets or sets the Hue channel. The hue channel is limited to the 0~360 degree range.
- `L` (Double, get/set) — Gets or sets the Lightness channel.

## ColorRGBA (struct)

Represents a RGBA (Red, Green, Blue, Alpha) color with double precision floating point channel.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorRGBA.htm)

### Constructors
- `public ColorRGBA(Color color)` — Constructs a new instance of ColorRGBA that is equivalent to an ARGB color.
- `public ColorRGBA(int argb)` — Constructs a new instance of ColorRGBA that is equivalent to an ARGB color.
- `public ColorRGBA(ColorRGBA color)` — Initializes a new instance of the ColorRGBA class
- `public ColorRGBA(double red, double green, double blue)` — Initializes a new instance of the ColorRGBA class
- `public ColorRGBA(double red, double green, double blue, double alpha)` — Initializes a new instance of the ColorRGBA class

### Methods
#### `public static ColorRGBA ApplyGamma(ColorRGBA col, double gamma)`



**Parameters:**
- `col` (Rhino.Display.ColorRGBA) — [Missing <param name="col"/> documentation for "M:Rhino.Display.ColorRGBA.ApplyGamma(Rhino.Display.ColorRGBA,System.Double)"]
- `gamma` (System.Double) — [Missing <param name="gamma"/> documentation for "M:Rhino.Display.ColorRGBA.ApplyGamma(Rhino.Display.ColorRGBA,System.Double)"]

**Returns:** `ColorRGBA` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.ApplyGamma(Rhino.Display.ColorRGBA,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_ApplyGamma.htm)

#### `public ColorRGBA BlendTo(ColorRGBA col, double coefficient)`



**Parameters:**
- `col` (Rhino.Display.ColorRGBA) — [Missing <param name="col"/> documentation for "M:Rhino.Display.ColorRGBA.BlendTo(Rhino.Display.ColorRGBA,System.Double)"]
- `coefficient` (System.Double) — [Missing <param name="coefficient"/> documentation for "M:Rhino.Display.ColorRGBA.BlendTo(Rhino.Display.ColorRGBA,System.Double)"]

**Returns:** `ColorRGBA` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.BlendTo(Rhino.Display.ColorRGBA,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_BlendTo.htm)

#### `public int CompareTo(ColorRGBA other)`

Compares this ColorRGBA with another ColorRGBA. Channels evaluation priority is first A, then R, then G, then B.

**Parameters:**
- `other` (Rhino.Display.ColorRGBA) — The other ColorRGBA to use in comparison.

**Returns:** `Int32` — 0: if this is identical to other-1: if this < other+1: if this > other

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CompareTo.htm)

#### `public static ColorRGBA CreateFromCMYK(ColorCMYK cmyk)`

Create the nearest RGBA equivalent of a CMYK color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — Target color in CMYK space.

**Returns:** `ColorRGBA` — The RGBA equivalent of the CMYK color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CreateFromCMYK.htm)

#### `public static ColorRGBA CreateFromHSL(ColorHSL hsl)`

Constructs the nearest RGBA equivalent of an HSV color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — Target color in HSL space.

**Returns:** `ColorRGBA` — The RGBA equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CreateFromHSL.htm)

#### `public static ColorRGBA CreateFromHSV(ColorHSV hsv)`

Constructs the nearest RGBA equivalent of an HSL color.

**Parameters:**
- `hsv` (Rhino.Display.ColorHSV) — Target color in HSL space.

**Returns:** `ColorRGBA` — The RGBA equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CreateFromHSV.htm)

#### `public static ColorRGBA CreateFromLAB(ColorLAB lab)`

Create the nearest RGBA equivalent of a LAB color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — Target color in LAB space.

**Returns:** `ColorRGBA` — The RGBA equivalent of the LAB color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CreateFromLAB.htm)

#### `public static ColorRGBA CreateFromLCH(ColorLCH lch)`

Create the nearest RGBA equivalent of a LCH color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — Target color in LCH space.

**Returns:** `ColorRGBA` — The RGBA equivalent of the LCH color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CreateFromLCH.htm)

#### `public static ColorRGBA CreateFromXYZ(ColorXYZ xyz)`

Create the nearest RGBA equivalent of an XYZ color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — Target color in XYZ space.

**Returns:** `ColorRGBA` — The RGBA equivalent of the XYZ color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_CreateFromXYZ.htm)

#### `public bool EpsilonEquals(ColorRGBA other, double epsilon)`

Check that all values in other are within epsilon of the values in this

**Parameters:**
- `other` (Rhino.Display.ColorRGBA) — [Missing <param name="other"/> documentation for "M:Rhino.Display.ColorRGBA.EpsilonEquals(Rhino.Display.ColorRGBA,System.Double)"]
- `epsilon` (System.Double) — [Missing <param name="epsilon"/> documentation for "M:Rhino.Display.ColorRGBA.EpsilonEquals(Rhino.Display.ColorRGBA,System.Double)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.EpsilonEquals(Rhino.Display.ColorRGBA,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_EpsilonEquals.htm)

#### `public static bool operator ==(ColorRGBA a, ColorRGBA b)`



**Parameters:**
- `a` (Rhino.Display.ColorRGBA) — [Missing <param name="a"/> documentation for "M:Rhino.Display.ColorRGBA.op_Equality(Rhino.Display.ColorRGBA,Rhino.Display.ColorRGBA)"]
- `b` (Rhino.Display.ColorRGBA) — [Missing <param name="b"/> documentation for "M:Rhino.Display.ColorRGBA.op_Equality(Rhino.Display.ColorRGBA,Rhino.Display.ColorRGBA)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.op_Equality(Rhino.Display.ColorRGBA,Rhino.Display.ColorRGBA)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_op_Equality.htm)

#### `public bool Equals(ColorRGBA other)`

Determines whether the specified ColorRGBA has the same values as the present color.

**Parameters:**
- `other` (Rhino.Display.ColorRGBA) — The specified color.

**Returns:** `Boolean` — true if color has the same channel values as this; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_Equals.htm)

#### `public override bool Equals(Object obj)`

(Overrides ValueType.Equals(Object).)

**Parameters:**
- `obj` (System.Object) — [Missing <param name="obj"/> documentation for "M:Rhino.Display.ColorRGBA.Equals(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.Equals(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_Equals_1.htm)

#### `public static explicit operator Color (ColorRGBA value)`

Explicitly converts a ColorRGBA in a Color.

**Parameters:**
- `value` (Rhino.Display.ColorRGBA) — A RGBA color.

**Returns:** `Color` — An ARGB Color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_op_Explicit.htm)

#### `public override int GetHashCode()`

(Overrides ValueType.GetHashCode().)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_GetHashCode.htm)

#### `public static implicit operator ColorRGBA (Color value)`



**Parameters:**
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.Display.ColorRGBA.op_Implicit(System.Drawing.Color)~Rhino.Display.ColorRGBA"]

**Returns:** `ColorRGBA` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.op_Implicit(System.Drawing.Color)~Rhino.Display.ColorRGBA"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_op_Implicit.htm)

#### `public static bool operator !=(ColorRGBA a, ColorRGBA b)`



**Parameters:**
- `a` (Rhino.Display.ColorRGBA) — [Missing <param name="a"/> documentation for "M:Rhino.Display.ColorRGBA.op_Inequality(Rhino.Display.ColorRGBA,Rhino.Display.ColorRGBA)"]
- `b` (Rhino.Display.ColorRGBA) — [Missing <param name="b"/> documentation for "M:Rhino.Display.ColorRGBA.op_Inequality(Rhino.Display.ColorRGBA,Rhino.Display.ColorRGBA)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.op_Inequality(Rhino.Display.ColorRGBA,Rhino.Display.ColorRGBA)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_op_Inequality.htm)

#### `public override string ToString()`

(Overrides ValueType.ToString().)

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_ToString.htm)

#### `public string ToString(string format, IFormatProvider formatProvider)`



**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.Display.ColorRGBA.ToString(System.String,System.IFormatProvider)"]
- `formatProvider` (System.IFormatProvider) — [Missing <param name="formatProvider"/> documentation for "M:Rhino.Display.ColorRGBA.ToString(System.String,System.IFormatProvider)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Display.ColorRGBA.ToString(System.String,System.IFormatProvider)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorRGBA_ToString_1.htm)

### Properties
- `A` (Double, get/set) — Gets or sets the alpha channel value. Alpha channels are limited to a 0~1 range.
- `B` (Double, get/set) — Gets or sets the blue channel value. Blue channels are limited to a 0~1 range.
- `Black` (ColorRGBA, get) — 
- `Blue` (ColorRGBA, get) — 
- `G` (Double, get/set) — Gets or sets the green channel value. Green channels are limited to a 0~1 range.
- `Green` (ColorRGBA, get) — 
- `R` (Double, get/set) — Gets or sets the red channel value. Red channels are limited to a 0~1 range.
- `Red` (ColorRGBA, get) — 
- `White` (ColorRGBA, get) — 

## ColorStop (struct)

Combination of a color and position. Used in defining gradient fills

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorStop.htm)

### Constructors
- `public ColorStop(Color color, double t)` — Create color stop from a color and position

### Properties
- `Color` (Color, get/set) — 
- `Position` (Double, get/set) — Parameter that Color is defined at

## ColorXYZ (struct)

Represents an XYZ (Hue, Saturation, Luminance) color with double precision floating point channels. XYZ colors are based on the CIE 1931 XYZ color space standard and they mimic the natural sensitivity of cones in the human retina.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ColorXYZ.htm)

### Constructors
- `public ColorXYZ(Color rgb)` — Constructs a new instance of ColorXYZ that is equivalent to an ARGB color.
- `public ColorXYZ(double x, double y, double z)` — Constructs a new instance of ColorXYZ with custom channel values.
- `public ColorXYZ(double alpha, double x, double y, double z)` — Constructs a new instance of ColorXYZ with custom channel values.

### Methods
#### `public static ColorXYZ CreateFromCMYK(ColorCMYK cmyk)`

Create the nearest XYZ equivalent of a CMYK color.

**Parameters:**
- `cmyk` (Rhino.Display.ColorCMYK) — Target color in CMYK space.

**Returns:** `ColorXYZ` — The XYZ equivalent of the CMYK color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_CreateFromCMYK.htm)

#### `public static ColorXYZ CreateFromHSL(ColorHSL hsl)`

Create the nearest XYZ equivalent of an HSL color.

**Parameters:**
- `hsl` (Rhino.Display.ColorHSL) — Target color in HSL space.

**Returns:** `ColorXYZ` — The XYZ equivalent of the HSL color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_CreateFromHSL.htm)

#### `public static ColorXYZ CreateFromHSV(ColorHSV hsv)`

Constructs the nearest XYZ equivalent of an HSV color.

**Parameters:**
- `hsv` (Rhino.Display.ColorHSV) — Target color in HSV space.

**Returns:** `ColorXYZ` — The XYZ equivalent of the HSV color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_CreateFromHSV.htm)

#### `public static ColorXYZ CreateFromLAB(ColorLAB lab)`

Create the nearest XYZ equivalent of a Lab color.

**Parameters:**
- `lab` (Rhino.Display.ColorLAB) — Target color in LAB space.

**Returns:** `ColorXYZ` — The XYZ equivalent of the LAB color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_CreateFromLAB.htm)

#### `public static ColorXYZ CreateFromLCH(ColorLCH lch)`

Create the nearest XYZ equivalent of an LCH color.

**Parameters:**
- `lch` (Rhino.Display.ColorLCH) — Target color in LCH space.

**Returns:** `ColorXYZ` — The XYZ equivalent of the LCH color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_CreateFromLCH.htm)

#### `public static ColorXYZ CreateFromRGBA(ColorRGBA rgba)`

Create the nearest XYZ equivalent of a RGBA color.

**Parameters:**
- `rgba` (Rhino.Display.ColorRGBA) — Target color in RGBA space.

**Returns:** `ColorXYZ` — The XYZ equivalent of the RGBA color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_CreateFromRGBA.htm)

#### `public static implicit operator Color (ColorXYZ xyz)`

Implicitly converts a XYZ color into a .Net library color.

**Parameters:**
- `xyz` (Rhino.Display.ColorXYZ) — A XYZ color.

**Returns:** `Color` — A ARGB .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ColorXYZ_op_Implicit.htm)

### Properties
- `A` (Double, get/set) — Gets or set the Alpha channel value. Channel will be limited to 0~1.
- `X` (Double, get/set) — Gets or set the X channel value. Channel will be limited to 0~1.
- `Y` (Double, get/set) — Gets or set the Y channel value. Channel will be limited to 0~1.
- `Z` (Double, get/set) — Gets or set the Z channel value. Channel will be limited to 0~1.

## CullFaceMode (enum)

[Missing <summary> documentation for "T:Rhino.Display.CullFaceMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_CullFaceMode.htm)

### Values
- `DrawFrontAndBack` = `0`
- `DrawFrontFaces` = `1`
- `DrawBackFaces` = `2`

## CullObjectEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.CullObjectEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_CullObjectEventArgs.htm)

### Properties
- `CullObject` (Boolean, get/set) — 
- `Display` (DisplayPipeline, get) — (Inherited from DrawEventArgs.)
- `RhinoDoc` (RhinoDoc, get) — (Inherited from DrawEventArgs.)
- `RhinoObject` (RhinoObject, get) — 
- `RhinoObjectSerialNumber` (UInt32, get) — Gets the rhino object runtime serial number.
- `Viewport` (RhinoViewport, get) — (Inherited from DrawEventArgs.)

## CustomDisplay (class)

Provides some basic (indeed, very basic) mechanisms for drawing custom geometry in viewports.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_CustomDisplay.htm)

### Constructors
- `public CustomDisplay(bool enable)` — Constructs a new CustomDisplay instance. You must call Dispose() when you are done with this instance, otherwise the display methods will never be switched off.

### Methods
#### `public void AddArc(Arc arc)`

Adds a new, black arc to the display list.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — Arc to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddArc.htm)

#### `public void AddArc(Arc arc, Color color)`

Adds a new, colored arc to the display list.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — Arc to add.
- `color` (System.Drawing.Color) — Color of arc.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddArc_1.htm)

#### `public void AddArc(Arc arc, Color color, int thickness)`

Adds a new, colored arc to the display list.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — Arc to add.
- `color` (System.Drawing.Color) — Color of arc.
- `thickness` (System.Int32) — Thickness of arc.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddArc_2.htm)

#### `public void AddCircle(Circle circle)`

Adds a new, black circle to the display list.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — Circle to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddCircle.htm)

#### `public void AddCircle(Circle circle, Color color)`

Adds a new, colored arc to the display list.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — Circle to add.
- `color` (System.Drawing.Color) — Color of circle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddCircle_1.htm)

#### `public void AddCircle(Circle circle, Color color, int thickness)`

Adds a new, colored circle to the display list.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — Circle to add.
- `color` (System.Drawing.Color) — Color of circle.
- `thickness` (System.Int32) — Thickness of circle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddCircle_2.htm)

#### `public void AddCurve(Curve curve)`

Adds a new, black curve to the display list. The curve will be duplicated so changes to the original will not affect the display.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddCurve.htm)

#### `public void AddCurve(Curve curve, Color color)`

Adds a new, colored curve to the display list. The curve will be duplicated so changes to the original will not affect the display.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve to add.
- `color` (System.Drawing.Color) — Color of curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddCurve_1.htm)

#### `public void AddCurve(Curve curve, Color color, int thickness)`

Adds a new, colored curve to the display list. The curve will be duplicated so changes to the original will not affect the display.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve to add.
- `color` (System.Drawing.Color) — Color of curve.
- `thickness` (System.Int32) — Thickness of curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddCurve_2.htm)

#### `public void AddLine(Line line)`

Adds a new, black line to the display list.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddLine.htm)

#### `public void AddLine(Line line, Color color)`

Adds a new, colored line to the display list.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to add.
- `color` (System.Drawing.Color) — Color of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddLine_1.htm)

#### `public void AddLine(Line line, Color color, int thickness)`

Adds a new, colored line to the display list.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to add.
- `color` (System.Drawing.Color) — Color of line.
- `thickness` (System.Int32) — Thickness of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddLine_2.htm)

#### `public void AddPoint(Point3d point)`

Adds a new, black point to the display list.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Point to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPoint.htm)

#### `public void AddPoint(Point3d point, Color color)`

Adds a new colored point to the display list.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Point to add.
- `color` (System.Drawing.Color) — Color of point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPoint_1.htm)

#### `public void AddPoint(Point3d point, Color color, PointStyle style, int radius)`

Adds a new stylized point to the display list.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Point to add.
- `color` (System.Drawing.Color) — Color of point.
- `style` (Rhino.Display.PointStyle) — Display style of point.
- `radius` (System.Int32) — Radius of point widget.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPoint_2.htm)

#### `public void AddPoints(IEnumerable<Point3d> points)`

Adds a collection of black points to the display list.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPoints.htm)

#### `public void AddPoints(IEnumerable<Point3d> points, Color color)`

Adds a collection of colored points to the display list.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.
- `color` (System.Drawing.Color) — Color of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPoints_1.htm)

#### `public void AddPoints(IEnumerable<Point3d> points, Color color, PointStyle style, int radius)`

Adds a collection of stylized points to the display list.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.
- `color` (System.Drawing.Color) — Color of points.
- `style` (Rhino.Display.PointStyle) — Display style of points.
- `radius` (System.Int32) — Radius of point widgets.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPoints_2.htm)

#### `public void AddPolygon(IEnumerable<Point3d> polygon, Color fillColor, Color edgeColor, bool drawFill, bool drawEdge)`

Adds a polygon to the drawing list. Polygons are not like Hatches, when you supply a concave polygon, the shading probably won't work.

**Parameters:**
- `polygon` (System.Collections.Generic.IEnumerable<Point3d>) — Points that define the corners of the polygon.
- `fillColor` (System.Drawing.Color) — Fill color of polygon.
- `edgeColor` (System.Drawing.Color) — Edge color of polygon.
- `drawFill` (System.Boolean) — If true, the polygon contents will be drawn.
- `drawEdge` (System.Boolean) — If true, the polygon edge will be drawn.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddPolygon.htm)

#### `public void AddText(string text, Plane plane, double size)`

Adds a new, black 3D text object to the display list.

**Parameters:**
- `text` (System.String) — Text to add.
- `plane` (Rhino.Geometry.Plane) — Plane for text orientation.
- `size` (System.Double) — Height (in units) of font.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddText_1.htm)

#### `public void AddText(string text, Plane plane, double size, Color color)`

Adds a new, colored 3D text object to the display list.

**Parameters:**
- `text` (System.String) — Text to add.
- `plane` (Rhino.Geometry.Plane) — Plane for text orientation.
- `size` (System.Double) — Height (in units) of font.
- `color` (System.Drawing.Color) — Color of text.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddText_2.htm)

#### `public void AddText(Text3d text, Color color)`

Adds a new 3D text object to the display list.

**Parameters:**
- `text` (Rhino.Display.Text3d) — Text object to add.
- `color` (System.Drawing.Color) — Color of text object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddText.htm)

#### `public void AddVector(Point3d anchor, Vector3d span)`

Adds a new, black vector to the display list.

**Parameters:**
- `anchor` (Rhino.Geometry.Point3d) — Anchor point of vector.
- `span` (Rhino.Geometry.Vector3d) — Direction and magnitude of vector.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddVector.htm)

#### `public void AddVector(Point3d anchor, Vector3d span, Color color)`

Adds a new, colored vector to the display list.

**Parameters:**
- `anchor` (Rhino.Geometry.Point3d) — Anchor point of vector.
- `span` (Rhino.Geometry.Vector3d) — Direction and magnitude of vector.
- `color` (System.Drawing.Color) — Color of vector.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddVector_1.htm)

#### `public void AddVector(Point3d anchor, Vector3d span, Color color, bool drawAnchor)`

Adds a new, colored vector to the display list.

**Parameters:**
- `anchor` (Rhino.Geometry.Point3d) — Anchor point of vector.
- `span` (Rhino.Geometry.Vector3d) — Direction and magnitude of vector.
- `color` (System.Drawing.Color) — Color of vector.
- `drawAnchor` (System.Boolean) — Include a point at the vector anchor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_AddVector_2.htm)

#### `public void Clear()`

Clear the drawing lists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_Clear.htm)

#### `public void Dispose()`

Dispose this CustomDisplay instance. You must call this function in order to properly shut down the CustomDisplay.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_CustomDisplay_Dispose.htm)

### Properties
- `ClippingBox` (BoundingBox, get) — Gets the clipping box of this CustomDisplay.
- `Enabled` (Boolean, get/set) — Gets or sets the Enabled state of this CustomDisplay instance. If you wish to terminate this CustomDisplay, place a call to Dispose() instead.
- `IsDisposed` (Boolean, get) — Gets a value indicating whether this CustomDisplay instance has been disposed. Once a CustomDisplay has been disposed, you can no longer use it.

## DefinedViewportProjection (enum)

Parallel and perspective projections that are "standard" in Rhino

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DefinedViewportProjection.htm)

### Values
- `None` = `0`
- `Top` = `1`
- `Bottom` = `2`
- `Left` = `3`
- `Right` = `4`
- `Front` = `5`
- `Back` = `6`
- `Perspective` = `7`
- `TwoPointPerspective` = `8`

## DepthMode (enum)

[Missing <summary> documentation for "T:Rhino.Display.DepthMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DepthMode.htm)

### Values
- `Neutral` = `0`
- `AlwaysInFront` = `1`
- `AlwaysInBack` = `2`

## DisplayBitmap (class)

A bitmap resource that can be used by the display pipeline (currently only in OpenGL display). Reuse DisplayBitmaps for drawing if possible; it is much more expensive to construct new DisplayBitmaps than it is to reuse existing DisplayBitmaps.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayBitmap.htm)

### Constructors
- `public DisplayBitmap(Bitmap bitmap)` — Constructs a DisplayBitmap from an existing bitmap.
- `public DisplayBitmap(string path, Bitmap bitmap)` — Creates a DisplayBitmap either from a path, or a bitmap. If the path is null, a random tag name will be used. If the bitmap is null, the bitmap will be loaded from the path. If both are null, the object is invalid. if both are valid objects, the bitmap will be used and it will be added to Rhino's bitmap cache with the path supplied. In other words, this is a way to add a bitmap from memory directly into Rhino's memory cache.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmap_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmap_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmap_Finalize.htm)

#### `public void GetBlendModes(out BlendMode source, out BlendMode destination)`

Gets the source and destination blend modes.

**Parameters:**
- `source` (Rhino.Display.BlendMode) — The source blend mode is assigned to this out parameter.
- `destination` (Rhino.Display.BlendMode) — The destination blend mode is assigned to this out parameter.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmap_GetBlendModes.htm)

#### `public static DisplayBitmap Load(string path)`

Load a DisplayBitmap from and image file on disk or from URL. If path starts with http:// or https:// then an attempt is made to load the bitmap from an online resource

**Parameters:**
- `path` (System.String) — A location from which to load the file.

**Returns:** `DisplayBitmap` — The new display bitmap, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmap_Load.htm)

#### `public void SetBlendFunction(BlendMode source, BlendMode destination)`

Sets blending function used to determine how this bitmap is blended with the current frame buffer color. The default setting is SourceAlpha for source and OneMinusSourceAlpha for destination. See OpenGL's glBlendFunc for details. http://www.opengl.org/sdk/docs/man/xhtml/glBlendFunc.xml

**Parameters:**
- `source` (Rhino.Display.BlendMode) — The source blend mode.
- `destination` (Rhino.Display.BlendMode) — The destination blend mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmap_SetBlendFunction.htm)

### Properties
- `Size` (Size, get) — Size of the underlying bitmap image

## DisplayBitmapDrawList (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayBitmapDrawList"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayBitmapDrawList.htm)

### Constructors
- `public DisplayBitmapDrawList()` — Initializes a new instance of the DisplayBitmapDrawList class

### Methods
#### `public void SetPoints(IEnumerable<Point3d> points)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.Display.DisplayBitmapDrawList.SetPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmapDrawList_SetPoints.htm)

#### `public void SetPoints(IEnumerable<Point3d> points, Color blendColor)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.Display.DisplayBitmapDrawList.SetPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},System.Drawing.Color)"]
- `blendColor` (System.Drawing.Color) — [Missing <param name="blendColor"/> documentation for "M:Rhino.Display.DisplayBitmapDrawList.SetPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmapDrawList_SetPoints_2.htm)

#### `public void SetPoints(IEnumerable<Point3d> points, IEnumerable<Color> colors)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.Display.DisplayBitmapDrawList.SetPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},System.Collections.Generic.IEnumerable{System.Drawing.Color})"]
- `colors` (System.Collections.Generic.IEnumerable<Color>) — [Missing <param name="colors"/> documentation for "M:Rhino.Display.DisplayBitmapDrawList.SetPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},System.Collections.Generic.IEnumerable{System.Drawing.Color})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmapDrawList_SetPoints_1.htm)

#### `public int[] Sort(Vector3d cameraDirection)`



**Parameters:**
- `cameraDirection` (Rhino.Geometry.Vector3d) — [Missing <param name="cameraDirection"/> documentation for "M:Rhino.Display.DisplayBitmapDrawList.Sort(Rhino.Geometry.Vector3d)"]

**Returns:** `Int32[]` — [Missing <returns> documentation for "M:Rhino.Display.DisplayBitmapDrawList.Sort(Rhino.Geometry.Vector3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayBitmapDrawList_Sort.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — 
- `MaximumCachedSortLists` (Int32, get/set) — Maximum number of cached sort order index lists stored on this class. Default is 10, but depending on the number of points in this list you may get better performance by setting this value to a certain percentage of the point count.
- `SortAngleTolerance` (Double, get/set) — Angle in radians used to determine if an index list is "parallel enough" to a viewports camera angle. Default is 0.0873 radians (5 degrees)

## DisplayConduit (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayConduit"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayConduit.htm)

### Constructors
- `protected DisplayConduit()` — Initializes a new instance of the DisplayConduit class

### Methods
#### `protected virtual void CalculateBoundingBox(CalculateBoundingBoxEventArgs e)`

Library developers should override this function to increase the bounding box of scene so it includes the geometry that you plan to draw in the "Draw" virtual functions. The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.CalculateBoundingBoxEventArgs) — The event argument contain the current bounding box state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_CalculateBoundingBox.htm)

#### `protected virtual void CalculateBoundingBoxZoomExtents(CalculateBoundingBoxEventArgs e)`

If you want to participate in the Zoom Extents command with your display conduit, then you will need to override ZoomExtentsBoundingBox. Typically you could just call your CalculateBoundingBox override, but you may also want to spend a little more time here and compute a tighter bounding box for your conduit geometry if that is needed. The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.CalculateBoundingBoxEventArgs) — The event argument contain the current bounding box state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_CalculateBoundingBoxZoomExtents.htm)

#### `protected virtual void DrawForeground(DrawEventArgs e)`

Called after all non-highlighted objects have been drawn and PostDrawObjects has been called. Depth writing and testing are turned OFF. If you want to draw with depth writing/testing, see PostDrawObjects. The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — The event argument contains the current viewport and display state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_DrawForeground.htm)

#### `protected virtual void DrawOverlay(DrawEventArgs e)`

If Rhino is in a feedback mode, the draw overlay call allows for temporary geometry to be drawn on top of everything in the scene. This is similar to the dynamic draw routine that occurs with custom get point. The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — The event argument contains the current viewport and display state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_DrawOverlay.htm)

#### `public void GetSelectionFilter(out bool on, out bool checkSubObjects)`

The selection filter will make per-object conduit functions only be called for selected objects (when the filter is turned on)

**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.DisplayConduit.GetSelectionFilter(System.Boolean@,System.Boolean@)"]
- `checkSubObjects` (System.Boolean) — [Missing <param name="checkSubObjects"/> documentation for "M:Rhino.Display.DisplayConduit.GetSelectionFilter(System.Boolean@,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_GetSelectionFilter.htm)

#### `protected virtual void ObjectCulling(CullObjectEventArgs e)`

The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.CullObjectEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.Display.DisplayConduit.ObjectCulling(Rhino.Display.CullObjectEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_ObjectCulling.htm)

#### `protected virtual void OnEnable(bool enable)`

Called when the enabled state changes for this class instance

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.Display.DisplayConduit.OnEnable(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_OnEnable.htm)

#### `protected virtual void PostDrawObjects(DrawEventArgs e)`

Called after all non-highlighted objects have been drawn. Depth writing and testing are still turned on. If you want to draw without depth writing/testing, see DrawForeground. The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — The event argument contains the current viewport and display state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_PostDrawObjects.htm)

#### `protected virtual void PreDrawObject(DrawObjectEventArgs e)`

Called before every object in the scene is drawn.

**Parameters:**
- `e` (Rhino.Display.DrawObjectEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.Display.DisplayConduit.PreDrawObject(Rhino.Display.DrawObjectEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_PreDrawObject.htm)

#### `protected virtual void PreDrawObjects(DrawEventArgs e)`

Called before objects are been drawn. Depth writing and testing are on. The default implementation does nothing.

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — The event argument contain the current viewport and display state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_PreDrawObjects.htm)

#### `public void SetObjectIdFilter(Guid id)`

Set an object Id that this conduit's per-object functions will only be called for

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Display.DisplayConduit.SetObjectIdFilter(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_SetObjectIdFilter_1.htm)

#### `public void SetObjectIdFilter(IEnumerable<Guid> ids)`

Set object Ids that this conduit's per-object functions will only be called for

**Parameters:**
- `ids` (System.Collections.Generic.IEnumerable<Guid>) — [Missing <param name="ids"/> documentation for "M:Rhino.Display.DisplayConduit.SetObjectIdFilter(System.Collections.Generic.IEnumerable{System.Guid})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_SetObjectIdFilter.htm)

#### `public void SetSelectionFilter(bool on, bool checkSubObjects)`

The selection filter will make per-object conduit functions only be called for selected objects (when the filter is turned on)

**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.DisplayConduit.SetSelectionFilter(System.Boolean,System.Boolean)"]
- `checkSubObjects` (System.Boolean) — [Missing <param name="checkSubObjects"/> documentation for "M:Rhino.Display.DisplayConduit.SetSelectionFilter(System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayConduit_SetSelectionFilter.htm)

### Properties
- `Enabled` (Boolean, get/set) — 
- `GeometryFilter` (ObjectType, get/set) — The geometry filter will ensure that your conduit's per-object functions will only be called for objects that are of certain geometry type
- `SpaceFilter` (ActiveSpace, get/set) — If you want this conduit to only work in a specific space (model or page), then set this filter to that specific space. The default is None meaning no filter is applied

## DisplayEngine (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayEngine"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayEngine.htm)

### Constructors
- `protected DisplayEngine()` — Initializes a new instance of the DisplayEngine class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayEngine_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayEngine_Finalize.htm)

## DisplayMaterial (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayMaterial"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayMaterial.htm)

### Constructors
- `public DisplayMaterial()` — Constructs a default material.
- `public DisplayMaterial(Color diffuse)` — Constructs a default material with a specific diffuse color.
- `public DisplayMaterial(DisplayMaterial other)` — Duplicate another material.
- `public DisplayMaterial(Material material)` — Initializes a new instance of the DisplayMaterial class
- `public DisplayMaterial(Color diffuse, double transparency)` — Constructs a default material with a specific diffuse color and transparency.
- `public DisplayMaterial(Color diffuse, Color specular, Color ambient, Color emission, double shine, double transparency)` — Constructs a material with custom properties.

### Methods
#### `public void Dispose()`

Releases all resources used by the DisplayMaterial

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the DisplayMaterial and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_Finalize.htm)

#### `public Texture GetBitmapTexture(bool front)`



**Parameters:**
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.GetBitmapTexture(System.Boolean)"]

**Returns:** `Texture` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.GetBitmapTexture(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_GetBitmapTexture.htm)

#### `public Texture GetBumpTexture(bool front)`

Gets the bump texture for this display material.

**Parameters:**
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.GetBumpTexture(System.Boolean)"]

**Returns:** `Texture` — The texture, or null if no bump texture has been added to this material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_GetBumpTexture.htm)

#### `public Texture GetEnvironmentTexture(bool front)`



**Parameters:**
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.GetEnvironmentTexture(System.Boolean)"]

**Returns:** `Texture` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.GetEnvironmentTexture(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_GetEnvironmentTexture.htm)

#### `public Texture GetTransparencyTexture(bool front)`



**Parameters:**
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.GetTransparencyTexture(System.Boolean)"]

**Returns:** `Texture` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.GetTransparencyTexture(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_GetTransparencyTexture.htm)

#### `public bool SetBitmapTexture(string filename, bool front)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBitmapTexture(System.String,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBitmapTexture(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetBitmapTexture(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetBitmapTexture_1.htm)

#### `public bool SetBitmapTexture(Texture texture, bool front)`



**Parameters:**
- `texture` (Rhino.DocObjects.Texture) — [Missing <param name="texture"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBitmapTexture(Rhino.DocObjects.Texture,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBitmapTexture(Rhino.DocObjects.Texture,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetBitmapTexture(Rhino.DocObjects.Texture,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetBitmapTexture.htm)

#### `public bool SetBumpTexture(string filename, bool front)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBumpTexture(System.String,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBumpTexture(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetBumpTexture(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetBumpTexture_1.htm)

#### `public bool SetBumpTexture(Texture texture, bool front)`



**Parameters:**
- `texture` (Rhino.DocObjects.Texture) — [Missing <param name="texture"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBumpTexture(Rhino.DocObjects.Texture,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetBumpTexture(Rhino.DocObjects.Texture,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetBumpTexture(Rhino.DocObjects.Texture,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetBumpTexture.htm)

#### `public bool SetEnvironmentTexture(string filename, bool front)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.Display.DisplayMaterial.SetEnvironmentTexture(System.String,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetEnvironmentTexture(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetEnvironmentTexture(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetEnvironmentTexture_1.htm)

#### `public bool SetEnvironmentTexture(Texture texture, bool front)`



**Parameters:**
- `texture` (Rhino.DocObjects.Texture) — [Missing <param name="texture"/> documentation for "M:Rhino.Display.DisplayMaterial.SetEnvironmentTexture(Rhino.DocObjects.Texture,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetEnvironmentTexture(Rhino.DocObjects.Texture,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetEnvironmentTexture(Rhino.DocObjects.Texture,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetEnvironmentTexture.htm)

#### `public bool SetTransparencyTexture(string filename, bool front)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.Display.DisplayMaterial.SetTransparencyTexture(System.String,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetTransparencyTexture(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetTransparencyTexture(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetTransparencyTexture_1.htm)

#### `public bool SetTransparencyTexture(Texture texture, bool front)`



**Parameters:**
- `texture` (Rhino.DocObjects.Texture) — [Missing <param name="texture"/> documentation for "M:Rhino.Display.DisplayMaterial.SetTransparencyTexture(Rhino.DocObjects.Texture,System.Boolean)"]
- `front` (System.Boolean) — [Missing <param name="front"/> documentation for "M:Rhino.Display.DisplayMaterial.SetTransparencyTexture(Rhino.DocObjects.Texture,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayMaterial.SetTransparencyTexture(Rhino.DocObjects.Texture,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayMaterial_SetTransparencyTexture.htm)

### Properties
- `BackDiffuse` (Color, get/set) — Gets or sets the Diffuse color of the back side of the Material. The alpha component of the color will be ignored.
- `BackEmission` (Color, get/set) — Gets or sets the Emissive color of the back side of the Material. The alpha component of the color will be ignored.
- `BackShine` (Double, get/set) — Gets or sets the shine factor of the back side of the material (0.0 to 1.0)
- `BackSpecular` (Color, get/set) — Gets or sets the Specular color of the back side of the Material. The alpha component of the color will be ignored.
- `BackTransparency` (Double, get/set) — Gets or sets the transparency of the back side material (0.0 = opaque to 1.0 = transparent)
- `Diffuse` (Color, get/set) — Gets or sets the Diffuse color of the Material. The alpha component of the color will be ignored.
- `Emission` (Color, get/set) — Gets or sets the Emissive color of the Material. The alpha component of the color will be ignored.
- `IsTwoSided` (Boolean, get/set) — 
- `Shine` (Double, get/set) — Gets or sets the shine factor of the material (0.0 to 1.0)
- `Specular` (Color, get/set) — Gets or sets the Specular color of the Material. The alpha component of the color will be ignored.
- `Transparency` (Double, get/set) — Gets or sets the transparency of the material (0.0 = opaque to 1.0 = transparent)

## DisplayModeChangedEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayModeChangedEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayModeChangedEventArgs.htm)

### Properties
- `ChangedDisplayModeId` (Guid, get) — 
- `OldDisplayModeId` (Guid, get) — 
- `RhinoDoc` (RhinoDoc, get) — 
- `Viewport` (RhinoViewport, get) — 

## DisplayModeDescription (class)

Description of a how Rhino will display in a viewport. These are the modes that are listed under "Advanced display" in the options dialog.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayModeDescription.htm)

### Methods
#### `public static Guid AddDisplayMode(DisplayModeDescription displayMode)`



**Parameters:**
- `displayMode` (Rhino.Display.DisplayModeDescription) — [Missing <param name="displayMode"/> documentation for "M:Rhino.Display.DisplayModeDescription.AddDisplayMode(Rhino.Display.DisplayModeDescription)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Display.DisplayModeDescription.AddDisplayMode(Rhino.Display.DisplayModeDescription)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_AddDisplayMode.htm)

#### `public static Guid AddDisplayMode(string name)`

Adds a new display mode.

**Parameters:**
- `name` (System.String) — The name of the new display mode.

**Returns:** `Guid` — The id of the new display mode if successful. Guid.Empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_AddDisplayMode_1.htm)

#### `public static Guid CopyDisplayMode(Guid id, string name)`

Copies an existing display mode.

**Parameters:**
- `id` (System.Guid) — The id of the existing display mode to copy.
- `name` (System.String) — The name of the new display mode.

**Returns:** `Guid` — The id of the new display mode if successful. Guid.Empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_CopyDisplayMode.htm)

#### `[ObsoleteAttribute("Use DisplayModeDescription.DeleteDisplayMode")] public static bool DeleteDiplayMode(Guid id)`

Deletes an existing display mode.

**Parameters:**
- `id` (System.Guid) — The id of the existing display mode to delete.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_DeleteDiplayMode.htm)

#### `public static bool DeleteDisplayMode(Guid id)`

Deletes an existing display mode.

**Parameters:**
- `id` (System.Guid) — The id of the existing display mode to delete.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_DeleteDisplayMode.htm)

#### `public void Dispose()`

Releases all resources used by the DisplayModeDescription

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the DisplayModeDescription and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_Dispose_1.htm)

#### `public static bool ExportToFile(DisplayModeDescription displayMode, string filename)`

Exports a DisplayModeDescription to a Windows-style .ini file.

**Parameters:**
- `displayMode` (Rhino.Display.DisplayModeDescription) — The DisplayModeDescription to export.
- `filename` (System.String) — The name of the file to create.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayModeDescription.ExportToFile(Rhino.Display.DisplayModeDescription,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_ExportToFile.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_Finalize.htm)

#### `public static DisplayModeDescription FindByName(string englishName)`



**Parameters:**
- `englishName` (System.String) — [Missing <param name="englishName"/> documentation for "M:Rhino.Display.DisplayModeDescription.FindByName(System.String)"]

**Returns:** `DisplayModeDescription` — [Missing <returns> documentation for "M:Rhino.Display.DisplayModeDescription.FindByName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_FindByName.htm)

#### `public static DisplayModeDescription GetDisplayMode(Guid id)`



**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Display.DisplayModeDescription.GetDisplayMode(System.Guid)"]

**Returns:** `DisplayModeDescription` — [Missing <returns> documentation for "M:Rhino.Display.DisplayModeDescription.GetDisplayMode(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_GetDisplayMode.htm)

#### `public static DisplayModeDescription[] GetDisplayModes()`

Gets all display mode descriptions that Rhino currently knows about.

**Returns:** `DisplayModeDescription[]` — Copies of all of the display mode descriptions. If you want to modify these descriptions, you must call UpdateDisplayMode or AddDisplayMode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_GetDisplayModes.htm)

#### `public static Guid ImportFromFile(string filename)`

Imports a DisplayModeDescription from a Windows-style .ini file.

**Parameters:**
- `filename` (System.String) — The name of the file to import.

**Returns:** `Guid` — The id of the DisplayModeDescription if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_ImportFromFile.htm)

#### `public static bool UpdateDisplayMode(DisplayModeDescription displayMode)`



**Parameters:**
- `displayMode` (Rhino.Display.DisplayModeDescription) — [Missing <param name="displayMode"/> documentation for "M:Rhino.Display.DisplayModeDescription.UpdateDisplayMode(Rhino.Display.DisplayModeDescription)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayModeDescription.UpdateDisplayMode(Rhino.Display.DisplayModeDescription)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayModeDescription_UpdateDisplayMode.htm)

### Properties
- `AllowObjectAssignment` (Boolean, get/set) — 
- `AmbientOcclusionId` (Guid, get) — 
- `ArtisticId` (Guid, get) — 
- `DisplayAttributes` (DisplayPipelineAttributes, get) — 
- `EnglishName` (String, get/set) — 
- `GhostedId` (Guid, get) — 
- `Id` (Guid, get) — 
- `InMenu` (Boolean, get/set) — 
- `LocalName` (String, get) — 
- `PenId` (Guid, get) — 
- `PipelineLocked` (Boolean, get/set) — 
- `RaytracedId` (Guid, get) — 
- `RenderedId` (Guid, get) — 
- `RenderedShadowsId` (Guid, get) — 
- `ShadedId` (Guid, get) — 
- `ShadedPipelineRequired` (Boolean, get/set) — 
- `SupportsShadeCommand` (Boolean, get/set) — 
- `SupportsShading` (Boolean, get/set) — 
- `TechId` (Guid, get) — 
- `WireframeId` (Guid, get) — 
- `WireframePipelineRequired` (Boolean, get/set) — 
- `XRayId` (Guid, get) — 

## DisplayPipeline (class)

The display pipeline calls events during specific phases of drawing During the drawing of a single frame the events are called in the following order. [Begin Drawing of a Frame] CalculateBoundingBoxCalculateClippingPanesSetupFrustumSetupLightingInitializeFrameBufferDrawBackgroundIf this is a layout and detail objects exist the channels are called in the same order for each detail object (drawn as a nested viewport)PreDrawObjectsFor Each Visible Non Highlighted ObjectPostDrawObjects - depth writing/testing onDrawForeGround - depth writing/testing offFor Each Visible Highlighted ObjectPostProcessFrameBuffer (If a delegate exists that requires this)DrawOverlay (if Rhino is in a feedback mode) [End of Drawing of a Frame] NOTE: There may be multiple DrawObject calls for a single object. An example of when this could happen would be with a shaded sphere. The shaded mesh is first drawn and these channels would be processed; then at a later time the isocurves for the sphere would be drawn.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipeline.htm)

### Methods
#### `public int AddClippingPlane(Point3d point, Vector3d normal)`

Add a clipping plane to be used during the drawing of this frame

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — point on the plane
- `normal` (Rhino.Geometry.Vector3d) — vector perpendicular to the plane

**Returns:** `Int32` — index for the added clipping plane

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_AddClippingPlane.htm)

#### `public static uint AvailableOpenGLVersion(out bool coreProfile)`

If Rhino is using OpenGL for display, this function will return major.minor version of OpenGL available for this instance of Rhino

**Parameters:**
- `coreProfile` (System.Boolean) — If true, OpenGL is being used in "core profile" mode

**Returns:** `UInt32` — major version * 10 + minor version For example, OpenGL 4.5 returns 45

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_AvailableOpenGLVersion.htm)

#### `public DisplayPipeline Clone(RhinoViewport viewport)`

Clones the pipeline. Creates an identical copy of "this" pipeline. Copies all conduits from "this" pipeline to the new pipeline.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.Display.DisplayPipeline.Clone(Rhino.Display.RhinoViewport)"]

**Returns:** `DisplayPipeline` — The newly cloned pipeline if successful, null otherwise. or failed to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Clone.htm)

#### `public bool Close()`

Closes the pipeline.

**Returns:** `Boolean` — True if the pipeline was closed, false if it was already closed or failed to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Close.htm)

#### `public static bool CullControlPolygon()`

Returns a value indicating if only points on the side of the surface that face the camera are displayed.

**Returns:** `Boolean` — true if back faces of surface and mesh control polygons are culled. This value is determined by the _CullControlPolygon command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_CullControlPolygon.htm)

#### `public void Draw2dLine(Point from, Point to, Color color, float thickness)`



**Parameters:**
- `from` (System.Drawing.Point) — [Missing <param name="from"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.Point,System.Drawing.Point,System.Drawing.Color,System.Single)"]
- `to` (System.Drawing.Point) — [Missing <param name="to"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.Point,System.Drawing.Point,System.Drawing.Color,System.Single)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.Point,System.Drawing.Point,System.Drawing.Color,System.Single)"]
- `thickness` (System.Single) — [Missing <param name="thickness"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.Point,System.Drawing.Point,System.Drawing.Color,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dLine.htm)

#### `public void Draw2dLine(PointF from, PointF to, Color color, float thickness)`



**Parameters:**
- `from` (System.Drawing.PointF) — [Missing <param name="from"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `to` (System.Drawing.PointF) — [Missing <param name="to"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `thickness` (System.Single) — [Missing <param name="thickness"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dLine(System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dLine_1.htm)

#### `public void Draw2dRectangle(Rectangle rectangle, Color strokeColor, int thickness, Color fillColor)`



**Parameters:**
- `rectangle` (System.Drawing.Rectangle) — [Missing <param name="rectangle"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dRectangle(System.Drawing.Rectangle,System.Drawing.Color,System.Int32,System.Drawing.Color)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dRectangle(System.Drawing.Rectangle,System.Drawing.Color,System.Int32,System.Drawing.Color)"]
- `thickness` (System.Int32) — [Missing <param name="thickness"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dRectangle(System.Drawing.Rectangle,System.Drawing.Color,System.Int32,System.Drawing.Color)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw2dRectangle(System.Drawing.Rectangle,System.Drawing.Color,System.Int32,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dRectangle.htm)

#### `public void Draw2dText(string text, Color color, Point2d screenCoordinate, bool middleJustified)`

Draws 2D text on the viewport.

**Parameters:**
- `text` (System.String) — the string to draw.
- `color` (System.Drawing.Color) — text color.
- `screenCoordinate` (Rhino.Geometry.Point2d) — definition point in screen coordinates (0,0 is top-left corner)
- `middleJustified` (System.Boolean) — if true text is centered around the definition point, otherwise it is lower-left justified.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dText.htm)

#### `public void Draw2dText(string text, Color color, Point2d screenCoordinate, bool middleJustified, int height)`

Draws 2D text on the viewport.

**Parameters:**
- `text` (System.String) — the string to draw.
- `color` (System.Drawing.Color) — text color.
- `screenCoordinate` (Rhino.Geometry.Point2d) — definition point in screen coordinates (0,0 is top-left corner)
- `middleJustified` (System.Boolean) — if true text is centered around the definition point, otherwise it is lower-left justified.
- `height` (System.Int32) — height in pixels (good default is 12)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dText_1.htm)

#### `public void Draw2dText(string text, Color color, Point2d screenCoordinate, bool middleJustified, int height, string fontface)`

Draws 2D text on the viewport.

**Parameters:**
- `text` (System.String) — the string to draw.
- `color` (System.Drawing.Color) — text color.
- `screenCoordinate` (Rhino.Geometry.Point2d) — definition point in screen coordinates (0,0 is top-left corner)
- `middleJustified` (System.Boolean) — if true text is centered around the definition point, otherwise it is lower-left justified.
- `height` (System.Int32) — height in pixels (good default is 12)
- `fontface` (System.String) — font name (good default is "Arial")

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dText_2.htm)

#### `public void Draw2dText(string text, Color color, Point3d worldCoordinate, bool middleJustified)`

Draws 2D text on the viewport.

**Parameters:**
- `text` (System.String) — the string to draw.
- `color` (System.Drawing.Color) — text color.
- `worldCoordinate` (Rhino.Geometry.Point3d) — definition point in world coordinates.
- `middleJustified` (System.Boolean) — if true text is centered around the definition point, otherwise it is lower-left justified.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dText_3.htm)

#### `public void Draw2dText(string text, Color color, Point3d worldCoordinate, bool middleJustified, int height)`

Draws 2D text on the viewport.

**Parameters:**
- `text` (System.String) — the string to draw.
- `color` (System.Drawing.Color) — text color.
- `worldCoordinate` (Rhino.Geometry.Point3d) — definition point in world coordinates.
- `middleJustified` (System.Boolean) — if true text is centered around the definition point, otherwise it is lower-left justified.
- `height` (System.Int32) — height in pixels (good default is 12)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dText_4.htm)

#### `public void Draw2dText(string text, Color color, Point3d worldCoordinate, bool middleJustified, int height, string fontface)`

Draws 2D text on the viewport.

**Parameters:**
- `text` (System.String) — The string to draw.
- `color` (System.Drawing.Color) — Text color.
- `worldCoordinate` (Rhino.Geometry.Point3d) — Definition point in world coordinates.
- `middleJustified` (System.Boolean) — If true text is centered around the definition point, otherwise it is lower-left justified.
- `height` (System.Int32) — Height in pixels (good default is 12).
- `fontface` (System.String) — Font name (good default is "Arial").

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw2dText_5.htm)

#### `public void Draw3dText(string text, Color color, Plane textPlane, double height, string fontface)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String)"]
- `textPlane` (Rhino.Geometry.Plane) — [Missing <param name="textPlane"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String)"]
- `fontface` (System.String) — [Missing <param name="fontface"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw3dText_3.htm)

#### `public void Draw3dText(string text, Color color, Plane textPlane, double height, string fontface, bool bold, bool italic)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]
- `textPlane` (Rhino.Geometry.Plane) — [Missing <param name="textPlane"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]
- `fontface` (System.String) — [Missing <param name="fontface"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw3dText_4.htm)

#### `public void Draw3dText(string text, Color color, Plane textPlane, double height, string fontface, bool bold, bool italic, TextHorizontalAlignment horizontalAlignment, TextVerticalAlignment verticalAlignment)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `textPlane` (Rhino.Geometry.Plane) — [Missing <param name="textPlane"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `fontface` (System.String) — [Missing <param name="fontface"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `horizontalAlignment` (Rhino.DocObjects.TextHorizontalAlignment) — [Missing <param name="horizontalAlignment"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `verticalAlignment` (Rhino.DocObjects.TextVerticalAlignment) — [Missing <param name="verticalAlignment"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(System.String,System.Drawing.Color,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw3dText_5.htm)

#### `public void Draw3dText(Text3d text, Color color)`



**Parameters:**
- `text` (Rhino.Display.Text3d) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(Rhino.Display.Text3d,System.Drawing.Color)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.Draw3dText(Rhino.Display.Text3d,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw3dText.htm)

#### `public void Draw3dText(Text3d text, Color color, Plane textPlane)`

Draws 3d text with a different plane than what is defined in the Text3d class.

**Parameters:**
- `text` (Rhino.Display.Text3d) — The string to draw.
- `color` (System.Drawing.Color) — Text color.
- `textPlane` (Rhino.Geometry.Plane) — The plane for the text object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw3dText_1.htm)

#### `public void Draw3dText(Text3d text, Color color, Point3d textPlaneOrigin)`

Draws 3d text using the Text3d plane with an adjusted origin.

**Parameters:**
- `text` (Rhino.Display.Text3d) — The string to draw.
- `color` (System.Drawing.Color) — Text color.
- `textPlaneOrigin` (Rhino.Geometry.Point3d) — The origin of the plane to draw.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Draw3dText_2.htm)

#### `public void DrawActivePoint(Point3d point)`

Draws a point in style used during "GetPoint" operations

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Location of the point in world coordinates

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawActivePoint.htm)

#### `public void DrawAnnotation(AnnotationBase annotation, Color color)`



**Parameters:**
- `annotation` (Rhino.Geometry.AnnotationBase) — [Missing <param name="annotation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawAnnotation(Rhino.Geometry.AnnotationBase,System.Drawing.Color)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawAnnotation(Rhino.Geometry.AnnotationBase,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawAnnotation.htm)

#### `public void DrawAnnotationArrowhead(Arrowhead arrowhead, Transform xform, Color color)`



**Parameters:**
- `arrowhead` (Rhino.Geometry.Arrowhead) — [Missing <param name="arrowhead"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawAnnotationArrowhead(Rhino.Geometry.Arrowhead,Rhino.Geometry.Transform,System.Drawing.Color)"]
- `xform` (Rhino.Geometry.Transform) — [Missing <param name="xform"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawAnnotationArrowhead(Rhino.Geometry.Arrowhead,Rhino.Geometry.Transform,System.Drawing.Color)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawAnnotationArrowhead(Rhino.Geometry.Arrowhead,Rhino.Geometry.Transform,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawAnnotationArrowhead.htm)

#### `public void DrawArc(Arc arc, Color color)`

Draw a single arc object.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — Arc to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArc.htm)

#### `public void DrawArc(Arc arc, Color color, int thickness)`

Draw a single arc object.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — Arc to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of arc.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArc_1.htm)

#### `public void DrawArrow(Line line, Color color)`

Draws a single arrow object. An arrow consists of a Shaft and an Arrow head at the end of the shaft.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Arrow shaft.
- `color` (System.Drawing.Color) — Color of arrow.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArrow.htm)

#### `public void DrawArrow(Line line, Color color, double screenSize, double relativeSize)`

Draws a single arrow object. An arrow consists of a Shaft and an Arrow head at the end of the shaft.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Arrow shaft.
- `color` (System.Drawing.Color) — Color of arrow.
- `screenSize` (System.Double) — If screenSize != 0.0 then the size (in screen pixels) of the arrow head will be equal to screenSize.
- `relativeSize` (System.Double) — If relativeSize != 0.0 and screen size == 0.0 the size of the arrow head will be proportional to the arrow shaft length.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArrow_1.htm)

#### `public void DrawArrowHead(Point3d tip, Vector3d direction, Color color, double screenSize, double worldSize)`

Draws a single arrow head.

**Parameters:**
- `tip` (Rhino.Geometry.Point3d) — Point of arrow head tip.
- `direction` (Rhino.Geometry.Vector3d) — Direction in which arrow head is pointing.
- `color` (System.Drawing.Color) — Color of arrow head.
- `screenSize` (System.Double) — If screenSize != 0.0, then the size (in screen pixels) of the arrow head will be equal to the screenSize.
- `worldSize` (System.Double) — If worldSize != 0.0 and screen size == 0.0 the size of the arrow head will be equal to the number of units in worldSize.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArrowHead.htm)

#### `public void DrawArrows(IEnumerable<Line> lines, Color color)`

Draws a collection of arrow objects. An arrow consists of a Shaft and an Arrow head at the end of the shaft.

**Parameters:**
- `lines` (System.Collections.Generic.IEnumerable<Line>) — Arrow shafts.
- `color` (System.Drawing.Color) — Color of arrows.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArrows_1.htm)

#### `public void DrawArrows(Line[] lines, Color color)`

Draws a collection of arrow objects. An arrow consists of a Shaft and an Arrow head at the end of the shaft.

**Parameters:**
- `lines` (Rhino.Geometry.Line[]) — Arrow shafts.
- `color` (System.Drawing.Color) — Color of arrows.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawArrows.htm)

#### `public void DrawBitmap(DisplayBitmap bitmap, int left, int top)`

Draws a bitmap in screen coordinates

**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — bitmap to draw
- `left` (System.Int32) — where top/left corner of bitmap should appear in screen coordinates
- `top` (System.Int32) — where top/left corner of bitmap should appear in screen coordinates

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBitmap.htm)

#### `public void DrawBox(BoundingBox box, Color color)`

Draws the edges of a BoundingBox.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw in.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBox.htm)

#### `public void DrawBox(BoundingBox box, Color color, int thickness)`

Draws the edges of a BoundingBox.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw in.
- `thickness` (System.Int32) — Thickness (in pixels) of box edges.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBox_1.htm)

#### `public void DrawBox(Box box, Color color)`

Draws the edges of a Box object.

**Parameters:**
- `box` (Rhino.Geometry.Box) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw in.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBox_2.htm)

#### `public void DrawBox(Box box, Color color, int thickness)`

Draws the edges of a Box object.

**Parameters:**
- `box` (Rhino.Geometry.Box) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw in.
- `thickness` (System.Int32) — Thickness (in pixels) of box edges.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBox_3.htm)

#### `public void DrawBoxCorners(BoundingBox box, Color color)`

Draws corner widgets of a world aligned bounding box. Widget size will be 5% of the Box diagonal.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBoxCorners.htm)

#### `public void DrawBoxCorners(BoundingBox box, Color color, double size)`

Draws corner widgets of a world aligned bounding box.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `size` (System.Double) — Size (in model units) of the corner widgets.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBoxCorners_1.htm)

#### `public void DrawBoxCorners(BoundingBox box, Color color, double size, int thickness)`

Draws corner widgets of a world aligned bounding box.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — Box to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `size` (System.Double) — Size (in model units) of the corner widgets.
- `thickness` (System.Int32) — Thickness (in pixels) of the corner widgets.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBoxCorners_2.htm)

#### `public void DrawBrepShaded(Brep brep, DisplayMaterial material)`

Draws a shaded mesh representation of a brep.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — Brep to draw.
- `material` (Rhino.Display.DisplayMaterial) — Material to draw faces with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBrepShaded.htm)

#### `public void DrawBrepWires(Brep brep, Color color)`

Draws all the wireframe curves of a brep object.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — Brep to draw.
- `color` (System.Drawing.Color) — Color of Wireframe curves.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBrepWires.htm)

#### `public void DrawBrepWires(Brep brep, Color color, int wireDensity)`

Draws all the wireframe curves of a brep object.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — Brep to draw.
- `color` (System.Drawing.Color) — Color of Wireframe curves.
- `wireDensity` (System.Int32) — "Density" of wireframe curves. -1 = no internal wires. 0 = default internal wires.>0 = custom high density.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawBrepWires_1.htm)

#### `public void DrawCircle(Circle circle, Color color)`

Draw a single circle object.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — Circle to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCircle.htm)

#### `public void DrawCircle(Circle circle, Color color, int thickness)`

Draw a single circle object.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — Circle to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of circle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCircle_1.htm)

#### `public void DrawCone(Cone cone, Color color)`

Draw a wireframe cone.

**Parameters:**
- `cone` (Rhino.Geometry.Cone) — Cone to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCone.htm)

#### `public void DrawCone(Cone cone, Color color, int thickness)`

Draw a wireframe cone.

**Parameters:**
- `cone` (Rhino.Geometry.Cone) — Cone to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of Cone wires.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCone_1.htm)

#### `public void DrawConstructionPlane(ConstructionPlane constructionPlane)`



**Parameters:**
- `constructionPlane` (Rhino.DocObjects.ConstructionPlane) — [Missing <param name="constructionPlane"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawConstructionPlane(Rhino.DocObjects.ConstructionPlane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawConstructionPlane.htm)

#### `public void DrawCurvatureGraph(Curve curve, Color color)`

Draw a typical Rhino Curvature Graph.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Base curve for curvature graph.
- `color` (System.Drawing.Color) — Color of curvature graph.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCurvatureGraph.htm)

#### `public void DrawCurvatureGraph(Curve curve, Color color, int hairScale)`

Draw a typical Rhino Curvature Graph.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Base curve for curvature graph.
- `color` (System.Drawing.Color) — Color of curvature graph.
- `hairScale` (System.Int32) — 100 = true length, > 100 magnified, < 100 shortened.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCurvatureGraph_1.htm)

#### `public void DrawCurvatureGraph(Curve curve, Color color, int hairScale, int hairDensity, int sampleDensity)`

Draw a typical Rhino Curvature Graph.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Base curve for curvature graph.
- `color` (System.Drawing.Color) — Color of curvature graph.
- `hairScale` (System.Int32) — 100 = true length, > 100 magnified, < 100 shortened.
- `hairDensity` (System.Int32) — >= 0 larger numbers = more hairs (good default is 1).
- `sampleDensity` (System.Int32) — Between 1 and 10. Higher numbers draw smoother outer curves. (good default is 2).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCurvatureGraph_2.htm)

#### `public void DrawCurve(Curve curve, Color color)`

Draw a single Curve object.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCurve.htm)

#### `public void DrawCurve(Curve curve, Color color, int thickness)`

Draw a single Curve object.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of curve.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCurve_1.htm)

#### `public void DrawCylinder(Cylinder cylinder, Color color)`

Draw a wireframe cylinder.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — Cylinder to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCylinder.htm)

#### `public void DrawCylinder(Cylinder cylinder, Color color, int thickness)`

Draw a wireframe cylinder.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — Cylinder to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of cylinder wires.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawCylinder_1.htm)

#### `public void DrawDirectionArrow(Point3d location, Vector3d direction, Color color)`



**Parameters:**
- `location` (Rhino.Geometry.Point3d) — [Missing <param name="location"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDirectionArrow(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,System.Drawing.Color)"]
- `direction` (Rhino.Geometry.Vector3d) — [Missing <param name="direction"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDirectionArrow(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,System.Drawing.Color)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDirectionArrow(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDirectionArrow.htm)

#### `public void DrawDot(Point3d worldPosition, string text)`

Draws a text dot in world coordinates.

**Parameters:**
- `worldPosition` (Rhino.Geometry.Point3d) — Location of dot in world coordinates.
- `text` (System.String) — Text content of dot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDot.htm)

#### `public void DrawDot(Point3d worldPosition, string text, Color dotColor, Color textColor)`

Draw a text dot in world coordinates.

**Parameters:**
- `worldPosition` (Rhino.Geometry.Point3d) — Location of dot in world coordinates.
- `text` (System.String) — Text content of dot.
- `dotColor` (System.Drawing.Color) — Dot background color.
- `textColor` (System.Drawing.Color) — Dot foreground color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDot_1.htm)

#### `public void DrawDot(float screenX, float screenY, string text)`

Draws a text dot in screen coordinates.

**Parameters:**
- `screenX` (System.Single) — X coordinate (in pixels) of dot center.
- `screenY` (System.Single) — Y coordinate (in pixels) of dot center.
- `text` (System.String) — Text content of dot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDot_3.htm)

#### `public void DrawDot(float screenX, float screenY, string text, Color dotColor, Color textColor)`

Draws a text dot in screen coordinates.

**Parameters:**
- `screenX` (System.Single) — X coordinate (in pixels) of dot center.
- `screenY` (System.Single) — Y coordinate (in pixels) of dot center.
- `text` (System.String) — Text content of dot.
- `dotColor` (System.Drawing.Color) — Dot background color.
- `textColor` (System.Drawing.Color) — Dot foreground color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDot_4.htm)

#### `public void DrawDot(TextDot dot, Color fillColor, Color textColor, Color borderColor)`

Draw a text dot as defined by the text dot class

**Parameters:**
- `dot` (Rhino.Geometry.TextDot) — [Missing <param name="dot"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDot(Rhino.Geometry.TextDot,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDot(Rhino.Geometry.TextDot,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `textColor` (System.Drawing.Color) — [Missing <param name="textColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDot(Rhino.Geometry.TextDot,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `borderColor` (System.Drawing.Color) — [Missing <param name="borderColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawDot(Rhino.Geometry.TextDot,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDot_2.htm)

#### `public void DrawDottedLine(Line line, Color color)`

Draws a single dotted line.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to draw.
- `color` (System.Drawing.Color) — Color of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDottedLine.htm)

#### `public void DrawDottedLine(Point3d from, Point3d to, Color color)`

Draws a single dotted line.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — Line start point.
- `to` (Rhino.Geometry.Point3d) — Line end point.
- `color` (System.Drawing.Color) — Color of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDottedLine_1.htm)

#### `public void DrawDottedPolyline(IEnumerable<Point3d> points, Color color, bool close)`

Draws a set of connected lines (polyline) in a dotted pattern (0x00001111).

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — End points of each line segment.
- `color` (System.Drawing.Color) — Color of polyline.
- `close` (System.Boolean) — Draw a line between the first and last points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawDottedPolyline.htm)

#### `public void DrawExtrusionWires(Extrusion extrusion, Color color)`

Draws all the wireframe curves of an extrusion object.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — Extrusion to draw.
- `color` (System.Drawing.Color) — Color of Wireframe curves.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawExtrusionWires.htm)

#### `public void DrawExtrusionWires(Extrusion extrusion, Color color, int wireDensity)`

Draws all the wireframe curves of an extrusion object.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — Extrusion to draw.
- `color` (System.Drawing.Color) — Color of Wireframe curves.
- `wireDensity` (System.Int32) — "Density" of wireframe curves. -1 = no internal wires. 0 = default internal wires.>0 = custom high density.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawExtrusionWires_1.htm)

#### `public void DrawGradientHatch(Hatch hatch, Color color1, Color color2, Point3d point1, Point3d point2, bool linearGradient, float boundaryThickness, Color boundaryColor)`

Draw a two point gradient filled hatch

**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `color1` (System.Drawing.Color) — [Missing <param name="color1"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `color2` (System.Drawing.Color) — [Missing <param name="color2"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `point1` (Rhino.Geometry.Point3d) — [Missing <param name="point1"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `point2` (Rhino.Geometry.Point3d) — [Missing <param name="point2"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `linearGradient` (System.Boolean) — [Missing <param name="linearGradient"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `boundaryThickness` (System.Single) — [Missing <param name="boundaryThickness"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]
- `boundaryColor` (System.Drawing.Color) — [Missing <param name="boundaryColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color,Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawGradientHatch_1.htm)

#### `public void DrawGradientHatch(Hatch hatch, IEnumerable<ColorStop> stops, Point3d point1, Point3d point2, bool linearGradient, float repeat, float boundaryThickness, Color boundaryColor)`



**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `stops` (System.Collections.Generic.IEnumerable<ColorStop>) — [Missing <param name="stops"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `point1` (Rhino.Geometry.Point3d) — [Missing <param name="point1"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `point2` (Rhino.Geometry.Point3d) — [Missing <param name="point2"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `linearGradient` (System.Boolean) — [Missing <param name="linearGradient"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `repeat` (System.Single) — [Missing <param name="repeat"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `boundaryThickness` (System.Single) — [Missing <param name="boundaryThickness"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]
- `boundaryColor` (System.Drawing.Color) — [Missing <param name="boundaryColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientHatch(Rhino.Geometry.Hatch,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single,System.Single,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawGradientHatch.htm)

#### `public void DrawGradientLines(IEnumerable<Line> lines, float strokeWidth, IEnumerable<ColorStop> stops, Point3d point1, Point3d point2, bool linearGradient, float repeat)`



**Parameters:**
- `lines` (System.Collections.Generic.IEnumerable<Line>) — [Missing <param name="lines"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `stops` (System.Collections.Generic.IEnumerable<ColorStop>) — [Missing <param name="stops"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `point1` (Rhino.Geometry.Point3d) — [Missing <param name="point1"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `point2` (Rhino.Geometry.Point3d) — [Missing <param name="point2"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `linearGradient` (System.Boolean) — [Missing <param name="linearGradient"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `repeat` (System.Single) — [Missing <param name="repeat"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientLines(System.Collections.Generic.IEnumerable{Rhino.Geometry.Line},System.Single,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawGradientLines.htm)

#### `public void DrawGradientMesh(Mesh mesh, IEnumerable<ColorStop> stops, Point3d point1, Point3d point2, bool linearGradient, float repeat)`



**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientMesh(Rhino.Geometry.Mesh,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `stops` (System.Collections.Generic.IEnumerable<ColorStop>) — [Missing <param name="stops"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientMesh(Rhino.Geometry.Mesh,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `point1` (Rhino.Geometry.Point3d) — [Missing <param name="point1"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientMesh(Rhino.Geometry.Mesh,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `point2` (Rhino.Geometry.Point3d) — [Missing <param name="point2"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientMesh(Rhino.Geometry.Mesh,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `linearGradient` (System.Boolean) — [Missing <param name="linearGradient"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientMesh(Rhino.Geometry.Mesh,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]
- `repeat` (System.Single) — [Missing <param name="repeat"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawGradientMesh(Rhino.Geometry.Mesh,System.Collections.Generic.IEnumerable{Rhino.Display.ColorStop},Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,System.Boolean,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawGradientMesh.htm)

#### `public void DrawHatch(Hatch hatch, Color hatchColor, Color boundaryColor)`



**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color)"]
- `hatchColor` (System.Drawing.Color) — [Missing <param name="hatchColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color)"]
- `boundaryColor` (System.Drawing.Color) — [Missing <param name="boundaryColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawHatch(Rhino.Geometry.Hatch,System.Drawing.Color,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawHatch.htm)

#### `public void DrawInstanceDefinition(InstanceDefinition instanceDefinition)`

Draws an InstanceDefinition.

**Parameters:**
- `instanceDefinition` (Rhino.DocObjects.InstanceDefinition) — The instance definition.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawInstanceDefinition.htm)

#### `public void DrawInstanceDefinition(InstanceDefinition instanceDefinition, Transform xform)`

Draws an InstanceDefinition.

**Parameters:**
- `instanceDefinition` (Rhino.DocObjects.InstanceDefinition) — The instance definition.
- `xform` (Rhino.Geometry.Transform) — The transformation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawInstanceDefinition_1.htm)

#### `public void DrawLine(Line line, Color color)`

Draws a single line object.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to draw.
- `color` (System.Drawing.Color) — Color to draw line in.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLine.htm)

#### `public void DrawLine(Line line, Color color, int thickness)`

Draws a single line object.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to draw.
- `color` (System.Drawing.Color) — Color to draw line in.
- `thickness` (System.Int32) — Thickness (in pixels) of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLine_1.htm)

#### `public void DrawLine(Point3d from, Point3d to, Color color)`

Draws a single line object.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — Line from point.
- `to` (Rhino.Geometry.Point3d) — Line to point.
- `color` (System.Drawing.Color) — Color to draw line in.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLine_2.htm)

#### `public void DrawLine(Point3d from, Point3d to, Color color, int thickness)`

Draws a single line object.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — Line from point.
- `to` (Rhino.Geometry.Point3d) — Line to point.
- `color` (System.Drawing.Color) — Color to draw line in.
- `thickness` (System.Int32) — Thickness (in pixels) of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLine_3.htm)

#### `public void DrawLineArrow(Line line, Color color, int thickness, double size)`

Draws an arrow made up of three line segments.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Base line for arrow.
- `color` (System.Drawing.Color) — Color of arrow.
- `thickness` (System.Int32) — Thickness (in pixels) of the arrow line segments.
- `size` (System.Double) — Size (in world units) of the arrow tip lines.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLineArrow.htm)

#### `public void DrawLineNoClip(Point3d from, Point3d to, Color color, int thickness)`

Draws a single line object .This version of line drawing will draw the segments of the line that extend beyond the near and far planes of the view frustum with depths on those planes

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — Line from point.
- `to` (Rhino.Geometry.Point3d) — Line to point.
- `color` (System.Drawing.Color) — Color to draw line in.
- `thickness` (System.Int32) — Thickness (in pixels) of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLineNoClip.htm)

#### `public void DrawLines(IEnumerable<Line> lines, Color color)`

Draws a set of lines with a given color and thickness. If you want the fastest possible set of lines to be drawn, pass a Line[] for lines.

**Parameters:**
- `lines` (System.Collections.Generic.IEnumerable<Line>) — Lines to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLines.htm)

#### `public void DrawLines(IEnumerable<Line> lines, Color color, int thickness)`

Draws a set of lines with a given color and thickness. If you want the fastest possible set of lines to be drawn, pass a Line[] for lines.

**Parameters:**
- `lines` (System.Collections.Generic.IEnumerable<Line>) — Lines to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of lines.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawLines_1.htm)

#### `public void DrawMarker(Point3d tip, Vector3d direction, Color color)`

Draws an arrow marker as a view-aligned widget.

**Parameters:**
- `tip` (Rhino.Geometry.Point3d) — Location of arrow tip point.
- `direction` (Rhino.Geometry.Vector3d) — Direction of arrow.
- `color` (System.Drawing.Color) — Color of arrow widget.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMarker.htm)

#### `public void DrawMarker(Point3d tip, Vector3d direction, Color color, int thickness)`

Draws an arrow marker as a view-aligned widget.

**Parameters:**
- `tip` (Rhino.Geometry.Point3d) — Location of arrow tip point.
- `direction` (Rhino.Geometry.Vector3d) — Direction of arrow.
- `color` (System.Drawing.Color) — Color of arrow widget.
- `thickness` (System.Int32) — Thickness of arrow widget lines.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMarker_1.htm)

#### `public void DrawMarker(Point3d tip, Vector3d direction, Color color, int thickness, double size)`

Draws an arrow marker as a view-aligned widget.

**Parameters:**
- `tip` (Rhino.Geometry.Point3d) — Location of arrow tip point.
- `direction` (Rhino.Geometry.Vector3d) — Direction of arrow.
- `color` (System.Drawing.Color) — Color of arrow widget.
- `thickness` (System.Int32) — Thickness of arrow widget lines.
- `size` (System.Double) — Size (in pixels) of the arrow shaft.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMarker_2.htm)

#### `public void DrawMarker(Point3d tip, Vector3d direction, Color color, int thickness, double size, double rotation)`

Draws an arrow marker as a view-aligned widget.

**Parameters:**
- `tip` (Rhino.Geometry.Point3d) — Location of arrow tip point.
- `direction` (Rhino.Geometry.Vector3d) — Direction of arrow.
- `color` (System.Drawing.Color) — Color of arrow widget.
- `thickness` (System.Int32) — Thickness of arrow widget lines.
- `size` (System.Double) — Size (in pixels) of the arrow shaft.
- `rotation` (System.Double) — Rotational angle adjustment (in radians, counter-clockwise of direction.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMarker_3.htm)

#### `public void DrawMeshFalseColors(Mesh mesh)`

Draws the mesh faces as false color patches. The mesh must have Vertex Colors defined for this to work.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to draw.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMeshFalseColors.htm)

#### `public void DrawMeshShaded(Mesh mesh, DisplayMaterial material)`

Draws the shaded faces of a given mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to draw.
- `material` (Rhino.Display.DisplayMaterial) — Material to draw faces with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMeshShaded.htm)

#### `public void DrawMeshShaded(Mesh mesh, DisplayMaterial material, int[] faceIndices)`

Draws the shaded faces of a given mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to draw.
- `material` (Rhino.Display.DisplayMaterial) — Material to draw faces with.
- `faceIndices` (System.Int32[]) — Indices of specific faces to draw

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMeshShaded_1.htm)

#### `public void DrawMeshVertices(Mesh mesh, Color color)`

Draws all the vertices in a given mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh for vertex drawing.
- `color` (System.Drawing.Color) — Color of mesh vertices.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMeshVertices.htm)

#### `public void DrawMeshWires(Mesh mesh, Color color)`

Draws all the wires in a given mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh for wire drawing.
- `color` (System.Drawing.Color) — Color of mesh wires.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMeshWires.htm)

#### `public void DrawMeshWires(Mesh mesh, Color color, int thickness)`

Draws all the wires in a given mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh for wire drawing.
- `color` (System.Drawing.Color) — Color of mesh wires.
- `thickness` (System.Int32) — Thickness (in pixels) of mesh wires.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawMeshWires_1.htm)

#### `public void DrawObject(RhinoObject rhinoObject)`

Draws a RhinoObject.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — The Rhino object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawObject.htm)

#### `public void DrawObject(RhinoObject rhinoObject, Transform xform)`

Draws a RhinoObject with an applied transformation.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — The Rhino object.
- `xform` (Rhino.Geometry.Transform) — The transformation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawObject_1.htm)

#### `public void DrawParticles(ParticleSystem particles)`



**Parameters:**
- `particles` (Rhino.Geometry.ParticleSystem) — [Missing <param name="particles"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawParticles(Rhino.Geometry.ParticleSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawParticles.htm)

#### `public void DrawParticles(ParticleSystem particles, DisplayBitmap bitmap)`



**Parameters:**
- `particles` (Rhino.Geometry.ParticleSystem) — [Missing <param name="particles"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawParticles(Rhino.Geometry.ParticleSystem,Rhino.Display.DisplayBitmap)"]
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawParticles(Rhino.Geometry.ParticleSystem,Rhino.Display.DisplayBitmap)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawParticles_1.htm)

#### `public void DrawParticles(ParticleSystem particles, DisplayBitmap[] bitmaps)`



**Parameters:**
- `particles` (Rhino.Geometry.ParticleSystem) — [Missing <param name="particles"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawParticles(Rhino.Geometry.ParticleSystem,Rhino.Display.DisplayBitmap[])"]
- `bitmaps` (Rhino.Display.DisplayBitmap[]) — [Missing <param name="bitmaps"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawParticles(Rhino.Geometry.ParticleSystem,Rhino.Display.DisplayBitmap[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawParticles_2.htm)

#### `public void DrawPatternedLine(Line line, Color color, int pattern, int thickness)`

Draws a single line with specified pattern.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line to draw.
- `color` (System.Drawing.Color) — Color of line.
- `pattern` (System.Int32) — Pattern of the line (like 0x00001111 for dotted line).
- `thickness` (System.Int32) — Thickness (in pixels) of lines.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPatternedLine.htm)

#### `public void DrawPatternedLine(Point3d from, Point3d to, Color color, int pattern, int thickness)`

Draws a single line with specified pattern.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — Line start point.
- `to` (Rhino.Geometry.Point3d) — Line end point.
- `color` (System.Drawing.Color) — Color of line.
- `pattern` (System.Int32) — Pattern of the line (like 0x00001111 for dotted line).
- `thickness` (System.Int32) — Thickness (in pixels) of lines.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPatternedLine_1.htm)

#### `public void DrawPatternedPolyline(IEnumerable<Point3d> points, Color color, int pattern, int thickness, bool close)`

Draws a set of connected lines (polyline) with specified pattern.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — End points of each line segment.
- `color` (System.Drawing.Color) — Color of polyline.
- `pattern` (System.Int32) — Pattern to use for the line (like 0x00001111 for dotted).
- `thickness` (System.Int32) — Thickness (in pixels) of lines.
- `close` (System.Boolean) — Draw a line between the first and last points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPatternedPolyline.htm)

#### `public void DrawPoint(Point3d point)`

Draws a point using the current display attribute size, style and color

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Location of point in world coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoint.htm)

#### `public void DrawPoint(Point3d point, Color color)`

Draws a point with a given radius, style and color.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Location of point in world coordinates.
- `color` (System.Drawing.Color) — Color of point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoint_4.htm)

#### `public void DrawPoint(Point3d point, PointStyle style, Color strokeColor, Color fillColor, float radius, float strokeWidth, float secondarySize, float rotationRadians, bool diameterIsInPixels, bool autoScaleForDpi)`



**Parameters:**
- `point` (Rhino.Geometry.Point3d) — [Missing <param name="point"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `style` (Rhino.Display.PointStyle) — [Missing <param name="style"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `radius` (System.Single) — [Missing <param name="radius"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `secondarySize` (System.Single) — [Missing <param name="secondarySize"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `rotationRadians` (System.Single) — [Missing <param name="rotationRadians"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `diameterIsInPixels` (System.Boolean) — [Missing <param name="diameterIsInPixels"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `autoScaleForDpi` (System.Boolean) — [Missing <param name="autoScaleForDpi"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoint(Rhino.Geometry.Point3d,Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoint_1.htm)

#### `public void DrawPoint(Point3d point, PointStyle style, int radius, Color color)`

Draws a point with a given radius, style and color.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Location of point in world coordinates.
- `style` (Rhino.Display.PointStyle) — Point display style.
- `radius` (System.Int32) — Point size in pixels.
- `color` (System.Drawing.Color) — Color of point. If style is ControlPoint, this will be the border color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoint_2.htm)

#### `public void DrawPoint(Point3d point, PointStyle style, float radius, Color color)`

Draws a point with a given radius, style and color.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — Location of point in world coordinates.
- `style` (Rhino.Display.PointStyle) — Point display style.
- `radius` (System.Single) — Point size in pixels.
- `color` (System.Drawing.Color) — Color of point. If style is ControlPoint, this will be the border color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoint_3.htm)

#### `public void DrawPointCloud(PointCloud cloud, int size)`

Draws a point cloud.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — Point cloud to draw, if the cloud has a color array, it will be used, otherwise the points will be black.
- `size` (System.Int32) — Size of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPointCloud.htm)

#### `public void DrawPointCloud(PointCloud cloud, int size, Color color)`

Draws a point cloud.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — Point cloud to draw.
- `size` (System.Int32) — Size of points.
- `color` (System.Drawing.Color) — Color of points in the cloud, if the cloud has a color array this setting is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPointCloud_1.htm)

#### `public void DrawPointCloud(PointCloud cloud, float size)`

Draws a point cloud.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — Point cloud to draw, if the cloud has a color array, it will be used, otherwise the points will be black.
- `size` (System.Single) — Size of points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPointCloud_2.htm)

#### `public void DrawPointCloud(PointCloud cloud, float size, Color color)`

Draws a point cloud.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — Point cloud to draw.
- `size` (System.Single) — Size of points.
- `color` (System.Drawing.Color) — Color of points in the cloud, if the cloud has a color array this setting is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPointCloud_3.htm)

#### `public void DrawPoints(IEnumerable<Point3d> points, PointStyle style, Color strokeColor, Color fillColor, float radius, float strokeWidth, float secondarySize, float rotationRadians, bool diameterIsInPixels, bool autoScaleForDpi)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `style` (Rhino.Display.PointStyle) — [Missing <param name="style"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `radius` (System.Single) — [Missing <param name="radius"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `secondarySize` (System.Single) — [Missing <param name="secondarySize"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `rotationRadians` (System.Single) — [Missing <param name="rotationRadians"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `diameterIsInPixels` (System.Boolean) — [Missing <param name="diameterIsInPixels"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]
- `autoScaleForDpi` (System.Boolean) — [Missing <param name="autoScaleForDpi"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawPoints(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.Display.PointStyle,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,System.Single,System.Single,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoints.htm)

#### `public void DrawPoints(IEnumerable<Point3d> points, PointStyle style, int radius, Color color)`

Draw a set of points with a given radius, style and color.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Location of points in world coordinates.
- `style` (Rhino.Display.PointStyle) — Point display style.
- `radius` (System.Int32) — Point size in pixels.
- `color` (System.Drawing.Color) — Color of points. If style is ControlPoint, this will be the border color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoints_1.htm)

#### `public void DrawPoints(IEnumerable<Point3d> points, PointStyle style, float radius, Color color)`

Draw a set of points with a given radius, style and color.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Location of points in world coordinates.
- `style` (Rhino.Display.PointStyle) — Point display style.
- `radius` (System.Single) — Point size in pixels.
- `color` (System.Drawing.Color) — Color of points. If style is ControlPoint, this will be the border color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPoints_2.htm)

#### `public void DrawPolygon(IEnumerable<Point3d> points, Color color, bool filled)`

Draws a filled, convex polygon from a collection of points.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Collection of world coordinate points that are connected by lines to form a closed shape. Collection must contain at least 3 points.
- `color` (System.Drawing.Color) — Color to draw with.
- `filled` (System.Boolean) — true if the closed area should be filled with color. false if you want to draw just the border of the closed shape.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPolygon.htm)

#### `public void DrawPolyline(IEnumerable<Point3d> polyline, Color color)`

Draws a single Polyline object.

**Parameters:**
- `polyline` (System.Collections.Generic.IEnumerable<Point3d>) — Polyline to draw.
- `color` (System.Drawing.Color) — Color to draw in.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPolyline.htm)

#### `public void DrawPolyline(IEnumerable<Point3d> polyline, Color color, int thickness)`

Draws a single Polyline object.

**Parameters:**
- `polyline` (System.Collections.Generic.IEnumerable<Point3d>) — Polyline to draw.
- `color` (System.Drawing.Color) — Color to draw in.
- `thickness` (System.Int32) — Thickness (in pixels) of the Polyline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawPolyline_1.htm)

#### `public void DrawRoundedRectangle(PointF center, float pixelWidth, float pixelHeight, float cornerRadius, Color strokeColor, float strokeWidth, Color fillColor)`



**Parameters:**
- `center` (System.Drawing.PointF) — [Missing <param name="center"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]
- `pixelWidth` (System.Single) — [Missing <param name="pixelWidth"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]
- `pixelHeight` (System.Single) — [Missing <param name="pixelHeight"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]
- `cornerRadius` (System.Single) — [Missing <param name="cornerRadius"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawRoundedRectangle(System.Drawing.PointF,System.Single,System.Single,System.Single,System.Drawing.Color,System.Single,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawRoundedRectangle.htm)

#### `public void DrawSphere(Sphere sphere, Color color)`

Draw a wireframe sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — Sphere to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSphere.htm)

#### `public void DrawSphere(Sphere sphere, Color color, int thickness)`

Draw a wireframe sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — Sphere to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of Sphere wires.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSphere_1.htm)

#### `public void DrawSprite(DisplayBitmap bitmap, Point2d screenLocation, float size)`



**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single)"]
- `screenLocation` (Rhino.Geometry.Point2d) — [Missing <param name="screenLocation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single)"]
- `size` (System.Single) — [Missing <param name="size"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprite.htm)

#### `public void DrawSprite(DisplayBitmap bitmap, Point2d screenLocation, float size, Color blendColor)`



**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Drawing.Color)"]
- `screenLocation` (Rhino.Geometry.Point2d) — [Missing <param name="screenLocation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Drawing.Color)"]
- `size` (System.Single) — [Missing <param name="size"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Drawing.Color)"]
- `blendColor` (System.Drawing.Color) — [Missing <param name="blendColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprite_1.htm)

#### `public void DrawSprite(DisplayBitmap bitmap, Point2d screenLocation, float width, float height)`

Draw screen oriented image centered at 2d screen location

**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Single)"]
- `screenLocation` (Rhino.Geometry.Point2d) — [Missing <param name="screenLocation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Single)"]
- `width` (System.Single) — [Missing <param name="width"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Single)"]
- `height` (System.Single) — [Missing <param name="height"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point2d,System.Single,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprite_2.htm)

#### `public void DrawSprite(DisplayBitmap bitmap, Point3d worldLocation, float size, bool sizeInWorldSpace)`



**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Boolean)"]
- `worldLocation` (Rhino.Geometry.Point3d) — [Missing <param name="worldLocation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Boolean)"]
- `size` (System.Single) — [Missing <param name="size"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Boolean)"]
- `sizeInWorldSpace` (System.Boolean) — [Missing <param name="sizeInWorldSpace"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprite_3.htm)

#### `public void DrawSprite(DisplayBitmap bitmap, Point3d worldLocation, float size, Color blendColor, bool sizeInWorldSpace)`



**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Drawing.Color,System.Boolean)"]
- `worldLocation` (Rhino.Geometry.Point3d) — [Missing <param name="worldLocation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Drawing.Color,System.Boolean)"]
- `size` (System.Single) — [Missing <param name="size"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Drawing.Color,System.Boolean)"]
- `blendColor` (System.Drawing.Color) — [Missing <param name="blendColor"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Drawing.Color,System.Boolean)"]
- `sizeInWorldSpace` (System.Boolean) — [Missing <param name="sizeInWorldSpace"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprite(Rhino.Display.DisplayBitmap,Rhino.Geometry.Point3d,System.Single,System.Drawing.Color,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprite_4.htm)

#### `public void DrawSprites(DisplayBitmap bitmap, DisplayBitmapDrawList items, float size, bool sizeInWorldSpace)`



**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,System.Boolean)"]
- `items` (Rhino.Display.DisplayBitmapDrawList) — [Missing <param name="items"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,System.Boolean)"]
- `size` (System.Single) — [Missing <param name="size"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,System.Boolean)"]
- `sizeInWorldSpace` (System.Boolean) — [Missing <param name="sizeInWorldSpace"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprites_1.htm)

#### `public void DrawSprites(DisplayBitmap bitmap, DisplayBitmapDrawList items, float size, Vector3d translation, bool sizeInWorldSpace)`



**Parameters:**
- `bitmap` (Rhino.Display.DisplayBitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,Rhino.Geometry.Vector3d,System.Boolean)"]
- `items` (Rhino.Display.DisplayBitmapDrawList) — [Missing <param name="items"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,Rhino.Geometry.Vector3d,System.Boolean)"]
- `size` (System.Single) — [Missing <param name="size"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,Rhino.Geometry.Vector3d,System.Boolean)"]
- `translation` (Rhino.Geometry.Vector3d) — [Missing <param name="translation"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,Rhino.Geometry.Vector3d,System.Boolean)"]
- `sizeInWorldSpace` (System.Boolean) — [Missing <param name="sizeInWorldSpace"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawSprites(Rhino.Display.DisplayBitmap,Rhino.Display.DisplayBitmapDrawList,System.Single,Rhino.Geometry.Vector3d,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSprites.htm)

#### `public bool DrawStereoFrameBuffer(ViewportInfo viewportLeft, ViewportInfo viewportRight, out uint handleLeft, out uint handleRight)`

Draws the viewport as seen from the left and the right eye viewports and returns the result as OpenGL texture handles.

**Parameters:**
- `viewportLeft` (Rhino.DocObjects.ViewportInfo) — The viewport representing the left eye location and look direction.
- `viewportRight` (Rhino.DocObjects.ViewportInfo) — The viewport representing the right eye location and look direction.
- `handleLeft` (System.UInt32) — Will contain the OpenGL texture handle which references the left output color buffer.
- `handleRight` (System.UInt32) — Will contain the OpenGL texture handle which references the right output color buffer.

**Returns:** `Boolean` — true if drawing succeeded, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawStereoFrameBuffer.htm)

#### `public void DrawSubDShaded(SubD subd, DisplayMaterial material)`

Draw a shaded mesh representation of a SubD

**Parameters:**
- `subd` (Rhino.Geometry.SubD) — SubD to draw
- `material` (Rhino.Display.DisplayMaterial) — Material to draw faces with

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSubDShaded.htm)

#### `public void DrawSubDWires(SubD subd, Color color, float thickness)`

Draws all the wireframe curves of a SubD object

**Parameters:**
- `subd` (Rhino.Geometry.SubD) — SubD to draw
- `color` (System.Drawing.Color) — wire color
- `thickness` (System.Single) — wire thickness

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSubDWires.htm)

#### `public void DrawSurface(Surface surface, Color wireColor, int wireDensity)`

Draw wireframe display for a single surface.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — Surface to draw.
- `wireColor` (System.Drawing.Color) — Color to draw with.
- `wireDensity` (System.Int32) — Thickness (in pixels) or wires to draw.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawSurface.htm)

#### `public void DrawText(TextEntity text, Color color)`



**Parameters:**
- `text` (Rhino.Geometry.TextEntity) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawText.htm)

#### `public void DrawText(TextEntity text, Color color, double scale)`



**Parameters:**
- `text` (Rhino.Geometry.TextEntity) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color,System.Double)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color,System.Double)"]
- `scale` (System.Double) — [Missing <param name="scale"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawText_2.htm)

#### `public void DrawText(TextEntity text, Color color, Transform xform)`



**Parameters:**
- `text` (Rhino.Geometry.TextEntity) — [Missing <param name="text"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color,Rhino.Geometry.Transform)"]
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color,Rhino.Geometry.Transform)"]
- `xform` (Rhino.Geometry.Transform) — [Missing <param name="xform"/> documentation for "M:Rhino.Display.DisplayPipeline.DrawText(Rhino.Geometry.TextEntity,System.Drawing.Color,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawText_1.htm)

#### `public static Bitmap DrawToBitmap(RhinoViewport viewport, int width, int height)`

Draw a given viewport to an off-screen bitmap.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — Viewport to draw.
- `width` (System.Int32) — Width of target image.
- `height` (System.Int32) — Height of target image.

**Returns:** `Bitmap` — A bitmap containing the given view, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawToBitmap.htm)

#### `public void DrawTorus(Torus torus, Color color)`

Draw a wireframe torus.

**Parameters:**
- `torus` (Rhino.Geometry.Torus) — Torus to draw.
- `color` (System.Drawing.Color) — Color to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawTorus.htm)

#### `public void DrawTorus(Torus torus, Color color, int thickness)`

Draw a wireframe torus.

**Parameters:**
- `torus` (Rhino.Geometry.Torus) — Torus to draw.
- `color` (System.Drawing.Color) — Color to draw with.
- `thickness` (System.Int32) — Thickness (in pixels) of torus wires.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawTorus_1.htm)

#### `public void DrawZebraPreview(Brep brep, Color color)`

Draws a shaded Brep with Zebra stripe preview.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — Brep to draw.
- `color` (System.Drawing.Color) — Object color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawZebraPreview.htm)

#### `public void DrawZebraPreview(Mesh mesh, Color color)`

Draws a shaded Mesh with Zebra stripe preview.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to draw.
- `color` (System.Drawing.Color) — Object color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_DrawZebraPreview_1.htm)

#### `public void EnableClippingPlanes(bool enable)`

Enable or disable the Clipping Plane logic of the engine.

**Parameters:**
- `enable` (System.Boolean) — true to enable Clipping Planes, false to disable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_EnableClippingPlanes.htm)

#### `public void EnableColorWriting(bool enable)`

Enable or disable the ColorWriting behavior of the engine.

**Parameters:**
- `enable` (System.Boolean) — true to enable ColorWriting, false to disable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_EnableColorWriting.htm)

#### `public void EnableDepthTesting(bool enable)`

Enable or disable the DepthTesting behavior of the engine. When DepthTesting is disabled, objects in front will no longer occlude objects behind them.

**Parameters:**
- `enable` (System.Boolean) — true to enable DepthTesting, false to disable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_EnableDepthTesting.htm)

#### `public void EnableDepthWriting(bool enable)`

Enable or disable the DepthWriting behavior of the engine. When DepthWriting is disabled, drawn geometry does not affect the Z-Buffer.

**Parameters:**
- `enable` (System.Boolean) — true to enable DepthWriting, false to disable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_EnableDepthWriting.htm)

#### `public void EnableLighting(bool enable)`

Enable or disable the Lighting logic of the engine.

**Parameters:**
- `enable` (System.Boolean) — true to enable Lighting, false to disable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_EnableLighting.htm)

#### `public void Flush()`

Force the pipeline to immediately flush any cached geometry to the display

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Flush.htm)

#### `public static void GetDrawListSerialNumbers(out uint modelSerialNumber, out uint pageSerialNumber)`

Gets the current model and page view draw list serial numbers, which can be used to determine if a model or page view needs to be redrawn.

**Parameters:**
- `modelSerialNumber` (System.UInt32) — The current model draw list serial number.
- `pageSerialNumber` (System.UInt32) — The current page view draw list serial number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_GetDrawListSerialNumbers.htm)

#### `public Light[] GetLights()`

Get lights that this pipeline is current using

**Returns:** `Light[]` — [Missing <returns> documentation for "M:Rhino.Display.DisplayPipeline.GetLights"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_GetLights.htm)

#### `public float[] GetOpenGLCameraToClip()`

Get an array of 16 floats that represents the "camera" to "clip" coordinate transformation in OpenGL's right handed coordinate system

**Returns:** `Single[]` — [Missing <returns> documentation for "M:Rhino.Display.DisplayPipeline.GetOpenGLCameraToClip"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_GetOpenGLCameraToClip.htm)

#### `public float[] GetOpenGLWorldToCamera(bool includeModelTransform)`

Get an array of 16 floats that represents the "world" to "camera" coordinate transformation in OpenGL's right handed coordinate system

**Parameters:**
- `includeModelTransform` (System.Boolean) — [Missing <param name="includeModelTransform"/> documentation for "M:Rhino.Display.DisplayPipeline.GetOpenGLWorldToCamera(System.Boolean)"]

**Returns:** `Single[]` — [Missing <returns> documentation for "M:Rhino.Display.DisplayPipeline.GetOpenGLWorldToCamera(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_GetOpenGLWorldToCamera.htm)

#### `public float[] GetOpenGLWorldToClip(bool includeModelTransform)`

Get an array of 16 floats that represents the "world" to "clip" coordinate transformation in OpenGL's right handed coordinate system

**Parameters:**
- `includeModelTransform` (System.Boolean) — [Missing <param name="includeModelTransform"/> documentation for "M:Rhino.Display.DisplayPipeline.GetOpenGLWorldToClip(System.Boolean)"]

**Returns:** `Single[]` — [Missing <returns> documentation for "M:Rhino.Display.DisplayPipeline.GetOpenGLWorldToClip(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_GetOpenGLWorldToClip.htm)

#### `public bool InterruptDrawing()`

Tests to see if the pipeline should stop drawing more geometry and just show what it has so far. If a drawing operation is taking a long time, this function will return true and tell Rhino it should just finish up and show the frame buffer. This is used in dynamic drawing operations.

**Returns:** `Boolean` — true if the pipeline should stop attempting to draw more geometry and just show the frame buffer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_InterruptDrawing.htm)

#### `public bool IsActive(RhinoObject rhinoObject)`

Determines if an object can be visible in this viewport based on it's object type and display attributes. This test does not check for visibility based on location of the object. NOTE: Use CRhinoDisplayPipeline::IsVisible() to perform "visibility" tests based on location (is some part of the object in the view frustum). Use CRhinoDisplayPipeline::IsActive() to perform "visibility" tests based on object type.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — Object to test.

**Returns:** `Boolean` — true if this object can be drawn in the pipeline's viewport based on it's object type and display attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_IsActive.htm)

#### `public bool IsVisible(BoundingBox bbox)`

Test a given box for visibility inside the view frustum under the current viewport and model transformation settings.

**Parameters:**
- `bbox` (Rhino.Geometry.BoundingBox) — Box to test for visibility.

**Returns:** `Boolean` — true if at least some portion of the box is visible, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_IsVisible_1.htm)

#### `public bool IsVisible(Point3d worldCoordinate)`

Test a given 3d world coordinate point for visibility inside the view frustum under the current viewport and model transformation settings.

**Parameters:**
- `worldCoordinate` (Rhino.Geometry.Point3d) — Point to test for visibility.

**Returns:** `Boolean` — true if the point is visible, false if it is not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_IsVisible_2.htm)

#### `public bool IsVisible(RhinoObject rhinoObject)`

Test a given object for visibility inside the view frustum under the current viewport and model transformation settings. This function calls a virtual IsVisibleFinal function that sub-classed pipelines can add extra tests to. In the base class, this test only tests visibility based on the objects world coordinates location and does not pay attention to the object's attributes. NOTE: Use CRhinoDisplayPipeline::IsVisible() to perform "visibility" tests based on location (is some part of the object in the view frustum). Use CRhinoDisplayPipeline::IsActive() to perform "visibility" tests based on object type.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — Object to test.

**Returns:** `Boolean` — true if the object is visible, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_IsVisible.htm)

#### `public static bool MakeDefaultOpenGLContextCurrent()`

Make a "default" OpenGL context current

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.DisplayPipeline.MakeDefaultOpenGLContextCurrent"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_MakeDefaultOpenGLContextCurrent.htm)

#### `public Rectangle Measure2dText(string text, Point2d definitionPoint, bool middleJustified, double rotationRadians, int height, string fontFace)`

Determines screen rectangle that would be drawn to using the Draw2dText(..) function with the same parameters.

**Parameters:**
- `text` (System.String) — text to measure.
- `definitionPoint` (Rhino.Geometry.Point2d) — either lower-left or middle of text.
- `middleJustified` (System.Boolean) — true=middle justified. false=lower-left justified.
- `rotationRadians` (System.Double) — text rotation in radians
- `height` (System.Int32) — height in pixels (good default is 12)
- `fontFace` (System.String) — font name (good default is "Arial")

**Returns:** `Rectangle` — rectangle in the viewport's screen coordinates on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Measure2dText.htm)

#### `public bool Open()`

Opens the pipeline.

**Returns:** `Boolean` — True if the pipeline was opened, false if it was already open or failed to open.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_Open.htm)

#### `[ObsoleteAttribute("Use PopDepthTesting")] public void PopClipTesting()`

Pop a ClipTesting flag off the engine's stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PopClipTesting.htm)

#### `public void PopCullFaceMode()`

Pop a FaceCull flag off the engine's stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PopCullFaceMode.htm)

#### `public void PopDepthTesting()`

Pop a DepthTesting flag off the engine's stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PopDepthTesting.htm)

#### `public void PopDepthWriting()`

Pop a DepthWriting flag off the engine's stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PopDepthWriting.htm)

#### `public void PopModelTransform()`

Pop a model transformation off the engine's model transform stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PopModelTransform.htm)

#### `[ObsoleteAttribute("Use PushDepthTesting")] public void PushClipTesting(bool enable)`

Push a ClipTesting flag on the engine's stack.

**Parameters:**
- `enable` (System.Boolean) — ClipTesting flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PushClipTesting.htm)

#### `public void PushCullFaceMode(CullFaceMode mode)`

Push a FaceCull flag on the engine's stack.

**Parameters:**
- `mode` (Rhino.Display.CullFaceMode) — FaceCull flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PushCullFaceMode.htm)

#### `public void PushDepthTesting(bool enable)`

Push a DepthTesting flag on the engine's stack.

**Parameters:**
- `enable` (System.Boolean) — DepthTesting flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PushDepthTesting.htm)

#### `public void PushDepthWriting(bool enable)`

Push a DepthWriting flag on the engine's stack.

**Parameters:**
- `enable` (System.Boolean) — DepthWriting flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PushDepthWriting.htm)

#### `public void PushModelTransform(Transform xform)`

Push a model transformation on the engine's model transform stack.

**Parameters:**
- `xform` (Rhino.Geometry.Transform) — Transformation to push.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_PushModelTransform.htm)

#### `public void RemoveClippingPlane(int index)`

Remove a clipping plane from the pipeline for this frame

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Display.DisplayPipeline.RemoveClippingPlane(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipeline_RemoveClippingPlane.htm)

### Properties
- `ActiveObject` (RhinoObject, get) — 
- `ActiveObjectNestingLevel` (Int32, get) — 
- `ActiveObjectNestingStack` (RhinoObject[], get) — 
- `ActiveTopLevelObject` (RhinoObject, get) — 
- `DefaultCurveThickness` (Int32, get) — Gets the curve thickness as defined by the current display mode. Note: this only applies to curve objects, Brep and Mesh wires may have different settings.
- `DepthMode` (DepthMode, get/set) — 
- `DisplayPipelineAttributes` (DisplayPipelineAttributes, get) — 
- `DpiScale` (Single, get) — Scale factor used for high resolution displays. When a monitor that this pipeline is drawing to is at a DPI of 96, this value is one. On high DPI monitors, this value will commonly be greater than one.
- `DrawingGrips` (Boolean, get) — Gets a value that indicates whether the pipeline is currently in a grip drawing operation.
- `DrawingSurfaces` (Boolean, get) — Gets a value that indicates whether the pipeline is currently in a surface drawing operation. Surface drawing means draw the shaded triangles of a mesh representing the surface (mesh, extrusion, or brep). This is useful when inside of a draw event or display conduit to check and see if the geometry is about to be drawn as a shaded set of triangles representing the geometry. See DrawingWires to check and see if the wireframe representation of the geometry is going to be drawn.
- `DrawingWires` (Boolean, get) — Gets a value that indicates whether the pipeline is currently in a curve drawing operation. This is useful when inside of a draw event or display conduit to check and see if the geometry is about to be drawn is going to be drawing the wire representation of the geometry (mesh, extrusion, or brep). See DrawingSurfaces to check and see if the shaded mesh representation of the geometry is going to be drawn.
- `FrameBuffer` (Bitmap, get) — Gets the contents of the frame buffer that this pipeline is drawing to.
- `FrameSize` (Size, get) — Gets the size of the frame buffer that this pipeline is drawing to.
- `IsDynamicDisplay` (Boolean, get) — Gets a value that indicates whether the viewport is in Dynamic Display state. Dynamic display is the state a viewport is in when it is rapidly redrawing because of an operation like panning or rotating. The pipeline will drop some level of detail while inside a dynamic display state to keep the frame rate as high as possible.
- `IsInViewCapture` (Boolean, get) — Gets a value that indicates whether this pipeline is currently drawing for ViewCaptureToFile or ViewCaptureToClipboard
- `IsOpen` (Boolean, get) — Is true of the pipeline is open, false otherwise.
- `IsOpenGL` (Boolean, get) — Gets a value indicating whether or not this pipeline is drawing into an OpenGL context.
- `IsPrinting` (Boolean, get) — Gets a value that indicates whether this pipeline is currently drawing for printing purposes.
- `IsStereoMode` (Boolean, get) — Gets a value that indicates whether this pipeline is currently using an engine that is performing stereo style drawing. Stereo drawing is for providing an "enhanced 3-D" effect through stereo viewing devices.
- `ModelTransform` (Transform, get/set) — Gets or sets the current model transformation that is applied to vertices when drawing.
- `ModelTransformIsIdentity` (Boolean, get) — Gets a value that indicates whether the Model Transform is an Identity transformation.
- `NestLevel` (Int32, get) — Gets the current nested viewport drawing level. This is used to know if you are currently inside the drawing of a nested viewport (detail object in Rhino). Nest level = 0 Drawing is occurring in a standard Rhino viewport or on the page viewport.Nest level = 1 Drawing is occurring inside a detail view object.
- `RenderPass` (Int32, get) — Gets the current pass that the pipeline is in for drawing a frame. Typically drawing a frame requires a single pass through the DrawFrameBuffer function, but some special display effects can be achieved through drawing with multiple passes.
- `ShadingRequired` (Boolean, get/set) — Gets or sets the "ShadingRequired" flag. This flag gets set inside the pipeline when a request is made to draw a shaded mesh but the current render engine doesn't support shaded mesh drawing...at this point the redraw mechanism will make sure everything will work the next time around.
- `StereoProjection` (Int32, get) — Gets the current stereo projection if stereo mode is on. 0 = left1 = right If stereo mode is not enables, this property always returns 0.
- `SupportsShading` (Boolean, get) — Gets whether or not this pipeline supports shaded meshes.
- `Viewport` (RhinoViewport, get) — 
- `ZBiasMode` (ZBiasMode, get/set) — 

### Events
#### `CalculateBoundingBox` (`System.EventHandler<CalculateBoundingBoxEventArgs>`)

**Signature:** `public static event EventHandler<CalculateBoundingBoxEventArgs> CalculateBoundingBox`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_CalculateBoundingBox.htm)

#### `CalculateBoundingBoxZoomExtents` (`System.EventHandler<CalculateBoundingBoxEventArgs>`)

**Signature:** `public static event EventHandler<CalculateBoundingBoxEventArgs> CalculateBoundingBoxZoomExtents`

Calculate a bounding to include in the Zoom Extents command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_CalculateBoundingBoxZoomExtents.htm)

#### `DisplayModeChanged` (`System.EventHandler<DisplayModeChangedEventArgs>`)

**Signature:** `public static event EventHandler<DisplayModeChangedEventArgs> DisplayModeChanged`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_DisplayModeChanged.htm)

#### `DrawForeground` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public static event EventHandler<DrawEventArgs> DrawForeground`

Called after all non-highlighted objects have been drawn and PostDrawObjects has been called. Depth writing and testing are turned OFF. If you want to draw with depth writing/testing, see PostDrawObjects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_DrawForeground.htm)

#### `DrawOverlay` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public static event EventHandler<DrawEventArgs> DrawOverlay`

If Rhino is in a feedback mode, the draw overlay call allows for temporary geometry to be drawn on top of everything in the scene. This is similar to the dynamic draw routine that occurs with custom get point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_DrawOverlay.htm)

#### `InitFrameBuffer` (`System.EventHandler<InitFrameBufferEventArgs>`)

**Signature:** `public static event EventHandler<InitFrameBufferEventArgs> InitFrameBuffer`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_InitFrameBuffer.htm)

#### `ObjectCulling` (`System.EventHandler<CullObjectEventArgs>`)

**Signature:** `public static event EventHandler<CullObjectEventArgs> ObjectCulling`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_ObjectCulling.htm)

#### `PostDrawObject` (`System.EventHandler<DrawObjectEventArgs>`)

**Signature:** `public static event EventHandler<DrawObjectEventArgs> PostDrawObject`

Called right after an individual object has been drawn. NOTE: Do not use this event unless you absolutely need to. It is called for every object in the document and can slow display down if a large number of objects exist in the document

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_PostDrawObject.htm)

#### `PostDrawObjects` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public static event EventHandler<DrawEventArgs> PostDrawObjects`

Called after all non-highlighted objects have been drawn. Depth writing and testing are still turned on. If you want to draw without depth writing/testing, see DrawForeground.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_PostDrawObjects.htm)

#### `PreDrawObject` (`System.EventHandler<DrawObjectEventArgs>`)

**Signature:** `public static event EventHandler<DrawObjectEventArgs> PreDrawObject`

Called right before an individual object is being drawn. NOTE: Do not use this event unless you absolutely need to. It is called for every object in the document and can slow display down if a large number of objects exist in the document

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_PreDrawObject.htm)

#### `PreDrawObjects` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public static event EventHandler<DrawEventArgs> PreDrawObjects`

Called before objects are been drawn. Depth writing and testing are on.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_PreDrawObjects.htm)

#### `PreDrawTransparentObjects` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public static event EventHandler<DrawEventArgs> PreDrawTransparentObjects`

Called before transparent objects have been drawn. Depth writing and testing are on.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_PreDrawTransparentObjects.htm)

#### `ViewportProjectionChanged` (`System.EventHandler<DrawEventArgs>`)

**Signature:** `public static event EventHandler<DrawEventArgs> ViewportProjectionChanged`

Called when the projection changes for a viewport being drawn.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_DisplayPipeline_ViewportProjectionChanged.htm)

## DisplayPipelineAttributes (class)

Represents display pipeline settings, such as "show transparency" and "show grips".

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipelineAttributes.htm)

### Constructors
- `protected DisplayPipelineAttributes(SerializationInfo info, StreamingContext context)` — Initializes a new instance of the DisplayPipelineAttributes class

### Methods
#### `public void Dispose()`

Releases all resources used by the DisplayPipelineAttributes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the DisplayPipelineAttributes and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_Finalize.htm)

#### `public void GetFill(out Color topLeft, out Color bottomLeft, out Color topRight, out Color bottomRight)`

Get fill colors used for clearing the frame buffer

**Parameters:**
- `topLeft` (System.Drawing.Color) — [Missing <param name="topLeft"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.GetFill(System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@)"]
- `bottomLeft` (System.Drawing.Color) — [Missing <param name="bottomLeft"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.GetFill(System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@)"]
- `topRight` (System.Drawing.Color) — [Missing <param name="topRight"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.GetFill(System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@)"]
- `bottomRight` (System.Drawing.Color) — [Missing <param name="bottomRight"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.GetFill(System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@,System.Drawing.Color@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_GetFill.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`



**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — [Missing <param name="info"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.GetObjectData(System.Runtime.Serialization.SerializationInfo,System.Runtime.Serialization.StreamingContext)"]
- `context` (System.Runtime.Serialization.StreamingContext) — [Missing <param name="context"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.GetObjectData(System.Runtime.Serialization.SerializationInfo,System.Runtime.Serialization.StreamingContext)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_GetObjectData.htm)

#### `public void SetFill(Color singleColor)`

Set fill mode to solid color and set the fill color

**Parameters:**
- `singleColor` (System.Drawing.Color) — [Missing <param name="singleColor"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_SetFill.htm)

#### `public void SetFill(Color gradientTop, Color gradientBottom)`

Set fill mode to two color and set the colors

**Parameters:**
- `gradientTop` (System.Drawing.Color) — [Missing <param name="gradientTop"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color,System.Drawing.Color)"]
- `gradientBottom` (System.Drawing.Color) — [Missing <param name="gradientBottom"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_SetFill_1.htm)

#### `public void SetFill(Color gradientTopLeft, Color gradientBottomLeft, Color gradientTopRight, Color gradientBottomRight)`

Set the fill mode to four color gradient and set the colors

**Parameters:**
- `gradientTopLeft` (System.Drawing.Color) — [Missing <param name="gradientTopLeft"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `gradientBottomLeft` (System.Drawing.Color) — [Missing <param name="gradientBottomLeft"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `gradientTopRight` (System.Drawing.Color) — [Missing <param name="gradientTopRight"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `gradientBottomRight` (System.Drawing.Color) — [Missing <param name="gradientBottomRight"/> documentation for "M:Rhino.Display.DisplayPipelineAttributes.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_DisplayPipelineAttributes_SetFill_2.htm)

### Properties
- `AmbientLightingColor` (Color, get/set) — 
- `BoundingBoxMode` (DisplayPipelineAttributes.BoundingBoxDisplayMode, get/set) — 
- `CastShadows` (Boolean, get/set) — Cast shadows.
- `ContextForDraw` (DisplayPipelineAttributes.ContextsForDraw, get) — 
- `CurveColor` (Color, get/set) — Color used for drawing curves
- `CurveThickness` (Int32, get/set) — Pixel thickness for curves
- `DisableConduits` (Boolean, get/set) — 
- `DisableTransparency` (Boolean, get/set) — 
- `EnglishName` (String, get/set) — 
- `FillMode` (DisplayPipelineAttributes.FrameBufferFillMode, get/set) — Get or set the frame buffer fill mode.
- `Id` (Guid, get) — 
- `IgnoreHighlights` (Boolean, get/set) — 
- `LocalName` (String, get) — 
- `LockedObjectsDrawBehindOthers` (Boolean, get/set) — Locked object are drawn behind other objects
- `MeshSpecificAttributes` (DisplayPipelineAttributes.MeshDisplayAttributes, get) — 
- `ObjectColor` (Color, get/set) — 
- `PointRadius` (Single, get/set) — 
- `PointStyle` (PointStyle, get/set) — 
- `RealtimeDisplayId` (Guid, get) — Get the ID of the real-time display engine attached to the view. This will be Guid.Empty if no real-time display engine is in use. This can be the case for instance when starting a _Render session for a real-time viewport integration. That still would cause this ID to be Guid.Empty.
- `RealtimeRenderPasses` (Int32, get/set) — Get or set the real-time passes amount
- `ShadeVertexColors` (Boolean, get/set) — Shade using vertex colors.
- `ShadingEnabled` (Boolean, get/set) — Draw shaded meshes and surfaces. Set to false to use Flat Shading.
- `ShadowColor` (Color, get/set) — 
- `ShowAnnotations` (Boolean, get/set) — Show annotations.
- `ShowClippingPlanes` (Boolean, get/set) — Show clipping planes.
- `ShowCurves` (Boolean, get/set) — Draw curves
- `ShowGrips` (Boolean, get/set) — 
- `ShowIsoCurves` (Boolean, get/set) — Draw surface ISO curves.
- `ShowLights` (Boolean, get/set) — Show light widgets.
- `ShowPointClouds` (Boolean, get/set) — Show point clouds.
- `ShowPoints` (Boolean, get/set) — Show points.
- `ShowRealtimeRenderProgressBar` (Boolean, get/set) — Get or set whether the display is used for preview rendering or not.
- `ShowSurfaceEdges` (Boolean, get/set) — Show surface edges.
- `ShowTangentEdges` (Boolean, get/set) — Show tangent edges.
- `ShowTangentSeams` (Boolean, get/set) — Show tangent seams.
- `ShowText` (Boolean, get/set) — Show text.
- `StereoContext` (StereoContext, get/set) — Get or set the stereo render context.
- `SurfaceEdgeThickness` (Int32, get/set) — Thickness for surface edges
- `UseAssignedObjectMaterial` (Boolean, get/set) — Gets or sets whether objects ought to be drawn using their assigned rendering material.
- `UseCustomObjectColor` (Boolean, get/set) — Gets or sets whether objects ought to be drawn using a custom color.
- `UseCustomObjectColorBackfaces` (Boolean, get/set) — Gets or sets whether objects ought to be drawn using a custom color for back faces.
- `UseCustomObjectMaterial` (Boolean, get/set) — Gets or sets whether objects ought to be drawn using a custom material.
- `UseCustomObjectMaterialBackfaces` (Boolean, get/set) — Gets or sets whether objects ought to be drawn using a custom material on backfaces.
- `UseSingleCurveColor` (Boolean, get/set) — Use a single color for drawing curves
- `ViewSpecificAttributes` (DisplayPipelineAttributes.ViewDisplayAttributes, get) — 
- `XrayAllObjects` (Boolean, get/set) — 

## DisplayPipelineAttributes.BoundingBoxDisplayMode (enum)

[Missing <summary> documentation for "T:Rhino.Display.DisplayPipelineAttributes.BoundingBoxDisplayMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipelineAttributes_BoundingBoxDisplayMode.htm)

### Values
- `None` = `0`
- `OnDuringDynamicDisplay` = `2`
- `OnAlways` = `1`

## DisplayPipelineAttributes.ContextsForDraw (enum)

[Missing <summary> documentation for "T:Rhino.Display.DisplayPipelineAttributes.ContextsForDraw"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipelineAttributes_ContextsForDraw.htm)

### Values
- `Unset` = `0`
- `FilePreview` = `1`
- `ViewCapture` = `2`
- `Printing` = `3`
- `UIPreview` = `4`
- `Mask` = `5`
- `RenderOverlays` = `6`

## DisplayPipelineAttributes.FrameBufferFillMode (enum)

[Missing <summary> documentation for "T:Rhino.Display.DisplayPipelineAttributes.FrameBufferFillMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipelineAttributes_FrameBufferFillMode.htm)

### Values
- `DefaultColor` = `1`
- `SolidColor` = `2`
- `Gradient2Color` = `3`
- `Gradient4Color` = `4`
- `Bitmap` = `5`
- `Renderer` = `6`
- `Transparent` = `7`

## DisplayPipelineAttributes.MeshDisplayAttributes (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayPipelineAttributes.MeshDisplayAttributes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipelineAttributes_MeshDisplayAttributes.htm)

### Properties
- `AllMeshWiresColor` (Color, get/set) — Color.Empty means that we are NOT using a single color for all mesh wires.
- `HighlightMeshes` (Boolean, get/set) — 
- `MeshWireThickness` (Int32, get/set) — 
- `ShowMeshVertices` (Boolean, get/set) — 
- `ShowMeshWires` (Boolean, get/set) — 

## DisplayPipelineAttributes.ViewDisplayAttributes (class)

[Missing <summary> documentation for "T:Rhino.Display.DisplayPipelineAttributes.ViewDisplayAttributes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayPipelineAttributes_ViewDisplayAttributes.htm)

### Properties
- `BlendGrid` (Boolean, get/set) — 
- `DrawGrid` (Boolean, get/set) — 
- `DrawGridAxes` (Boolean, get/set) — 
- `DrawTransparentGridPlane` (Boolean, get/set) — 
- `DrawWorldAxes` (Boolean, get/set) — 
- `DrawZAxis` (Boolean, get/set) — 
- `HorizontalViewportScale` (Double, get/set) — 
- `ShowGridOnTop` (Boolean, get/set) — 
- `UseDocumentGrid` (Boolean, get/set) — 
- `VerticalViewportScale` (Double, get/set) — 
- `WorldAxisColorX` (Color, get/set) — 
- `WorldAxisColorY` (Color, get/set) — 
- `WorldAxisColorZ` (Color, get/set) — 

## DisplayTechnology (enum)

Graphics display techologies.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DisplayTechnology.htm)

### Values
- `None` = `0`
- `OpenGL` = `1`
- `Metal` = `2`
- `DirectX` = `3`
- `Software` = `4`
- `Vulkan` = `5`

## DrawEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.DrawEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DrawEventArgs.htm)

### Properties
- `Display` (DisplayPipeline, get) — 
- `RhinoDoc` (RhinoDoc, get) — 
- `Viewport` (RhinoViewport, get) — 

## DrawForegroundEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.DrawForegroundEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DrawForegroundEventArgs.htm)

### Properties
- `Display` (DisplayPipeline, get) — (Inherited from DrawEventArgs.)
- `DrawWorldAxes` (Boolean, get/set) — 
- `RhinoDoc` (RhinoDoc, get) — (Inherited from DrawEventArgs.)
- `Viewport` (RhinoViewport, get) — (Inherited from DrawEventArgs.)
- `WorldAxesDrawn` (Boolean, get/set) — 

## DrawFrameStages (enum)

[Missing <summary> documentation for "T:Rhino.Display.DrawFrameStages"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DrawFrameStages.htm)

### Values
- `InitializeFrameBuffer` = `1`
- `SetupFrustum` = `2`
- `ObjectCulling` = `4`
- `CalculateBoundingBox` = `8`
- `CalculateClippingPlanes` = `16`
- `SetupLighting` = `32`
- `DrawBackground` = `64`
- `PreDrawObjects` = `128`
- `DrawObject` = `256`
- `PostDrawObjects` = `512`
- `DrawForeGround` = `1024`
- `DrawOverlay` = `2048`
- `PostProcessFrameBuffer` = `4096`
- `MeshingParameters` = `8192`
- `ObjectDisplayAttributes` = `16384`
- `PreObjectDraw` = `32768`
- `PostObjectDraw` = `65536`
- `ViewExtents` = `131072`
- `DrawMiddleGround` = `896`
- `ObjectBasedChannel` = `114948`
- `All` = `4294836223`

## DrawObjectEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.DrawObjectEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_DrawObjectEventArgs.htm)

### Properties
- `Display` (DisplayPipeline, get) — (Inherited from DrawEventArgs.)
- `DrawObject` (Boolean, get/set) — 
- `RhinoDoc` (RhinoDoc, get) — (Inherited from DrawEventArgs.)
- `RhinoObject` (RhinoObject, get) — 
- `Viewport` (RhinoViewport, get) — (Inherited from DrawEventArgs.)

## GradientType (enum)

Style of color gradient

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_GradientType.htm)

### Values
- `None` = `0` — No gradient
- `Linear` = `1` — Linear (or axial) gradient between two points
- `Radial` = `2` — Radial (or spherical) gradient using a center point and a radius
- `LinearDisabled` = `3` — Disabled linear gradient. Useful for keeping gradient information around, but not having it displayed
- `RadialDisabled` = `4` — Disabled radial gradient. Useful for keeping gradient information around, but not having it displayed

## InitFrameBufferEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.InitFrameBufferEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_InitFrameBufferEventArgs.htm)

### Methods
#### `public void SetFill(Color color)`



**Parameters:**
- `color` (System.Drawing.Color) — [Missing <param name="color"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_InitFrameBufferEventArgs_SetFill.htm)

#### `public void SetFill(Color top, Color bottom)`



**Parameters:**
- `top` (System.Drawing.Color) — [Missing <param name="top"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color,System.Drawing.Color)"]
- `bottom` (System.Drawing.Color) — [Missing <param name="bottom"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_InitFrameBufferEventArgs_SetFill_1.htm)

#### `public void SetFill(Color topLeft, Color bottomLeft, Color topRight, Color bottomRight)`



**Parameters:**
- `topLeft` (System.Drawing.Color) — [Missing <param name="topLeft"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `bottomLeft` (System.Drawing.Color) — [Missing <param name="bottomLeft"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `topRight` (System.Drawing.Color) — [Missing <param name="topRight"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]
- `bottomRight` (System.Drawing.Color) — [Missing <param name="bottomRight"/> documentation for "M:Rhino.Display.InitFrameBufferEventArgs.SetFill(System.Drawing.Color,System.Drawing.Color,System.Drawing.Color,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_InitFrameBufferEventArgs_SetFill_2.htm)

## PageViewSpaceChangeEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.PageViewSpaceChangeEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_PageViewSpaceChangeEventArgs.htm)

### Properties
- `NewActiveDetailId` (Guid, get) — The id of the detail object was set active. Note, if this id is equal to Guid.Empty, then the active detail object is the page view itself.
- `OldActiveDetailId` (Guid, get) — The id of the previously active detail object. Note, if this id is equal to Guid.Empty, then the active detail object was the page view itself.
- `PageView` (RhinoPageView, get) — The page view on which a different detail object was set active.

## PointStyle (enum)

Defines styles used for drawing points.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_PointStyle.htm)

### Values
- `Simple` = `0` — Square
- `ControlPoint` = `1` — Square with white center
- `ActivePoint` = `2` — Like a control point but includes vertical/horizontal crosshair lines.
- `X` = `3` — X shape
- `RoundSimple` = `4` — Circular
- `RoundControlPoint` = `5` — Circular with white center
- `RoundActivePoint` = `6` — Round control point with crosshair lines
- `Circle` = `4` — Circle or ring shape when secondarySize is set
- `Square` = `0` — Square or diamond shape when rotated
- `Triangle` = `7` — Triangular shape
- `Heart` = `8` — Heart shape
- `Chevron` = `9` — Chevron shape (two directional arrows)
- `Clover` = `10` — Three unioned circles
- `Tag` = `11` — Tag shape
- `Asterisk` = `12` — * shape
- `Pin` = `13` — Map style pin symbol. secondarySize defines hole zie in pin
- `ArrowTail` = `14` — Arrow shape with tail as definition point. Shape is offset from tail by secondarySize.
- `ArrowTip` = `15` — Arrow shape with tip as definition point. Shape is offset from tip by secondarySize.

## RhinoPageView (class)

A window that contains a single layout 'page'

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_RhinoPageView.htm)

### Methods
#### `public DetailViewObject AddDetailView(string title, Point2d corner0, Point2d corner1, DefinedViewportProjection initialProjection)`

Creates a detail view object that is displayed on this page and adds it to the document.

**Parameters:**
- `title` (System.String) — The detail view title.
- `corner0` (Rhino.Geometry.Point2d) — Corners of the detail view in world coordinates.
- `corner1` (Rhino.Geometry.Point2d) — Corners of the detail view in world coordinates.
- `initialProjection` (Rhino.Display.DefinedViewportProjection) — The defined initial projection type.

**Returns:** `DetailViewObject` — Newly created detail view on success, null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_AddDetailView.htm)

#### `public Bitmap CaptureToBitmap()`

Capture View contents to a bitmap.

**Returns:** `Bitmap` — The bitmap of the complete view.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap.htm)

#### `public Bitmap CaptureToBitmap(bool grid, bool worldAxes, bool cplaneAxes)`

Captures the view contents to a bitmap allowing for visibility of grid and axes.

**Parameters:**
- `grid` (System.Boolean) — true if the construction plane grid should be visible.
- `worldAxes` (System.Boolean) — true if the world axis should be visible.
- `cplaneAxes` (System.Boolean) — true if the construction plane close the grid should be visible.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_3.htm)

#### `public Bitmap CaptureToBitmap(DisplayModeDescription mode)`

Capture View contents to a bitmap using a display mode description to define how drawing is performed.

**Parameters:**
- `mode` (Rhino.Display.DisplayModeDescription) — The display mode.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_1.htm)

#### `public Bitmap CaptureToBitmap(DisplayPipelineAttributes attributes)`

Captures view contents to a bitmap using display attributes to define how drawing is performed.

**Parameters:**
- `attributes` (Rhino.Display.DisplayPipelineAttributes) — The specific display mode attributes.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_2.htm)

#### `public Bitmap CaptureToBitmap(Size size)`

Capture View contents to a bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Size of Bitmap to capture to.

**Returns:** `Bitmap` — The bitmap of the specified part of the view.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_4.htm)

#### `public Bitmap CaptureToBitmap(Size size, bool grid, bool worldAxes, bool cplaneAxes)`

Captures a part of the view contents to a bitmap allowing for visibility of grid and axes.

**Parameters:**
- `size` (System.Drawing.Size) — The width and height of the returned bitmap.
- `grid` (System.Boolean) — true if the construction plane grid should be visible.
- `worldAxes` (System.Boolean) — true if the world axis should be visible.
- `cplaneAxes` (System.Boolean) — true if the construction plane close the grid should be visible.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_7.htm)

#### `public Bitmap CaptureToBitmap(Size size, DisplayModeDescription mode)`

Capture View contents to a bitmap using a display mode description to define how drawing is performed.

**Parameters:**
- `size` (System.Drawing.Size) — The width and height of the returned bitmap.
- `mode` (Rhino.Display.DisplayModeDescription) — The display mode.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_5.htm)

#### `public Bitmap CaptureToBitmap(Size size, DisplayPipelineAttributes attributes)`

Capture View contents to a bitmap using display attributes to define how drawing is performed.

**Parameters:**
- `size` (System.Drawing.Size) — The width and height of the returned bitmap.
- `attributes` (Rhino.Display.DisplayPipelineAttributes) — The specific display mode attributes.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_6.htm)

#### `public Point ClientToScreen(Point clientPoint)`

(Inherited from RhinoView.)

**Parameters:**
- `clientPoint` (System.Drawing.Point) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(System.Drawing.Point)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ClientToScreen_1.htm)

#### `public Point2d ClientToScreen(Point2d clientPoint)`

(Inherited from RhinoView.)

**Parameters:**
- `clientPoint` (Rhino.Geometry.Point2d) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(Rhino.Geometry.Point2d)"]

**Returns:** `Point2d` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ClientToScreen.htm)

#### `public bool Close()`

Remove this View from Rhino. DO NOT attempt to use this instance of this class after calling Close.

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_Close.htm)

#### `public bool CreateShadedPreviewImage(string imagePath, Size size, bool ignoreHighlights, bool drawConstructionPlane, bool useGhostedShading)`

Creates a bitmap preview image of model.

**Parameters:**
- `imagePath` (System.String) — [in] The name of the bitmap file to create. The extension of the imagePath controls the format of the bitmap file created (BMP, TGA, JPG, PCX, PNG, TIF).
- `size` (System.Drawing.Size) — [in] The width and height of the bitmap in pixels.
- `ignoreHighlights` (System.Boolean) — true if highlighted elements should be drawn normally.
- `drawConstructionPlane` (System.Boolean) — true if the CPlane should be drawn.
- `useGhostedShading` (System.Boolean) — true if ghosted shading (partially transparent shading) should be used.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CreateShadedPreviewImage.htm)

#### `public bool CreateWireframePreviewImage(string imagePath, Size size, bool ignoreHighlights, bool drawConstructionPlane)`

Creates a bitmap preview image of model.

**Parameters:**
- `imagePath` (System.String) — [in] The name of the bitmap file to create. The extension of the imagePath controls the format of the bitmap file created (BMP, TGA, JPG, PCX, PNG, TIF).
- `size` (System.Drawing.Size) — [in] The width and height of the bitmap in pixels.
- `ignoreHighlights` (System.Boolean) — true if highlighted elements should be drawn normally.
- `drawConstructionPlane` (System.Boolean) — true if the CPlane should be drawn.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CreateWireframePreviewImage.htm)

#### `public RhinoPageView Duplicate(bool duplicatePageGeometry)`

Copies a page view.

**Parameters:**
- `duplicatePageGeometry` (System.Boolean) — Set true if you want the page view geometry copied, along with the view.

**Returns:** `RhinoPageView` — The new page view if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_Duplicate.htm)

#### `public override bool Equals(Object obj)`

(Inherited from RhinoView.)

**Parameters:**
- `obj` (System.Object) — [Missing <param name="obj"/> documentation for "M:Rhino.Display.RhinoView.Equals(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.Equals(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_Equals.htm)

#### `public DetailViewObject[] GetDetailViews()`

Gets a list of the detail view objects associated with this layout.

**Returns:** `DetailViewObject[]` — An array of detail view objects if successful, an empty array if the layout has no details.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_GetDetailViews.htm)

#### `public override int GetHashCode()`

(Inherited from RhinoView.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_GetHashCode.htm)

#### `public Bitmap GetPreviewImage(Size size, bool grayScale)`

Creates a preview image of the page.

**Parameters:**
- `size` (System.Drawing.Size) — The size of the preview image.
- `grayScale` (System.Boolean) — Set true to produce a grayscale image, false to produce a color image.

**Returns:** `Bitmap` — A bitmap if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_GetPreviewImage.htm)

#### `public bool MouseCaptured(bool bIncludeMovement)`

Returns whether or not the mouse is captured in this view.

**Parameters:**
- `bIncludeMovement` (System.Boolean) — If captured, test if the mouse has moved between mouse button down and mouse button up.

**Returns:** `Boolean` — true if captured, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_MouseCaptured.htm)

#### `public void Redraw()`

Redraws this view.

**Remarks:** If you change something in "this" view like the projection, construction plane, background bitmap, etc., then you need to call RhinoView.Redraw() to redraw "this" view./ The other views will not be changed. If you change something in the document (like adding new geometry to the model), then you need to call RhinoDoc.Views.Redraw() to redraw all the views.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_Redraw.htm)

#### `public Point ScreenToClient(Point screenPoint)`

Converts a point in screen coordinates to client coordinates for this view.

**Parameters:**
- `screenPoint` (System.Drawing.Point) — The 2D screen point.

**Returns:** `Point` — A 2D point in client coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ScreenToClient_1.htm)

#### `public Point2d ScreenToClient(Point2d screenPoint)`

(Inherited from RhinoView.)

**Parameters:**
- `screenPoint` (Rhino.Geometry.Point2d) — [Missing <param name="screenPoint"/> documentation for "M:Rhino.Display.RhinoView.ScreenToClient(Rhino.Geometry.Point2d)"]

**Returns:** `Point2d` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.ScreenToClient(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ScreenToClient.htm)

#### `public bool SetActiveDetail(Guid detailId)`

Sets the active detail.

**Parameters:**
- `detailId` (System.Guid) — The id of the detail view object to set active.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_SetActiveDetail.htm)

#### `public bool SetActiveDetail(string detailName, bool compareCase)`

Sets the active detail.

**Parameters:**
- `detailName` (System.String) — The name, or title, of the detail to set active.
- `compareCase` (System.Boolean) — Unused.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_SetActiveDetail_1.htm)

#### `public void SetPageAsActive()`

Deactivates the active details and sets the page view as active.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoPageView_SetPageAsActive.htm)

#### `public double SpeedTest(int frameCount, bool freezeDrawList, int direction, double angleDeltaRadians)`

(Inherited from RhinoView.)

**Parameters:**
- `frameCount` (System.Int32) — [Missing <param name="frameCount"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]
- `freezeDrawList` (System.Boolean) — [Missing <param name="freezeDrawList"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]
- `direction` (System.Int32) — [Missing <param name="direction"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]
- `angleDeltaRadians` (System.Double) — [Missing <param name="angleDeltaRadians"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_SpeedTest.htm)

### Properties
- `ActiveDetail` (DetailViewObject, get) — Returns the active detail object. If no detail is active, or if the page is active, then null is returned.
- `ActiveDetailId` (Guid, get) — Returns the id of the active detail. If no detail is active, or if the page is active, then Guid.Empty is returned.
- `ActiveViewport` (RhinoViewport, get) — Gets the active viewport. The ActiveViewport is the same as the MainViewport for standard RhinoViews. In a RhinoPageView, the active viewport may be the RhinoViewport of a child detail object. Most of the time, you will use ActiveViewport unless you explicitly need to work with the main viewport.
- `ActiveViewportID` (Guid, get) — Returns viewport ID for the active viewport. Faster than ActiveViewport function when working with page views.
- `Bounds` (Rectangle, get) — Gets the size and location of the view including its non-client elements, in pixels, relative to the parent control.
- `ClientRectangle` (Rectangle, get) — Gets the rectangle that represents the client area of the view.
- `DisplayPipeline` (DisplayPipeline, get) — Gets the display pipeline used for this view.
- `Document` (RhinoDoc, get) — (Inherited from RhinoView.)
- `Floating` (Boolean, get/set) — Floating state of RhinoView. if true, then the view will be in a floating frame window. Otherwise the view will be embedded in the main frame.
- `Handle` (IntPtr, get) — Gets the window handle that this view is bound to.
- `MainViewport` (RhinoViewport, get) — A RhinoView contains a "main viewport" that fills the entire view client window. RhinoPageViews may also contain nested child RhinoViewports for implementing detail viewports. The MainViewport will always return this RhinoView's m_vp.
- `Maximized` (Boolean, get/set) — (Inherited from RhinoView.)
- `PageHeight` (Double, get/set) — Height of the page in the document's PageUnitSystem
- `PageIsActive` (Boolean, get) — Returns true if the page is active, rather than any detail view. This occurs when the MainViewport.Id == ActiveViewportID.
- `PageName` (String, get/set) — Same as the MainViewport.Name.
- `PageNumber` (Int32, get/set) — Gets or sets the runtime page number and updates the page number for all of the other pages. The first page has a value of 0.
- `PageWidth` (Double, get/set) — Width of the page in the document's PageUnitSystem
- `PaperName` (String, get) — Returns the name of the layout's media, or paper (e.g. Letter, Legal, A1, etc.), used to determine the page width and page height.
- `PrinterName` (String, get) — Returns the name of the layout's destination printer.
- `RealtimeDisplayMode` (RealtimeDisplayMode, get) — Gets the RealtimeDisplayMode active for this view. null if the view doesn't have a RealtimeDisplayMode set.
- `RuntimeSerialNumber` (UInt32, get) — (Inherited from RhinoView.)
- `ScreenRectangle` (Rectangle, get) — Gets the rectangle that represents the client area of the view in screen coordinates.
- `Size` (Size, get/set) — Gets or sets the size of the view
- `TitleVisible` (Boolean, get/set) — Visibility of the viewport title window.

### Events
#### `PageViewSpaceChange` (`System.EventHandler<PageViewSpaceChangeEventArgs>`)

**Signature:** `public static event EventHandler<PageViewSpaceChangeEventArgs> PageViewSpaceChange`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_RhinoPageView_PageViewSpaceChange.htm)

## RhinoView (class)

A RhinoView represents a single "window" display of a document. A view could contain one or many RhinoViewports (many in the case of Layout views with detail viewports). Standard Rhino modeling views have one viewport.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_RhinoView.htm)

### Methods
#### `public Bitmap CaptureToBitmap()`

Capture View contents to a bitmap.

**Returns:** `Bitmap` — The bitmap of the complete view.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap.htm)

#### `public Bitmap CaptureToBitmap(bool grid, bool worldAxes, bool cplaneAxes)`

Captures the view contents to a bitmap allowing for visibility of grid and axes.

**Parameters:**
- `grid` (System.Boolean) — true if the construction plane grid should be visible.
- `worldAxes` (System.Boolean) — true if the world axis should be visible.
- `cplaneAxes` (System.Boolean) — true if the construction plane close the grid should be visible.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_3.htm)

#### `public Bitmap CaptureToBitmap(DisplayModeDescription mode)`

Capture View contents to a bitmap using a display mode description to define how drawing is performed.

**Parameters:**
- `mode` (Rhino.Display.DisplayModeDescription) — The display mode.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_1.htm)

#### `public Bitmap CaptureToBitmap(DisplayPipelineAttributes attributes)`

Captures view contents to a bitmap using display attributes to define how drawing is performed.

**Parameters:**
- `attributes` (Rhino.Display.DisplayPipelineAttributes) — The specific display mode attributes.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_2.htm)

#### `public Bitmap CaptureToBitmap(Size size)`

Capture View contents to a bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Size of Bitmap to capture to.

**Returns:** `Bitmap` — The bitmap of the specified part of the view.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_4.htm)

#### `public Bitmap CaptureToBitmap(Size size, bool grid, bool worldAxes, bool cplaneAxes)`

Captures a part of the view contents to a bitmap allowing for visibility of grid and axes.

**Parameters:**
- `size` (System.Drawing.Size) — The width and height of the returned bitmap.
- `grid` (System.Boolean) — true if the construction plane grid should be visible.
- `worldAxes` (System.Boolean) — true if the world axis should be visible.
- `cplaneAxes` (System.Boolean) — true if the construction plane close the grid should be visible.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_7.htm)

#### `public Bitmap CaptureToBitmap(Size size, DisplayModeDescription mode)`

Capture View contents to a bitmap using a display mode description to define how drawing is performed.

**Parameters:**
- `size` (System.Drawing.Size) — The width and height of the returned bitmap.
- `mode` (Rhino.Display.DisplayModeDescription) — The display mode.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_5.htm)

#### `public Bitmap CaptureToBitmap(Size size, DisplayPipelineAttributes attributes)`

Capture View contents to a bitmap using display attributes to define how drawing is performed.

**Parameters:**
- `size` (System.Drawing.Size) — The width and height of the returned bitmap.
- `attributes` (Rhino.Display.DisplayPipelineAttributes) — The specific display mode attributes.

**Returns:** `Bitmap` — A new bitmap.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CaptureToBitmap_6.htm)

#### `public Point ClientToScreen(Point clientPoint)`



**Parameters:**
- `clientPoint` (System.Drawing.Point) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(System.Drawing.Point)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ClientToScreen_1.htm)

#### `public Point2d ClientToScreen(Point2d clientPoint)`



**Parameters:**
- `clientPoint` (Rhino.Geometry.Point2d) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(Rhino.Geometry.Point2d)"]

**Returns:** `Point2d` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.ClientToScreen(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ClientToScreen.htm)

#### `public bool Close()`

Remove this View from Rhino. DO NOT attempt to use this instance of this class after calling Close.

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_Close.htm)

#### `public bool CreateShadedPreviewImage(string imagePath, Size size, bool ignoreHighlights, bool drawConstructionPlane, bool useGhostedShading)`

Creates a bitmap preview image of model.

**Parameters:**
- `imagePath` (System.String) — [in] The name of the bitmap file to create. The extension of the imagePath controls the format of the bitmap file created (BMP, TGA, JPG, PCX, PNG, TIF).
- `size` (System.Drawing.Size) — [in] The width and height of the bitmap in pixels.
- `ignoreHighlights` (System.Boolean) — true if highlighted elements should be drawn normally.
- `drawConstructionPlane` (System.Boolean) — true if the CPlane should be drawn.
- `useGhostedShading` (System.Boolean) — true if ghosted shading (partially transparent shading) should be used.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CreateShadedPreviewImage.htm)

#### `public bool CreateWireframePreviewImage(string imagePath, Size size, bool ignoreHighlights, bool drawConstructionPlane)`

Creates a bitmap preview image of model.

**Parameters:**
- `imagePath` (System.String) — [in] The name of the bitmap file to create. The extension of the imagePath controls the format of the bitmap file created (BMP, TGA, JPG, PCX, PNG, TIF).
- `size` (System.Drawing.Size) — [in] The width and height of the bitmap in pixels.
- `ignoreHighlights` (System.Boolean) — true if highlighted elements should be drawn normally.
- `drawConstructionPlane` (System.Boolean) — true if the CPlane should be drawn.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_CreateWireframePreviewImage.htm)

#### `public override bool Equals(Object obj)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `obj` (System.Object) — [Missing <param name="obj"/> documentation for "M:Rhino.Display.RhinoView.Equals(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.Equals(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_Equals.htm)

#### `public static RhinoView FromRuntimeSerialNumber(uint serialNumber)`

Get a RhinoView from it's unique runtime serial number

**Parameters:**
- `serialNumber` (System.UInt32) — [Missing <param name="serialNumber"/> documentation for "M:Rhino.Display.RhinoView.FromRuntimeSerialNumber(System.UInt32)"]

**Returns:** `RhinoView` — RhinoView or null if no view exists for a given serial number

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_FromRuntimeSerialNumber.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_GetHashCode.htm)

#### `public bool MouseCaptured(bool bIncludeMovement)`

Returns whether or not the mouse is captured in this view.

**Parameters:**
- `bIncludeMovement` (System.Boolean) — If captured, test if the mouse has moved between mouse button down and mouse button up.

**Returns:** `Boolean` — true if captured, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_MouseCaptured.htm)

#### `public void Redraw()`

Redraws this view.

**Remarks:** If you change something in "this" view like the projection, construction plane, background bitmap, etc., then you need to call RhinoView.Redraw() to redraw "this" view./ The other views will not be changed. If you change something in the document (like adding new geometry to the model), then you need to call RhinoDoc.Views.Redraw() to redraw all the views.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_Redraw.htm)

#### `public Point ScreenToClient(Point screenPoint)`

Converts a point in screen coordinates to client coordinates for this view.

**Parameters:**
- `screenPoint` (System.Drawing.Point) — The 2D screen point.

**Returns:** `Point` — A 2D point in client coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ScreenToClient_1.htm)

#### `public Point2d ScreenToClient(Point2d screenPoint)`



**Parameters:**
- `screenPoint` (Rhino.Geometry.Point2d) — [Missing <param name="screenPoint"/> documentation for "M:Rhino.Display.RhinoView.ScreenToClient(Rhino.Geometry.Point2d)"]

**Returns:** `Point2d` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.ScreenToClient(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_ScreenToClient.htm)

#### `public double SpeedTest(int frameCount, bool freezeDrawList, int direction, double angleDeltaRadians)`



**Parameters:**
- `frameCount` (System.Int32) — [Missing <param name="frameCount"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]
- `freezeDrawList` (System.Boolean) — [Missing <param name="freezeDrawList"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]
- `direction` (System.Int32) — [Missing <param name="direction"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]
- `angleDeltaRadians` (System.Double) — [Missing <param name="angleDeltaRadians"/> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Display.RhinoView.SpeedTest(System.Int32,System.Boolean,System.Int32,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoView_SpeedTest.htm)

### Properties
- `ActiveViewport` (RhinoViewport, get) — The ActiveViewport is the same as the MainViewport for standard RhinoViews. In a RhinoPageView, the active viewport may be the RhinoViewport of a child detail object. Most of the time, you will use ActiveViewport unless you explicitly need to work with the main viewport.
- `ActiveViewportID` (Guid, get) — Returns viewport ID for the active viewport. Faster than ActiveViewport function when working with page views.
- `Bounds` (Rectangle, get) — Gets the size and location of the view including its non-client elements, in pixels, relative to the parent control.
- `ClientRectangle` (Rectangle, get) — Gets the rectangle that represents the client area of the view.
- `DisplayPipeline` (DisplayPipeline, get) — Gets the display pipeline used for this view.
- `Document` (RhinoDoc, get) — 
- `EnableDrawing` (Boolean, get/set) — Gets or sets the 'drawing enabled' flag. By default, drawing is enabled. There are some rare situations where scripts want to disable drawing for a while.
- `Floating` (Boolean, get/set) — Floating state of RhinoView. if true, then the view will be in a floating frame window. Otherwise the view will be embedded in the main frame.
- `Handle` (IntPtr, get) — Gets the window handle that this view is bound to.
- `MainViewport` (RhinoViewport, get) — A RhinoView contains a "main viewport" that fills the entire view client window. RhinoPageViews may also contain nested child RhinoViewports for implementing detail viewports. The MainViewport will always return this RhinoView's m_vp.
- `Maximized` (Boolean, get/set) — 
- `RealtimeDisplayMode` (RealtimeDisplayMode, get) — Gets the RealtimeDisplayMode active for this view. null if the view doesn't have a RealtimeDisplayMode set.
- `RuntimeSerialNumber` (UInt32, get) — 
- `ScreenRectangle` (Rectangle, get) — Gets the rectangle that represents the client area of the view in screen coordinates.
- `Size` (Size, get/set) — Gets or sets the size of the view
- `TitleVisible` (Boolean, get/set) — Visibility of the viewport title window.

### Events
#### `Create` (`System.EventHandler<ViewEventArgs>`)

**Signature:** `public static event EventHandler<ViewEventArgs> Create`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_RhinoView_Create.htm)

#### `Destroy` (`System.EventHandler<ViewEventArgs>`)

**Signature:** `public static event EventHandler<ViewEventArgs> Destroy`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_RhinoView_Destroy.htm)

#### `Modified` (`System.EventHandler<ViewEventArgs>`)

**Signature:** `public static event EventHandler<ViewEventArgs> Modified`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_RhinoView_Modified.htm)

#### `Rename` (`System.EventHandler<ViewEventArgs>`)

**Signature:** `public static event EventHandler<ViewEventArgs> Rename`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_RhinoView_Rename.htm)

#### `SetActive` (`System.EventHandler<ViewEventArgs>`)

**Signature:** `public static event EventHandler<ViewEventArgs> SetActive`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Display_RhinoView_SetActive.htm)

## RhinoViewport (class)

Displays geometry with a given projection. In standard modeling views there is a one to one relationship between RhinoView and RhinoViewports. In a page layout, there may be multiple RhinoViewports for a single layout.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_RhinoViewport.htm)

### Constructors
- `public RhinoViewport()` — Initializes a new instance of the RhinoViewport class
- `public RhinoViewport(RhinoViewport other)` — Initializes a new instance of the RhinoViewport class

### Methods
#### `public bool ChangeToParallelProjection(bool symmetricFrustum)`

Use this function to change projections of valid viewports from perspective to parallel. It will make common additional adjustments to the frustum so the resulting views are similar. The camera location and direction will not be changed.

**Parameters:**
- `symmetricFrustum` (System.Boolean) — true if you want the resulting frustum to be symmetric.

**Returns:** `Boolean` — If the current projection is parallel and bSymmetricFrustum, FrustumIsLeftRightSymmetric() and FrustumIsTopBottomSymmetric() are all equal, then no changes are made and true is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ChangeToParallelProjection.htm)

#### `public bool ChangeToPerspectiveProjection(bool symmetricFrustum, double lensLength)`

Use this function to change projections of valid viewports from parallel to perspective. It will make common additional adjustments to the frustum and camera location so the resulting views are similar. The camera direction and target point are not be changed.

**Parameters:**
- `symmetricFrustum` (System.Boolean) — true if you want the resulting frustum to be symmetric.
- `lensLength` (System.Double) — (pass 50.0 when in doubt) 35 mm lens length to use when changing from parallel to perspective projections. If the current projection is perspective or lens_length is <= 0.0, then this parameter is ignored.

**Returns:** `Boolean` — If the current projection is perspective and bSymmetricFrustum, FrustumIsLeftRightSymmetric() and FrustumIsTopBottomSymmetric() are all equal, then no changes are made and true is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ChangeToPerspectiveProjection.htm)

#### `public bool ChangeToPerspectiveProjection(double targetDistance, bool symmetricFrustum, double lensLength)`

Use this function to change projections of valid viewports from parallel to perspective. It will make common additional adjustments to the frustum and camera location so the resulting views are similar. The camera direction and target point are not be changed.

**Parameters:**
- `targetDistance` (System.Double) — If RhinoMath.UnsetValue this parameter is ignored. Otherwise it must be > 0 and indicates which plane in the current view frustum should be preserved.
- `symmetricFrustum` (System.Boolean) — true if you want the resulting frustum to be symmetric.
- `lensLength` (System.Double) — (pass 50.0 when in doubt) 35 mm lens length to use when changing from parallel to perspective projections. If the current projection is perspective or lens_length is <= 0.0, then this parameter is ignored.

**Returns:** `Boolean` — If the current projection is perspective and bSymmetricFrustum, FrustumIsLeftRightSymmetric() and FrustumIsTopBottomSymmetric() are all equal, then no changes are made and true is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ChangeToPerspectiveProjection_1.htm)

#### `public bool ChangeToTwoPointPerspectiveProjection(double lensLength)`

Use this function to change projections of valid viewports to a two point perspective. It will make common additional adjustments to the frustum and camera location and direction so the resulting views are similar.

**Parameters:**
- `lensLength` (System.Double) — (pass 50.0 when in doubt) 35 mm lens length to use when changing from parallel to perspective projections. If the current projection is perspective or lens_length is <= 0.0, then this parameter is ignored.

**Returns:** `Boolean` — If the current projection is perspective and bSymmetricFrustum, FrustumIsLeftRightSymmetric() and FrustumIsTopBottomSymmetric() are all equal, then no changes are made and true is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ChangeToTwoPointPerspectiveProjection.htm)

#### `public bool ChangeToTwoPointPerspectiveProjection(double targetDistance, Vector3d up, double lensLength)`

Use this function to change projections of valid viewports to a two point perspective. It will make common additional adjustments to the frustum and camera location and direction so the resulting views are similar.

**Parameters:**
- `targetDistance` (System.Double) — If RhinoMath.UnsetValue this parameter is ignored. Otherwise it must be > 0 and indicates which plane in the current view frustum should be preserved.
- `up` (Rhino.Geometry.Vector3d) — This direction will be the locked up direction. Pass ON_3dVector::ZeroVector if you want to use the world axis direction that is closest to the current up direction. Pass CameraY() if you want to preserve the current up direction.
- `lensLength` (System.Double) — (pass 50.0 when in doubt) 35 mm lens length to use when changing from parallel to perspective projections. If the current projection is perspective or lens_length is <= 0.0, then this parameter is ignored.

**Returns:** `Boolean` — If the current projection is perspective and bSymmetricFrustum, FrustumIsLeftRightSymmetric() and FrustumIsTopBottomSymmetric() are all equal, then no changes are made and true is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ChangeToTwoPointPerspectiveProjection_1.htm)

#### `public void ClearTraceImage()`

Remove trace image (background bitmap) for this viewport if one exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ClearTraceImage.htm)

#### `public Point ClientToScreen(Point clientPoint)`



**Parameters:**
- `clientPoint` (System.Drawing.Point) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoViewport.ClientToScreen(System.Drawing.Point)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.ClientToScreen(System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ClientToScreen_1.htm)

#### `public Point ClientToScreen(Point2d clientPoint)`



**Parameters:**
- `clientPoint` (Rhino.Geometry.Point2d) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoViewport.ClientToScreen(Rhino.Geometry.Point2d)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.ClientToScreen(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ClientToScreen.htm)

#### `public Line ClientToWorld(Point clientPoint)`



**Parameters:**
- `clientPoint` (System.Drawing.Point) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoViewport.ClientToWorld(System.Drawing.Point)"]

**Returns:** `Line` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.ClientToWorld(System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ClientToWorld_1.htm)

#### `public Line ClientToWorld(Point2d clientPoint)`



**Parameters:**
- `clientPoint` (Rhino.Geometry.Point2d) — [Missing <param name="clientPoint"/> documentation for "M:Rhino.Display.RhinoViewport.ClientToWorld(Rhino.Geometry.Point2d)"]

**Returns:** `Line` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.ClientToWorld(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ClientToWorld.htm)

#### `public Plane ConstructionPlane()`

Simple plane information for this viewport's construction plane. If you want detailed construction plane information, use GetConstructionPlane.

**Returns:** `Plane` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.ConstructionPlane"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ConstructionPlane.htm)

#### `public void DeleteAllUserStrings()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_DeleteAllUserStrings.htm)

#### `public bool DeleteUserString(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.Display.RhinoViewport.DeleteUserString(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.DeleteUserString(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_DeleteUserString.htm)

#### `public void Dispose()`

Releases all resources used by the RhinoViewport

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the RhinoViewport and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_Finalize.htm)

#### `public static RhinoViewport FromId(Guid id)`

Call this method to get the viewport with the specified Id.

**Parameters:**
- `id` (System.Guid) — Id to search for.

**Returns:** `RhinoViewport` — Returns a RhinoViewport if the Id is found otherwise null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_FromId.htm)

#### `public bool GetCameraAngle(out double halfDiagonalAngle, out double halfVerticalAngle, out double halfHorizontalAngle)`



**Parameters:**
- `halfDiagonalAngle` (System.Double) — [Missing <param name="halfDiagonalAngle"/> documentation for "M:Rhino.Display.RhinoViewport.GetCameraAngle(System.Double@,System.Double@,System.Double@)"]
- `halfVerticalAngle` (System.Double) — [Missing <param name="halfVerticalAngle"/> documentation for "M:Rhino.Display.RhinoViewport.GetCameraAngle(System.Double@,System.Double@,System.Double@)"]
- `halfHorizontalAngle` (System.Double) — [Missing <param name="halfHorizontalAngle"/> documentation for "M:Rhino.Display.RhinoViewport.GetCameraAngle(System.Double@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.GetCameraAngle(System.Double@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetCameraAngle.htm)

#### `public BoundingBox GetCameraExtents(IEnumerable<Point3d> points)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.Display.RhinoViewport.GetCameraExtents(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

**Returns:** `BoundingBox` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.GetCameraExtents(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetCameraExtents.htm)

#### `public bool GetCameraFrame(out Plane frame)`

Gets the camera plane.

**Parameters:**
- `frame` (Rhino.Geometry.Plane) — A plane is assigned to this out parameter during the call, if the operation succeeded.

**Returns:** `Boolean` — true if current camera orientation is valid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetCameraFrame.htm)

#### `public ConstructionPlane GetConstructionPlane()`



**Returns:** `ConstructionPlane` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.GetConstructionPlane"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetConstructionPlane.htm)

#### `public bool GetDepth(BoundingBox bbox, out double nearDistance, out double farDistance)`

Gets near and far clipping distances of a bounding box.

**Parameters:**
- `bbox` (Rhino.Geometry.BoundingBox) — The bounding box.
- `nearDistance` (System.Double) — The near distance is assigned to this out parameter during this call.
- `farDistance` (System.Double) — The far distance is assigned to this out parameter during this call.

**Returns:** `Boolean` — true if the bounding box intersects the view frustum and near_dist/far_dist were set. false if the bounding box does not intersect the view frustum.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetDepth.htm)

#### `public bool GetDepth(Point3d point, out double distance)`

Gets clipping distance of a point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A 3D point.
- `distance` (System.Double) — A computed distance is assigned to this out parameter if this call succeeds.

**Returns:** `Boolean` — true if the point is in the view frustum and near_dist/far_dist were set. false if the bounding box does not intersect the view frustum.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetDepth_1.htm)

#### `public bool GetDepth(Sphere sphere, out double nearDistance, out double farDistance)`

Gets near and far clipping distances of a sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — The sphere.
- `nearDistance` (System.Double) — The near distance is assigned to this out parameter during this call.
- `farDistance` (System.Double) — The far distance is assigned to this out parameter during this call.

**Returns:** `Boolean` — true if the sphere intersects the view frustum and near_dist/far_dist were set. false if the sphere does not intersect the view frustum.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetDepth_2.htm)

#### `public Point3d[] GetFarRect()`

Get corners of far clipping plane rectangle.

**Returns:** `Point3d[]` — [left_bottom, right_bottom, left_top, right_top] points on success null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFarRect.htm)

#### `public bool GetFrustum(out double left, out double right, out double bottom, out double top, out double nearDistance, out double farDistance)`

Gets the view frustum.

**Parameters:**
- `left` (System.Double) — left < right.
- `right` (System.Double) — left < right.
- `bottom` (System.Double) — bottom < top.
- `top` (System.Double) — bottom < top.
- `nearDistance` (System.Double) — 0 < nearDistance < farDistance.
- `farDistance` (System.Double) — 0 < nearDistance < farDistance.

**Returns:** `Boolean` — true if operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustum.htm)

#### `public bool GetFrustumBottomPlane(out Plane plane)`

Get bottom world frustum clipping plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — frustum bottom side clipping plane. The normal points into the visible region of the frustum. If the projection is perspective, the origin is at the camera location, otherwise the origin is the point on the plane that is closest to the camera location.

**Returns:** `Boolean` — true if camera and frustum are valid and plane was set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumBottomPlane.htm)

#### `public BoundingBox GetFrustumBoundingBox()`



**Returns:** `BoundingBox` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.GetFrustumBoundingBox"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumBoundingBox.htm)

#### `public bool GetFrustumCenter(out Point3d center)`

Returns world coordinates of frustum's center.

**Parameters:**
- `center` (Rhino.Geometry.Point3d) — The center coordinate is assigned to this out parameter if this call succeeds.

**Returns:** `Boolean` — true if the center was successfully computed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumCenter.htm)

#### `public bool GetFrustumFarPlane(out Plane plane)`

Get far clipping plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — far clipping plane if camera and frustum are valid. The plane's frame is the same as the camera's frame. The origin is located at the intersection of the camera direction ray and the far clipping plane.

**Returns:** `Boolean` — true if camera and frustum are valid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumFarPlane.htm)

#### `public bool GetFrustumLeftPlane(out Plane plane)`

Get left world frustum clipping plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — frustum left side clipping plane. The normal points into the visible region of the frustum. If the projection is perspective, the origin is at the camera location, otherwise the origin is the point on the plane that is closest to the camera location.

**Returns:** `Boolean` — true if camera and frustum are valid and plane was set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumLeftPlane.htm)

#### `public bool GetFrustumLine(double screenX, double screenY, out Line worldLine)`

Gets the world coordinate line in the view frustum that projects to a point on the screen.

**Parameters:**
- `screenX` (System.Double) — (screenx,screeny) = screen location.
- `screenY` (System.Double) — (screenx,screeny) = screen location.
- `worldLine` (Rhino.Geometry.Line) — 3d world coordinate line segment starting on the near clipping plane and ending on the far clipping plane.

**Returns:** `Boolean` — true if successful. false if view projection or frustum is invalid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumLine.htm)

#### `public bool GetFrustumNearPlane(out Plane plane)`

Get near clipping plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — near clipping plane if camera and frustum are valid. The plane's frame is the same as the camera's frame. The origin is located at the intersection of the camera direction ray and the near clipping plane.

**Returns:** `Boolean` — true if camera and frustum are valid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumNearPlane.htm)

#### `public bool GetFrustumRightPlane(out Plane plane)`

Get right world frustum clipping plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — frustum right side clipping plane. The normal points into the visible region of the frustum. If the projection is perspective, the origin is at the camera location, otherwise the origin is the point on the plane that is closest to the camera location.

**Returns:** `Boolean` — true if camera and frustum are valid and plane was set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumRightPlane.htm)

#### `public bool GetFrustumTopPlane(out Plane plane)`

Get top world frustum clipping plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — frustum top side clipping plane. The normal points into the visible region of the frustum. If the projection is perspective, the origin is at the camera location, otherwise the origin is the point on the plane that is closest to the camera location.

**Returns:** `Boolean` — true if camera and frustum are valid and plane was set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetFrustumTopPlane.htm)

#### `public Point3d[] GetNearRect()`

Get corners of near clipping plane rectangle.

**Returns:** `Point3d[]` — [left_bottom, right_bottom, left_top, right_top] points on success null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetNearRect.htm)

#### `public Transform GetPickTransform(int clientX, int clientY)`

Takes a rectangle in screen coordinates and returns a transformation that maps the 3d frustum defined by the rectangle to a -1/+1 clipping coordinate box. This takes a single point and inflates it by Rhino.ApplicationSettings.ModelAidSettings.MousePickBoxRadius to define the screen rectangle.

**Parameters:**
- `clientX` (System.Int32) — The client point X coordinate.
- `clientY` (System.Int32) — The client point Y coordinate.

**Returns:** `Transform` — A transformation matrix.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetPickTransform_2.htm)

#### `public Transform GetPickTransform(Point clientPoint)`

Takes a rectangle in screen coordinates and returns a transformation that maps the 3d frustum defined by the rectangle to a -1/+1 clipping coordinate box. This takes a single point and inflates it by Rhino.ApplicationSettings.ModelAidSettings.MousePickBoxRadius to define the screen rectangle.

**Parameters:**
- `clientPoint` (System.Drawing.Point) — The client point.

**Returns:** `Transform` — A transformation matrix.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetPickTransform.htm)

#### `public Transform GetPickTransform(Rectangle clientRectangle)`

Takes a rectangle in screen coordinates and returns a transformation that maps the 3d frustum defined by the rectangle to a -1/+1 clipping coordinate box.

**Parameters:**
- `clientRectangle` (System.Drawing.Rectangle) — The client rectangle.

**Returns:** `Transform` — A transformation matrix.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetPickTransform_1.htm)

#### `public bool GetScreenPort(out int portLeft, out int portRight, out int portBottom, out int portTop, out int portNear, out int portFar)`

Location of viewport in pixels. These are provided so you can set the port you are using and get the appropriate transformations to and from screen space.

**Parameters:**
- `portLeft` (System.Int32) — portLeft != portRight.
- `portRight` (System.Int32) — portLeft != portRight.
- `portBottom` (System.Int32) — portTop != portBottom.
- `portTop` (System.Int32) — portTop != portBottom.
- `portNear` (System.Int32) — The viewport near value.
- `portFar` (System.Int32) — The viewport far value.

**Returns:** `Boolean` — true if the operation is successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetScreenPort.htm)

#### `public Transform GetTransform(CoordinateSystem sourceSystem, CoordinateSystem destinationSystem)`

Gets a transform from origin coordinate system to a target coordinate system.

**Parameters:**
- `sourceSystem` (Rhino.DocObjects.CoordinateSystem) — The origin coordinate system.
- `destinationSystem` (Rhino.DocObjects.CoordinateSystem) — The target coordinate system.

**Returns:** `Transform` — 4x4 transformation matrix (acts on the left) Identity matrix is returned if this function fails.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetTransform.htm)

#### `public string GetUserString(string key)`

Gets a user string.

**Parameters:**
- `key` (System.String) — id used to retrieve the string.

**Returns:** `String` — string associated with the key if successful. null if no key was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetUserString.htm)

#### `public NameValueCollection GetUserStrings()`

Gets an independent copy of the collection of (user text key, user text value) pairs attached to this object.

**Returns:** `NameValueCollection` — A collection of key strings and values strings. This

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetUserStrings.htm)

#### `public bool GetWorldToScreenScale(Point3d pointInFrustum, out double pixelsPerUnit)`

Gets the world to screen size scaling factor at a point in frustum.

**Parameters:**
- `pointInFrustum` (Rhino.Geometry.Point3d) — A point in frustum.
- `pixelsPerUnit` (System.Double) — scale = number of pixels per world unit at the 3d point. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if the operation is successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_GetWorldToScreenScale.htm)

#### `public bool IsVisible(BoundingBox bbox)`

Returns true if some portion of a world coordinate bounding box is potentially visible in the viewing frustum.

**Parameters:**
- `bbox` (Rhino.Geometry.BoundingBox) — A bounding box that is tested for visibility.

**Returns:** `Boolean` — true if the box is potentially visible; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_IsVisible.htm)

#### `public bool IsVisible(Point3d point)`

Determines if a world coordinate point is visible in the viewing frustum.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that is tested for visibility.

**Returns:** `Boolean` — true if the point is visible; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_IsVisible_1.htm)

#### `public bool KeyboardDolly(bool leftRight, double amount)`

Emulates the keyboard arrow key in terms of interaction.

**Parameters:**
- `leftRight` (System.Boolean) — left/right dolly if true, up/down dolly if false.
- `amount` (System.Double) — The dolly amount.

**Returns:** `Boolean` — true if operation succeeded; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_KeyboardDolly.htm)

#### `public bool KeyboardDollyInOut(double amount)`

Emulates the keyboard arrow key in terms of interaction.

**Parameters:**
- `amount` (System.Double) — The dolly amount.

**Returns:** `Boolean` — true if operation succeeded; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_KeyboardDollyInOut.htm)

#### `public bool KeyboardRotate(bool leftRight, double angleRadians)`

Emulates the keyboard arrow key in terms of interaction.

**Parameters:**
- `leftRight` (System.Boolean) — left/right rotate if true, up/down rotate if false.
- `angleRadians` (System.Double) — If less than 0, rotation is to left or down. If greater than 0, rotation is to right or up.

**Returns:** `Boolean` — true if operation succeeded; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_KeyboardRotate.htm)

#### `public bool Magnify(double magnificationFactor, bool mode)`

Zooms or dollies in order to scale the viewport projection of observed objects.

**Parameters:**
- `magnificationFactor` (System.Double) — The scale factor.
- `mode` (System.Boolean) — false = perform a "dolly" magnification by moving the camera towards/away from the target so that the amount of the screen subtended by an object changes. true = perform a "zoom" magnification by adjusting the "lens" angle

**Returns:** `Boolean` — true if operation succeeded; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_Magnify.htm)

#### `public bool Magnify(double magnificationFactor, bool mode, Point fixedScreenPoint)`

Zooms or dollies in order to scale the viewport projection of observed objects.

**Parameters:**
- `magnificationFactor` (System.Double) — The scale factor.
- `mode` (System.Boolean) — false = perform a "dolly" magnification by moving the camera towards/away from the target so that the amount of the screen subtended by an object changes. true = perform a "zoom" magnification by adjusting the "lens" angle
- `fixedScreenPoint` (System.Drawing.Point) — A point in the screen that should remain fixed.

**Returns:** `Boolean` — true if operation succeeded; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_Magnify_1.htm)

#### `public bool MouseAdjustLensLength(Point mousePreviousPoint, Point mouseCurrentPoint, bool moveTarget)`

Adjusts the camera lens length.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.
- `moveTarget` (System.Boolean) — Should this operation move the target?

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseAdjustLensLength(System.Drawing.Point,System.Drawing.Point,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseAdjustLensLength.htm)

#### `public bool MouseDollyZoom(Point mousePreviousPoint, Point mouseCurrentPoint)`

Zooms lens (thus adjusting the field of view) while moving the camera.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseDollyZoom(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseDollyZoom.htm)

#### `public bool MouseInOutDolly(Point mousePreviousPoint, Point mouseCurrentPoint)`

Moves the camera towards or away from the view maintaining focus on the view.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseInOutDolly(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseInOutDolly.htm)

#### `public bool MouseLateralDolly(Point mousePreviousPoint, Point mouseCurrentPoint)`

Pans the camera

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseLateralDolly(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseLateralDolly.htm)

#### `public bool MouseMagnify(Point mousePreviousPoint, Point mouseCurrentPoint)`

Moves the camera towards or away from the view.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseMagnify(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseMagnify.htm)

#### `public bool MouseRotateAroundTarget(Point mousePreviousPoint, Point mouseCurrentPoint)`

Rotates the viewport around target.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseRotateAroundTarget(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseRotateAroundTarget.htm)

#### `public bool MouseRotateCamera(Point mousePreviousPoint, Point mouseCurrentPoint)`

Rotates the view around the camera location.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseRotateCamera(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseRotateCamera.htm)

#### `public bool MouseTilt(Point mousePreviousPoint, Point mouseCurrentPoint)`

Tilts the camera view.

**Parameters:**
- `mousePreviousPoint` (System.Drawing.Point) — The mouse previous point.
- `mouseCurrentPoint` (System.Drawing.Point) — The mouse current point.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.MouseTilt(System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_MouseTilt.htm)

#### `public bool NextConstructionPlane()`

Sets the construction plane to the plane that was active before the last call to PreviousConstructionPlane.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_NextConstructionPlane.htm)

#### `public bool NextViewProjection()`

Sets the view projection and target to the settings that were active before the last call to PrevView.

**Returns:** `Boolean` — true if the view stack was popped.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_NextViewProjection.htm)

#### `public bool PopConstructionPlane()`

Sets the construction plane to the plane that was active before the last call to PushConstructionPlane.

**Returns:** `Boolean` — true if a construction plane was popped.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PopConstructionPlane.htm)

#### `public bool PopViewProjection()`

Sets the view projection and target to the settings at the top of the view stack and removes those settings from the view stack.

**Returns:** `Boolean` — true if there were settings that could be popped from the stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PopViewProjection.htm)

#### `public bool PreviousConstructionPlane()`

Sets the construction plane to the plane that was active before the last call to NextConstructionPlane or SetConstructionPlane.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PreviousConstructionPlane.htm)

#### `public bool PreviousViewProjection()`

Sets the view projection and target to the settings that were active before the last call to NextViewProjection.

**Returns:** `Boolean` — true if the view stack was popped.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PreviousViewProjection.htm)

#### `public void PushConstructionPlane(ConstructionPlane cplane)`

Pushes the current construction plane on the viewport's construction plane stack and sets the construction plane to cplane.

**Parameters:**
- `cplane` (Rhino.DocObjects.ConstructionPlane) — The construction plane to push.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PushConstructionPlane.htm)

#### `public bool PushViewInfo(ViewInfo viewinfo, bool includeTraceImage)`



**Parameters:**
- `viewinfo` (Rhino.DocObjects.ViewInfo) — [Missing <param name="viewinfo"/> documentation for "M:Rhino.Display.RhinoViewport.PushViewInfo(Rhino.DocObjects.ViewInfo,System.Boolean)"]
- `includeTraceImage` (System.Boolean) — [Missing <param name="includeTraceImage"/> documentation for "M:Rhino.Display.RhinoViewport.PushViewInfo(Rhino.DocObjects.ViewInfo,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.PushViewInfo(Rhino.DocObjects.ViewInfo,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PushViewInfo.htm)

#### `public void PushViewProjection()`

Appends the current view projection and target to the viewport's view stack.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_PushViewProjection.htm)

#### `public bool Rotate(double angleRadians, Vector3d rotationAxis, Point3d rotationCenter)`

Rotates about the specified axis. A positive rotation angle results in a counter-clockwise rotation about the axis (right hand rule).

**Parameters:**
- `angleRadians` (System.Double) — angle of rotation in radians.
- `rotationAxis` (Rhino.Geometry.Vector3d) — direction of the axis of rotation.
- `rotationCenter` (Rhino.Geometry.Point3d) — point on the axis of rotation.

**Returns:** `Boolean` — true if geometry successfully rotated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_Rotate.htm)

#### `public Point ScreenToClient(Point screenPoint)`



**Parameters:**
- `screenPoint` (System.Drawing.Point) — [Missing <param name="screenPoint"/> documentation for "M:Rhino.Display.RhinoViewport.ScreenToClient(System.Drawing.Point)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.ScreenToClient(System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ScreenToClient.htm)

#### `public void SetCameraDirection(Vector3d cameraDirection, bool updateTargetLocation)`

Set viewport camera direction. By default the target location is changed so that the vector from the camera location to the target is parallel to the camera direction.

**Parameters:**
- `cameraDirection` (Rhino.Geometry.Vector3d) — new camera direction.
- `updateTargetLocation` (System.Boolean) — if true, the target location is changed so that the vector from the camera location to the target is parallel to the camera direction. If false, the target location is not changed. See the remarks section of RhinoViewport.SetTarget for important details.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetCameraDirection.htm)

#### `public void SetCameraLocation(Point3d cameraLocation, bool updateTargetLocation)`

Set viewport camera location. By default the target location is changed so that the vector from the camera location to the target is parallel to the camera direction vector.

**Parameters:**
- `cameraLocation` (Rhino.Geometry.Point3d) — new camera location.
- `updateTargetLocation` (System.Boolean) — if true, the target location is changed so that the vector from the camera location to the target is parallel to the camera direction vector. If false, the target location is not changed. See the remarks section of RhinoViewport.SetTarget for important details.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetCameraLocation.htm)

#### `public void SetCameraLocations(Point3d targetLocation, Point3d cameraLocation)`

Set viewport camera location and target location. The camera direction vector is changed so that it is parallel to the vector from the camera location to the target location.

**Parameters:**
- `targetLocation` (Rhino.Geometry.Point3d) — new target location.
- `cameraLocation` (Rhino.Geometry.Point3d) — new camera location.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetCameraLocations.htm)

#### `public void SetCameraTarget(Point3d targetLocation, bool updateCameraLocation)`

Set viewport target point. By default the camera location is translated so that the camera direction vector is parallel to the vector from the camera location to the target location.

**Remarks:** In general, Rhino users expect to have the camera direction vector and the vector from the camera location to the target location to be parallel. If you use the RhinoViewport functions to set the camera location, camera direction, and target point, then the relationship between these three points and vectors is automatically maintained. If you directly manipulate the camera properties, then you should carefully set the target by calling SetTarget() with updateCameraLocation=false.

**Parameters:**
- `targetLocation` (Rhino.Geometry.Point3d) — new target location.
- `updateCameraLocation` (System.Boolean) — if true, the camera location is translated so that the camera direction vector is parallel to the vector from the camera location to the target location. If false, the camera location is not changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetCameraTarget.htm)

#### `public void SetClippingPlanes(BoundingBox box)`

Sets optimal clipping planes to view objects in a world coordinate 3d bounding box.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — The bounding box

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetClippingPlanes.htm)

#### `public void SetConstructionPlane(ConstructionPlane cplane)`

Sets the construction plane to cplane.

**Parameters:**
- `cplane` (Rhino.DocObjects.ConstructionPlane) — The construction plane to set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetConstructionPlane.htm)

#### `public void SetConstructionPlane(Plane plane)`



**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Display.RhinoViewport.SetConstructionPlane(Rhino.Geometry.Plane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetConstructionPlane_1.htm)

#### `public bool SetProjection(DefinedViewportProjection projection, string viewName, bool updateConstructionPlane)`

Set viewport to a defined projection.

**Parameters:**
- `projection` (Rhino.Display.DefinedViewportProjection) — The "standard" projection type.
- `viewName` (System.String) — If not null or empty, the name is set.
- `updateConstructionPlane` (System.Boolean) — If true, the construction plane is set to the viewport plane.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetProjection.htm)

#### `public bool SetToPlanView(Point3d planeOrigin, Vector3d planeXaxis, Vector3d planeYaxis, bool setConstructionPlane)`



**Parameters:**
- `planeOrigin` (Rhino.Geometry.Point3d) — [Missing <param name="planeOrigin"/> documentation for "M:Rhino.Display.RhinoViewport.SetToPlanView(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,Rhino.Geometry.Vector3d,System.Boolean)"]
- `planeXaxis` (Rhino.Geometry.Vector3d) — [Missing <param name="planeXaxis"/> documentation for "M:Rhino.Display.RhinoViewport.SetToPlanView(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,Rhino.Geometry.Vector3d,System.Boolean)"]
- `planeYaxis` (Rhino.Geometry.Vector3d) — [Missing <param name="planeYaxis"/> documentation for "M:Rhino.Display.RhinoViewport.SetToPlanView(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,Rhino.Geometry.Vector3d,System.Boolean)"]
- `setConstructionPlane` (System.Boolean) — [Missing <param name="setConstructionPlane"/> documentation for "M:Rhino.Display.RhinoViewport.SetToPlanView(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,Rhino.Geometry.Vector3d,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.SetToPlanView(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d,Rhino.Geometry.Vector3d,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetToPlanView.htm)

#### `public bool SetTraceImage(string bitmapFileName, Plane plane, double width, double height, bool grayscale, bool filtered)`

Set trace image (background bitmap) for this viewport.

**Parameters:**
- `bitmapFileName` (System.String) — The bitmap file name.
- `plane` (Rhino.Geometry.Plane) — A picture plane.
- `width` (System.Double) — The picture width.
- `height` (System.Double) — The picture height.
- `grayscale` (System.Boolean) — true if the picture should be in grayscale.
- `filtered` (System.Boolean) — true if image should be filtered (bilinear) before displayed.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetTraceImage.htm)

#### `public bool SetUserString(string key, string value)`

Attach a user string (key,value combination) to this geometry.

**Parameters:**
- `key` (System.String) — id used to retrieve this string.
- `value` (System.String) — string associated with key. If null, the key will be removed

**Returns:** `Boolean` — true on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetUserString.htm)

#### `public bool SetViewProjection(ViewportInfo projection, bool updateTargetLocation)`

Sets the viewport camera projection.

**Parameters:**
- `projection` (Rhino.DocObjects.ViewportInfo) — The "standard" projection type.
- `updateTargetLocation` (System.Boolean) — if true, the target location is changed so that the vector from the camera location to the target is parallel to the camera direction vector. If false, the target location is not changed.

**Returns:** `Boolean` — true on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetViewProjection.htm)

#### `public bool SetWallpaper(string imageFilename, bool grayscale)`



**Parameters:**
- `imageFilename` (System.String) — [Missing <param name="imageFilename"/> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean)"]
- `grayscale` (System.Boolean) — [Missing <param name="grayscale"/> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetWallpaper.htm)

#### `public bool SetWallpaper(string imageFilename, bool grayscale, bool visible)`



**Parameters:**
- `imageFilename` (System.String) — [Missing <param name="imageFilename"/> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean,System.Boolean)"]
- `grayscale` (System.Boolean) — [Missing <param name="grayscale"/> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean,System.Boolean)"]
- `visible` (System.Boolean) — [Missing <param name="visible"/> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Display.RhinoViewport.SetWallpaper(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_SetWallpaper_1.htm)

#### `public Point2d WorldToClient(Point3d worldPoint)`

Convert a point from world coordinates in the viewport to a 2d screen point in the local coordinates of the viewport (X/Y of point is relative to top left corner of viewport on screen)

**Parameters:**
- `worldPoint` (Rhino.Geometry.Point3d) — The 3D point in world coordinates.

**Returns:** `Point2d` — The 2D point on the screen.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_WorldToClient.htm)

#### `public bool ZoomBoundingBox(BoundingBox box)`

Zooms the viewport to the given bounding box.

**Parameters:**
- `box` (Rhino.Geometry.BoundingBox) — The bounding box to zoom.

**Returns:** `Boolean` — true if operation succeeded; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ZoomBoundingBox.htm)

#### `public bool ZoomExtents()`

Dollies the camera location and so that the view frustum contains all of the selected document objects that can be seen in view. If the projection is perspective, the camera angle is not changed.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ZoomExtents.htm)

#### `public bool ZoomExtentsSelected()`

Dollies the camera location and so that the view frustum contains all of the selected document objects that can be seen in view. If the projection is perspective, the camera angle is not changed.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_RhinoViewport_ZoomExtentsSelected.htm)

### Properties
- `Bounds` (Rectangle, get) — Gets the size and location of the viewport, in pixels, relative to the parent view.
- `Camera35mmLensLength` (Double, get/set) — 
- `CameraDirection` (Vector3d, get) — 
- `CameraLocation` (Point3d, get) — 
- `CameraTarget` (Point3d, get) — Viewport target point.
- `CameraUp` (Vector3d, get/set) — 
- `CameraX` (Vector3d, get) — Gets the "unit to the right" vector.
- `CameraY` (Vector3d, get) — Gets the "unit up" vector.
- `CameraZ` (Vector3d, get) — Gets the unit vector in CameraDirection.
- `ChangeCounter` (UInt32, get) — The value of change counter is incremented every time the view projection or construction plane changes. The user can the mouse and nestable view manipulation commands to change a view at any time. The value of change counter can be used to detect these changes in code that is sensitive to the view projection.
- `ConstructionAxesVisible` (Boolean, get/set) — 
- `ConstructionGridVisible` (Boolean, get/set) — 
- `DisplayMode` (DisplayModeDescription, get/set) — 
- `FrustumAspect` (Double, get) — Gets the width/height ratio of the frustum.
- `Id` (Guid, get) — Unique id for this viewport.
- `IsParallelProjection` (Boolean, get) — 
- `IsPerspectiveProjection` (Boolean, get) — 
- `IsPlanView` (Boolean, get) — true if construction plane z axis is parallel to camera direction.
- `IsTwoPointPerspectiveProjection` (Boolean, get) — 
- `IsValidCamera` (Boolean, get) — 
- `IsValidFrustum` (Boolean, get) — 
- `Name` (String, get/set) — Name associated with this viewport.
- `ParentView` (RhinoView, get) — Gets the parent view, if there is one Every RhinoView has an associated RhinoViewport that does all the 3d display work. Those associated viewports return the RhinoView as their parent view. However, RhinoViewports are used in other image creating contexts that do not have a parent RhinoView. If you call ParentView, you MUST check for NULL return values.
- `ScreenPortAspect` (Double, get) — screen port's width/height.
- `Size` (Size, get/set) — Get or set the height and width of the viewport (in pixels)
- `UserStringCount` (Int32, get) — 
- `ViewportType` (ViewportType, get) — 
- `WallpaperFilename` (String, get) — 
- `WallpaperGrayscale` (Boolean, get) — 
- `WallpaperVisible` (Boolean, get) — 
- `WorldAxesVisible` (Boolean, get/set) — 

## ShaderLanguage (enum)

Graphics Shader Language https://en.wikipedia.org/wiki/OpenGL_Shading_Language#Versions

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ShaderLanguage.htm)

### Values
- `GLSL_ES100` = `1` — Used for OpenGL ES 2.0 and WebGL 1.0
- `GLSL_ES300` = `2` — Used for OpenGL ES 3.0 and WebGL 2.0
- `GLSL_ES310` = `3`
- `GLSL_120` = `4` — Used for OpenGL 2.1
- `GLSL_130` = `5` — Used for OpenGL 3.0
- `GLSL_140` = `6` — Used for OpenGL 3.1
- `GLSL_150` = `7` — Used for OpenGL 3.2
- `GLSL_330` = `8` — Used for OpenGL 3.3
- `GLSL_400` = `9` — Used for OpenGL 4.0
- `GLSL_410` = `10` — Used for OpenGL 4.1
- `GLSL_420` = `11` — Used for OpenGL 4.2
- `GLSL_430` = `12` — Used for OpenGL 4.3
- `GLSL_440` = `13` — Used for OpenGL 4.4
- `Metal` = `14` — Metal for Apple

## StereoContext (enum)

[Missing <summary> documentation for "T:Rhino.Display.StereoContext"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_StereoContext.htm)

### Values
- `None` = `0`
- `LeftEye` = `1`
- `RightEye` = `2`
- `BothEyes` = `3`

## Text3d (class)

3D aligned text with font settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_Text3d.htm)

### Constructors
- `public Text3d(string text)` — Constructs a new instance of Text3d.
- `public Text3d(string text, Plane plane, double height)` — Constructs a new instance of Text3d.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_Text3d_Dispose.htm)

### Properties
- `Bold` (Boolean, get/set) — Gets or sets whether this Text3d object will be drawn in Bold.
- `BoundingBox` (BoundingBox, get) — Gets the bounding box for this Text3d object.
- `FontFace` (String, get/set) — Gets or sets the FontFace name.
- `Height` (Double, get/set) — Gets or sets the height (in units) of this Text3d object. The height should be a positive number larger than zero.
- `HorizontalAlignment` (TextHorizontalAlignment, get/set) — Horizontal alignment that this Text3d is drawn with
- `Italic` (Boolean, get/set) — Gets or sets whether this Text3d object will be drawn in Italics.
- `Text` (String, get/set) — Gets or sets the text string for this Text3d object.
- `TextPlane` (Plane, get/set) — Gets or sets the 3D aligned plane for this Text3d object.
- `VerticalAlignment` (TextVerticalAlignment, get/set) — Vertical alignment that this Text3d is drawn with

## ViewCapture (class)

[Missing <summary> documentation for "T:Rhino.Display.ViewCapture"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewCapture.htm)

### Constructors
- `public ViewCapture()` — Initializes a new instance of the ViewCapture class

### Methods
#### `public Bitmap CaptureToBitmap(RhinoView sourceView)`



**Parameters:**
- `sourceView` (Rhino.Display.RhinoView) — [Missing <param name="sourceView"/> documentation for "M:Rhino.Display.ViewCapture.CaptureToBitmap(Rhino.Display.RhinoView)"]

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.Display.ViewCapture.CaptureToBitmap(Rhino.Display.RhinoView)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCapture_CaptureToBitmap.htm)

#### `public static Bitmap CaptureToBitmap(ViewCaptureSettings settings)`



**Parameters:**
- `settings` (Rhino.Display.ViewCaptureSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.Display.ViewCapture.CaptureToBitmap(Rhino.Display.ViewCaptureSettings)"]

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.Display.ViewCapture.CaptureToBitmap(Rhino.Display.ViewCaptureSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCapture_CaptureToBitmap_1.htm)

#### `public static XmlDocument CaptureToSvg(ViewCaptureSettings settings)`



**Parameters:**
- `settings` (Rhino.Display.ViewCaptureSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.Display.ViewCapture.CaptureToSvg(Rhino.Display.ViewCaptureSettings)"]

**Returns:** `XmlDocument` — [Missing <returns> documentation for "M:Rhino.Display.ViewCapture.CaptureToSvg(Rhino.Display.ViewCaptureSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCapture_CaptureToSvg.htm)

### Properties
- `DrawAxes` (Boolean, get/set) — 
- `DrawGrid` (Boolean, get/set) — 
- `DrawGridAxes` (Boolean, get/set) — 
- `Height` (Int32, get/set) — Height of capture in Pixels
- `Preview` (Boolean, get/set) — 
- `RealtimeRenderPasses` (Int32, get/set) — 
- `ScaleScreenItems` (Boolean, get/set) — 
- `TransparentBackground` (Boolean, get/set) — 
- `Width` (Int32, get/set) — Width of capture in Pixels

## ViewCaptureSettings (class)

Holds the information required to generate high resolution output of a RhinoViewport. This is used for generating paper prints or image files

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewCaptureSettings.htm)

### Constructors
- `public ViewCaptureSettings()` — Initializes a new instance of the ViewCaptureSettings class
- `public ViewCaptureSettings(RhinoPageView sourcePageView, double dpi)` — Initializes a new instance of the ViewCaptureSettings class
- `public ViewCaptureSettings(RhinoView sourceView, Size mediaSize, double dpi)` — Initializes a new instance of the ViewCaptureSettings class

### Methods
#### `public ViewCaptureSettings CreatePreviewSettings(Size size)`

Create a ViewCaptureSettings based on this instance, but scaled to fit in a different sized area. Scaling is also performed on dpi. This is primarily used to for capturing images that are shown as print previews

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Display.ViewCaptureSettings.CreatePreviewSettings(System.Drawing.Size)"]

**Returns:** `ViewCaptureSettings` — new ViewCaptureSettings instance on success. Null on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_CreatePreviewSettings.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_Finalize.htm)

#### `public bool GetMargins(UnitSystem lengthUnits, out double left, out double top, out double right, out double bottom)`

Get distances from the edge of the paper (MediaSize) to the CropRectangle in a defined unit system

**Parameters:**
- `lengthUnits` (Rhino.UnitSystem) — Units to get distances in
- `left` (System.Double) — Distance from left edge of paper to left edge of CropRectangle
- `top` (System.Double) — Distance from top edge of paper to top edge of CropRectangle
- `right` (System.Double) — Distance from right edge of paper to right edge of CropRectangle
- `bottom` (System.Double) — Distance from bottom edge of paper to bottom edge of CropRectangle

**Returns:** `Boolean` — True if successful. False if unsuccessful (this could happen if there is no set device_dpi)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_GetMargins.htm)

#### `public double GetModelScale(UnitSystem pageUnits, UnitSystem modelUnits)`

Returns the model scale factor.

**Parameters:**
- `pageUnits` (Rhino.UnitSystem) — The current page units.
- `modelUnits` (Rhino.UnitSystem) — The current model units.

**Returns:** `Double` — The model scale factor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_GetModelScale.htm)

#### `public void GetOffset(UnitSystem lengthUnits, out bool fromMargin, out double x, out double y)`



**Parameters:**
- `lengthUnits` (Rhino.UnitSystem) — [Missing <param name="lengthUnits"/> documentation for "M:Rhino.Display.ViewCaptureSettings.GetOffset(Rhino.UnitSystem,System.Boolean@,System.Double@,System.Double@)"]
- `fromMargin` (System.Boolean) — [Missing <param name="fromMargin"/> documentation for "M:Rhino.Display.ViewCaptureSettings.GetOffset(Rhino.UnitSystem,System.Boolean@,System.Double@,System.Double@)"]
- `x` (System.Double) — [Missing <param name="x"/> documentation for "M:Rhino.Display.ViewCaptureSettings.GetOffset(Rhino.UnitSystem,System.Boolean@,System.Double@,System.Double@)"]
- `y` (System.Double) — [Missing <param name="y"/> documentation for "M:Rhino.Display.ViewCaptureSettings.GetOffset(Rhino.UnitSystem,System.Boolean@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_GetOffset.htm)

#### `public bool MatchViewportAspectRatio()`

Adjust crop rectangle to match the aspect ratio of the original viewport that these settings reference

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_MatchViewportAspectRatio.htm)

#### `public void MaximizePrintableArea()`

Minimize cropping so the full drawable area is used

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_MaximizePrintableArea.htm)

#### `public void SetLayout(Size mediaSize, Rectangle cropRectangle)`



**Parameters:**
- `mediaSize` (System.Drawing.Size) — [Missing <param name="mediaSize"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetLayout(System.Drawing.Size,System.Drawing.Rectangle)"]
- `cropRectangle` (System.Drawing.Rectangle) — [Missing <param name="cropRectangle"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetLayout(System.Drawing.Size,System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetLayout.htm)

#### `public bool SetMargins(UnitSystem lengthUnits, double left, double top, double right, double bottom)`

Set distances from the edge of the paper (MediaSize) to the CropRectange in a defined unit system

**Parameters:**
- `lengthUnits` (Rhino.UnitSystem) — Units that left, top, right, and bottom are defined in
- `left` (System.Double) — Distance from left edge of paper to left edge of CropRectangle
- `top` (System.Double) — Distance from top edge of paper to top edge of CropRectangle
- `right` (System.Double) — Distance from right edge of paper to right edge of CropRectangle
- `bottom` (System.Double) — Distance from bottom edge of paper to bottom edge of CropRectangle

**Returns:** `Boolean` — True if successful. False if unsuccessful (this could happen if there is no set device_dpi)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetMargins.htm)

#### `public void SetModelScaleToFit(bool promptOnChange)`

Scales the model to fit.

**Parameters:**
- `promptOnChange` (System.Boolean) — Prompt the user if the model scale will change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetModelScaleToFit.htm)

#### `public void SetModelScaleToValue(double scale)`

Sets the model scale to a value.

**Parameters:**
- `scale` (System.Double) — The scale value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetModelScaleToValue.htm)

#### `public void SetOffset(UnitSystem lengthUnits, bool fromMargin, double x, double y)`



**Parameters:**
- `lengthUnits` (Rhino.UnitSystem) — [Missing <param name="lengthUnits"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetOffset(Rhino.UnitSystem,System.Boolean,System.Double,System.Double)"]
- `fromMargin` (System.Boolean) — [Missing <param name="fromMargin"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetOffset(Rhino.UnitSystem,System.Boolean,System.Double,System.Double)"]
- `x` (System.Double) — [Missing <param name="x"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetOffset(Rhino.UnitSystem,System.Boolean,System.Double,System.Double)"]
- `y` (System.Double) — [Missing <param name="y"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetOffset(Rhino.UnitSystem,System.Boolean,System.Double,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetOffset.htm)

#### `public void SetViewport(RhinoViewport viewport)`



**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.Display.ViewCaptureSettings.SetViewport(Rhino.Display.RhinoViewport)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetViewport.htm)

#### `public void SetWindowRect(Point2d screenPoint1, Point2d screenPoint2)`

Set the print area to a window selection based on two points in screen coordinates

**Parameters:**
- `screenPoint1` (Rhino.Geometry.Point2d) — first point; it doesn't matter what corner of the rectangle this point represents
- `screenPoint2` (Rhino.Geometry.Point2d) — point representing opposite corner of rectangle from screenPoint1

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetWindowRect.htm)

#### `public void SetWindowRect(Point3d worldPoint1, Point3d worldPoint2)`

Set the print area to a window selection based on two points in world coordinates

**Parameters:**
- `worldPoint1` (Rhino.Geometry.Point3d) — First point in world coordinates. This point is projected to screen coordinates
- `worldPoint2` (Rhino.Geometry.Point3d) — Second point in world coordinates. This point is projected to screen coordinates

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ViewCaptureSettings_SetWindowRect_1.htm)

### Properties
- `ArrowheadSizeMillimeters` (Double, get/set) — arrowhead size in millimeters
- `CropRectangle` (Rectangle, get) — Actual area of output rectangle that view capture is sent to.
- `DefaultPrintWidthMillimeters` (Double, get/set) — Line thickness used to print objects with no defined thickness (in mm)
- `Document` (RhinoDoc, get/set) — 
- `DrawAxis` (Boolean, get/set) — 
- `DrawBackground` (Boolean, get/set) — 
- `DrawBackgroundBitmap` (Boolean, get/set) — 
- `DrawClippingPlanes` (Boolean, get/set) — 
- `DrawGrid` (Boolean, get/set) — 
- `DrawLights` (Boolean, get/set) — 
- `DrawLockedObjects` (Boolean, get/set) — 
- `DrawMargins` (Boolean, get/set) — 
- `DrawSelectedObjectsOnly` (Boolean, get/set) — 
- `DrawWallpaper` (Boolean, get/set) — 
- `FooterText` (String, get/set) — Text drawn at the bottom of the output
- `HeaderText` (String, get/set) — Text drawn at the top of the output
- `IsScaleToFit` (Boolean, get) — Returns true if the model has been scaled to fit.
- `IsValid` (Boolean, get) — 
- `MatchLinetypePatternDefinition` (Boolean, get/set) — Default is true. Linetype scales are normally generated right before printing/view capture in order to get linetypes to print to the same lengths as defined. If false, the linetypes are not scaled and the current pattern lengths as seen on the screen as used.
- `MediaSize` (Size, get) — Total size of the image or page in dots
- `ModelScaleType` (Int32, get/set) — 
- `OffsetAnchor` (ViewCaptureSettings.AnchorLocation, get/set) — 
- `OutputColor` (ViewCaptureSettings.ColorMode, get/set) — 
- `PointSizeMillimeters` (Double, get/set) — size of point objects in millimeters if scale <= 0 the size is minimized so points are always drawn as small as possible
- `RasterMode` (Boolean, get/set) — 
- `Resolution` (Double, get/set) — Capture "density" in dots per inch
- `UsePrintWidths` (Boolean, get/set) — 
- `ViewArea` (ViewCaptureSettings.ViewAreaMapping, get/set) — How the RhinoViewport is mapped to the output rectangle
- `WireThicknessScale` (Double, get/set) — scaling factor to apply to object print widths (typically 1.0). This is helpful when printing something at 1/2 scale and having all of the curves print 1/2 as thick

## ViewCaptureSettings.AnchorLocation (enum)

[Missing <summary> documentation for "T:Rhino.Display.ViewCaptureSettings.AnchorLocation"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewCaptureSettings_AnchorLocation.htm)

### Values
- `LowerLeft` = `0`
- `LowerRight` = `2`
- `UpperLeft` = `1`
- `UpperRight` = `3`
- `Center` = `4`

## ViewCaptureSettings.ColorMode (enum)

[Missing <summary> documentation for "T:Rhino.Display.ViewCaptureSettings.ColorMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewCaptureSettings_ColorMode.htm)

### Values
- `DisplayColor` = `0`
- `PrintColor` = `1`
- `BlackAndWhite` = `2`

## ViewCaptureSettings.ViewAreaMapping (enum)

Defines how a viewport is mapped to output.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewCaptureSettings_ViewAreaMapping.htm)

### Values
- `View` = `0` — Best fit rectangle of what is displayed in a viewport
- `Extents` = `1` — Zoom projection to all visible geometry in a viewport
- `Window` = `2` — Use window rectangle defined by user

## ViewEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Display.ViewEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewEventArgs.htm)

### Properties
- `View` (RhinoView, get) — 

## ViewportType (enum)

[Missing <summary> documentation for "T:Rhino.Display.ViewportType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ViewportType.htm)

### Values
- `StandardModelingViewport` = `0`
- `PageViewMainViewport` = `1`
- `DetailViewport` = `2`

## VisualAnalysisMode (class)

Represents a base class for visual analysis modes. This class is abstract.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_VisualAnalysisMode.htm)

### Constructors
- `protected VisualAnalysisMode()` — Initializes a new instance of the VisualAnalysisMode class

### Methods
#### `public static bool AdjustAnalysisMeshes(RhinoDoc doc, Guid analysisModeId)`

Interactively adjusts surface analysis meshes of objects using a Rhino built-in analysis mode.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document.
- `analysisModeId` (System.Guid) — The id of the analysis mode.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_AdjustAnalysisMeshes.htm)

#### `protected virtual void DrawBrepObject(BrepObject brep, DisplayPipeline pipeline)`

Draws one brep. Override this method to add your custom behavior. The default implementation does nothing.

**Parameters:**
- `brep` (Rhino.DocObjects.BrepObject) — A brep object.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawBrepObject.htm)

#### `protected virtual void DrawCurveObject(CurveObject curve, DisplayPipeline pipeline)`

If Style==Wireframe, then the default decomposes the curve object into nurbs curve segments and calls the virtual DrawNurbsCurve for each segment.

**Parameters:**
- `curve` (Rhino.DocObjects.CurveObject) — A document curve object.
- `pipeline` (Rhino.Display.DisplayPipeline) — The drawing pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawCurveObject.htm)

#### `protected virtual void DrawMesh(RhinoObject obj, Mesh mesh, DisplayPipeline pipeline)`

Draws a mesh. The default implementation does nothing.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — A Rhino object corresponding to the surface.
- `mesh` (Rhino.Geometry.Mesh) — The mesh geometry.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawMesh.htm)

#### `protected virtual void DrawMeshObject(MeshObject mesh, DisplayPipeline pipeline)`

Draws one mesh. Override this method to add your custom behavior. The default implementation does nothing.

**Parameters:**
- `mesh` (Rhino.DocObjects.MeshObject) — A mesh object.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawMeshObject.htm)

#### `protected virtual void DrawNurbsCurve(RhinoObject obj, NurbsCurve curve, DisplayPipeline pipeline)`

Draws a NURBS curve. This is a good function to override for analysis modes like curvature hair display. The default implementation does nothing.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — A Rhino object corresponding to the curve.
- `curve` (Rhino.Geometry.NurbsCurve) — The curve geometry.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawNurbsCurve.htm)

#### `protected virtual void DrawNurbsSurface(RhinoObject obj, NurbsSurface surface, DisplayPipeline pipeline)`

Draws a NURBS surface. This is a good function to override to display object-related meshes. The default implementation does nothing.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — A Rhino object corresponding to the surface.
- `surface` (Rhino.Geometry.NurbsSurface) — The surface geometry.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawNurbsSurface.htm)

#### `protected virtual void DrawPointCloudObject(PointCloudObject pointCloud, DisplayPipeline pipeline)`

Draws one point cloud. Override this method to add your custom behavior. The default implementation does nothing.

**Parameters:**
- `pointCloud` (Rhino.DocObjects.PointCloudObject) — A point cloud object.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawPointCloudObject.htm)

#### `protected virtual void DrawPointObject(PointObject point, DisplayPipeline pipeline)`

Draws one point. Override this method to add your custom behavior. The default implementation does nothing.

**Parameters:**
- `point` (Rhino.DocObjects.PointObject) — A point object.
- `pipeline` (Rhino.Display.DisplayPipeline) — The current display pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_DrawPointObject.htm)

#### `public virtual void EnableUserInterface(bool on)`

Turns the analysis mode's user interface on and off. For Rhino's built in modes this opens or closes the modeless dialog that controls the analysis mode's display settings.

**Parameters:**
- `on` (System.Boolean) — true if the interface should be shown; false if it should be concealed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_EnableUserInterface.htm)

#### `public static VisualAnalysisMode Find(Guid id)`

Finds a visual analysis mode by guid.

**Parameters:**
- `id` (System.Guid) — The globally unique identifier to search for.

**Returns:** `VisualAnalysisMode` — The found visual analysis mode, or null if it was not found, or on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_Find.htm)

#### `public static VisualAnalysisMode Find(Type t)`

Finds a visual analysis mode by type.

**Parameters:**
- `t` (System.Type) — A visual analysis mode type.

**Returns:** `VisualAnalysisMode` — A visual analysis mode on success, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_Find_1.htm)

#### `public virtual bool ObjectSupportsAnalysisMode(RhinoObject obj)`

Gets a value indicating if this visual analysis mode can be used on a given Rhino object.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — The object to be tested.

**Returns:** `Boolean` — true if this mode can indeed be used on the object; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_ObjectSupportsAnalysisMode.htm)

#### `public static VisualAnalysisMode Register(Type customAnalysisModeType)`

Registers a custom visual analysis mode for use in Rhino. It is OK to call register multiple times for a single custom analysis mode type, since subsequent register calls will notice that the type has already been registered.

**Parameters:**
- `customAnalysisModeType` (System.Type) — Must be a type that is a subclass of VisualAnalysisMode.

**Returns:** `VisualAnalysisMode` — An instance of registered analysis mode on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_Register.htm)

#### `protected virtual void SetUpDisplayAttributes(RhinoObject obj, DisplayPipelineAttributes attributes)`

If an analysis mode needs to modify display attributes, this is the place to do it. In particular, Style==Texture, then this function must be overridden.

**Remarks:** Shaded analysis modes that use texture mapping, like zebra and emap, override this function set the texture, diffuse_color, and EnableLighting parameter.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — The object for which to set up attributes.
- `attributes` (Rhino.Display.DisplayPipelineAttributes) — The linked attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_SetUpDisplayAttributes.htm)

#### `protected virtual void UpdateVertexColors(RhinoObject obj, Mesh[] meshes)`

If Style==falseColor, then this virtual function must be overridden. Rhino calls this function when it is time for to set the false colors on the analysis mesh vertices. For breps, there is one mesh per face. For mesh objects there is a single mesh.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — The object for which to update vertex colors.
- `meshes` (Rhino.Geometry.Mesh[]) — An array of meshes that should be updated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_VisualAnalysisMode_UpdateVertexColors.htm)

### Properties
- `Id` (Guid, get) — Gets the visual analysis mode GUID. The Guid is specified with the GuidAttribute applied to the class.
- `Name` (String, get) — Gets the name of the analysis mode. It is used by the _What command and the object properties details window to describe the object.
- `RhinoCurvatureColorAnalyisModeId` (Guid, get) — Id for Rhino's built-in curvature color analysis mode. Surface curvature is shown using false color mapping.
- `RhinoCurvatureGraphAnalysisModeId` (Guid, get) — Id for Rhino's built-in curvature graphs analysis mode. Curvature hair is shown on curves and surfaces.
- `RhinoDraftAngleAnalysisModeId` (Guid, get) — Id for Rhino's built-in draft angle analysis mode. Draft angle is displayed using false colors.
- `RhinoEdgeAnalysisModeId` (Guid, get) — Id for Rhino's built-in edge analysis mode. Brep and mesh edges are shown in a selected color.
- `RhinoEdgeContinuityAlalysisModeId` (Guid, get) — Id for Rhino's built-in edge continuity analysis mode.
- `RhinoEmapAnalysisModeId` (Guid, get) — Id for Rhino's built-in emap analysis mode. An environment map is shown on surfaces and meshes.
- `RhinoThicknessAnalysisModeId` (Guid, get) — Id for Rhino's built-in thickness analysis mode.
- `RhinoZebraStripeAnalysisModeId` (Guid, get) — Id for Rhino's built-in zebra stripe analysis mode. Zebra stripes are shown on surfaces and meshes.
- `ShowIsoCurves` (Boolean, get) — Gets true if this visual analysis mode will show isocuves on shaded surface objects. Often a mode's user interface will provide a way to change this setting. The default is false.
- `Style` (VisualAnalysisMode.AnalysisStyle, get) — Gets the visual analysis mode style.

## VisualAnalysisMode.AnalysisStyle (enum)

Contains enumerated values for analysis styles, such as wireframe, texture or false colors..

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_VisualAnalysisMode_AnalysisStyle.htm)

### Values
- `Wireframe` = `1` — The analysis is showing with wires.
- `Texture` = `2` — The analysis is showing with textures.
- `FalseColor` = `4` — The analysis is showing with false colors.

## VisualAnalysisMode.EdgeContinuityMode (enum)

Continuity mode to report

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_VisualAnalysisMode_EdgeContinuityMode.htm)

### Values
- `Distance` = `0` — G0 Continuity
- `Tangency` = `1` — G1 Continuity
- `Curvature` = `2` — G2 Continuity

## ZBiasMode (enum)

Biasing applied to geometry to attempt to get coplanar items to draw on top of or below other geometry

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ZBiasMode.htm)

### Values
- `Neutral` = `0`
- `TowardsCamera` = `1`
- `AwayFromCamera` = `2`

## ZBufferCapture (class)

Provides functionality for getting the z-buffer values from a viewport and a given display mode

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Display_ZBufferCapture.htm)

### Constructors
- `public ZBufferCapture(RhinoViewport viewport)` — Initializes a new instance of the ZBufferCapture class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_Finalize.htm)

#### `public Bitmap GrayscaleDib()`



**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.Display.ZBufferCapture.GrayscaleDib"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_GrayscaleDib.htm)

#### `public int HitCount()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Display.ZBufferCapture.HitCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_HitCount.htm)

#### `public float MaxZ()`



**Returns:** `Single` — [Missing <returns> documentation for "M:Rhino.Display.ZBufferCapture.MaxZ"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_MaxZ.htm)

#### `public float MinZ()`



**Returns:** `Single` — [Missing <returns> documentation for "M:Rhino.Display.ZBufferCapture.MinZ"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_MinZ.htm)

#### `public void SetDisplayMode(Guid modeId)`



**Parameters:**
- `modeId` (System.Guid) — [Missing <param name="modeId"/> documentation for "M:Rhino.Display.ZBufferCapture.SetDisplayMode(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_SetDisplayMode.htm)

#### `public void ShowAnnotations(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowAnnotations(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowAnnotations.htm)

#### `public void ShowCurves(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowCurves(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowCurves.htm)

#### `public void ShowIsocurves(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowIsocurves(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowIsocurves.htm)

#### `public void ShowLights(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowLights(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowLights.htm)

#### `public void ShowMeshWires(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowMeshWires(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowMeshWires.htm)

#### `public void ShowPoints(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowPoints(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowPoints.htm)

#### `public void ShowText(bool on)`



**Parameters:**
- `on` (System.Boolean) — [Missing <param name="on"/> documentation for "M:Rhino.Display.ZBufferCapture.ShowText(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ShowText.htm)

#### `public Point3d WorldPointAt(int x, int y)`



**Parameters:**
- `x` (System.Int32) — [Missing <param name="x"/> documentation for "M:Rhino.Display.ZBufferCapture.WorldPointAt(System.Int32,System.Int32)"]
- `y` (System.Int32) — [Missing <param name="y"/> documentation for "M:Rhino.Display.ZBufferCapture.WorldPointAt(System.Int32,System.Int32)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.Display.ZBufferCapture.WorldPointAt(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_WorldPointAt.htm)

#### `public float ZValueAt(int x, int y)`



**Parameters:**
- `x` (System.Int32) — [Missing <param name="x"/> documentation for "M:Rhino.Display.ZBufferCapture.ZValueAt(System.Int32,System.Int32)"]
- `y` (System.Int32) — [Missing <param name="y"/> documentation for "M:Rhino.Display.ZBufferCapture.ZValueAt(System.Int32,System.Int32)"]

**Returns:** `Single` — [Missing <returns> documentation for "M:Rhino.Display.ZBufferCapture.ZValueAt(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Display_ZBufferCapture_ZValueAt.htm)

