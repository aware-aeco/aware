# texture-get-alpha-blend-values

Lifecycle: single

If the TextureCombineMode is Blend, then the blending function             for alpha is determined by                          new alpha = constant                         + a0*(current alpha)                         + a1*(texture alpha)                         + a2*min(current alpha,texture alpha)                         + a3*max(current alpha,texture alpha)
