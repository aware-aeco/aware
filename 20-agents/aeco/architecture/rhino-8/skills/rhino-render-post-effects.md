---
name: rhino-common-rhino-render-post-effects
description: API reference for namespace Rhino.Render.PostEffects from RhinoCommon.dll
---

# Rhino.Render.PostEffects

- **CustomPostEffectAttribute**
- **IPostEffects**
- **PostEffect**
- **PostEffectChannel**
- **PostEffectCollection**
  - Represents the collection of post effects in render settings.
- **PostEffectData**
  - This is a wrapper around the data ('on', 'shown', 'state' parameters, etc.) of a post effect.
- **PostEffectExecuteContexts**
- **PostEffectExecuteWhileRenderingOptions**
- **PostEffectExecutionControl**
- **PostEffectHistograms**
- **PostEffectJob**
- **PostEffectJobChannels**
- **PostEffectPipeline**
  - This object provides a way for post effects to access the frame buffer channels from a rendering and create             new channels containing post-processed information which can be passed to the next post effect in the chain.             Consider a simple post effect that just modifies the red component of a rendering.It will call GetChannel()             to get the red channel as its input, and it will call NewChannel() to get a new red channel for its output.             It will then read the input channel, do calculations and write to the output channel.When finished, it will             call Commit() passing the new channel.Because both channels have the same identifier, this will replace the             old channel with the new one so that subsequent post effects in the chain will use the new channel instead of the             original.Note that this will only replace the channel used by the pipeline.The original channel will still             exist in the frame buffer.This system allows any post effect to access any number of channels for reading and             create any number of new channels which may or may not replace existing channels depending on the channel             id.The final stage (convert to 8-bit) operates on the channels left in the pipeline by the post effect chain to             produce the final 32-bit RGBA image in a dib.                          It is also possible for a post effect to create and use any number of 'scratch' channels.If a post effect needs a             temporary pixel buffer for some intermediate results, it can call NewChannel() with a custom (random) id.             Once it is finished with this scratch channel, it can call Discard() on it.
- **PostEffectState**
- **PostEffectStyles**
- **PostEffectThreadEngine**
- **PostEffectType**
- **PostEffectUI**
  - PostEffectUI class provides a way for post effect plug-ins to add ui sections.
- **PostEffectUuids**
