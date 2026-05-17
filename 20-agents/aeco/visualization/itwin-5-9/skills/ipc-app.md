---
name: core-frontend-ipc-app
description: IpcApp declarations from core-frontend
---

# IpcApp

## Methods

- `addListener(channel: string, handler: IpcListener)`
- `removeListener(channel: string, listener: IpcListener)`
- `invoke(channel: string, ...args: any[])`
- `send(channel: string, ...data: any[])`
- `callIpcChannel(channelName: string, methodName: string, ...args: any[])`
- `makeIpcProxy(channelName: C)`
- `makeIpcFunctionProxy(channelName: string, functionName: string)`
- `startup(ipc: IpcSocketFrontend, opts?: IpcAppOptions)`
- `shutdown()`
- `addListener(channel: string, handler: IpcListener)`
- `removeListener(channel: string, listener: IpcListener)`
- `invoke(channel: string, ...args: any[])`
- `send(channel: string, ...data: any[])`
- `callIpcChannel(channelName: string, methodName: string, ...args: any[])`
- `makeIpcProxy(channelName: C)`
- `makeIpcFunctionProxy(channelName: string, functionName: string)`
- `startup(ipc: IpcSocketFrontend, opts?: IpcAppOptions)`
- `shutdown()`
