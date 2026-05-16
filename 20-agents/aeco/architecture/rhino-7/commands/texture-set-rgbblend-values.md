# texture-set-rgbblend-values

Lifecycle: single

If the TextureCombineMode is Blend, then the blending function              for RGB is determined by                            new rgb = colorcolor                      + a0[0]*(current RGB)                      + a1[1]*(texture RGB)                      + a2[2]*min(current RGB,texture RGB)                      + a3[3]*max(current RGB,texture RGB)
