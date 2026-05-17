---
name: core-frontend-tile-request-channel
description: TileRequestChannel declarations from core-frontend
---

# TileRequestChannel

## Methods

- `resetStatistics()`
- `recordTimeout()`
- `recordFailure()`
- `recordCompletion(tile: Tile, content: TileContent, elapsedMilliseconds: number)`
- `swapPending()`
- `append(request: TileRequest)`
- `process()`
- `cancelAndClearAll()`
- `onNoContent(_request: TileRequest)`
- `onActiveRequestCanceled(_request: TileRequest)`
- `processCancellations()`
- `onIModelClosed(_iModel: IModelConnection)`
- `requestContent(tile: Tile, isCanceled: () => boolean)`
- `dispatch(request: TileRequest)`
- `cancel(request: TileRequest)`
- `dropActiveRequest(request: TileRequest)`
- `resetStatistics()`
- `recordTimeout()`
- `recordFailure()`
- `recordCompletion(tile: Tile, content: TileContent, elapsedMilliseconds: number)`
- `swapPending()`
- `append(request: TileRequest)`
- `process()`
- `cancelAndClearAll()`
- `onNoContent(_request: TileRequest)`
- `onActiveRequestCanceled(_request: TileRequest)`
- `processCancellations()`
- `onIModelClosed(_iModel: IModelConnection)`
- `requestContent(tile: Tile, isCanceled: () => boolean)`
- `dispatch(request: TileRequest)`
- `cancel(request: TileRequest)`
- `dropActiveRequest(request: TileRequest)`
