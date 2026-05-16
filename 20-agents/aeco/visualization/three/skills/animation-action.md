---
name: three-animation-action
description: AnimationAction declarations from three
---

# AnimationAction

## Methods

- `play()`
- `stop()`
- `reset()`
- `isRunning()`
- `isScheduled()`
- `startAt(time: number)`
- `setLoop(mode: AnimationActionLoopStyles, repetitions: number)`
- `setEffectiveWeight(weight: number)`
- `getEffectiveWeight()`
- `fadeIn(duration: number)`
- `fadeOut(duration: number)`
- `crossFadeFrom(fadeOutAction: AnimationAction, duration: number, warp?: boolean)`
- `crossFadeTo(fadeInAction: AnimationAction, duration: number, warp?: boolean)`
- `stopFading()`
- `setEffectiveTimeScale(timeScale: number)`
- `getEffectiveTimeScale()`
- `setDuration(duration: number)`
- `syncWith(action: AnimationAction)`
- `halt(duration: number)`
- `warp(startTimeScale: number, endTimeScale: number, duration: number)`
- `stopWarping()`
- `getMixer()`
- `getClip()`
- `getRoot()`
- `_scheduleFading(duration: number, weightNow: number, weightThen: number)`
