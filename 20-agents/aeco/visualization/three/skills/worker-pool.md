---
name: three-worker-pool
description: WorkerPool declarations from three
---

# WorkerPool

## Methods

- `_initWorker(workerId: number)`
- `workerCreator()`
- `_getIdleWorker()`
- `_onMessage(workerId: number, msg: any)`
- `setWorkerCreator(workerCreator: () => Worker)`
- `setWorkerLimit(pool: number)`
- `postMessage(msg: any, transfer?: Array<Transferable>)`
- `dispose()`
